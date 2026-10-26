use super::common::*;
use super::open::Opener;
use super::request::{self, Config, Outcome};
use ckc_kernel::ERequest;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, Shutdown, TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

// One deadline for reading the request (line, headers, body) and a fresh one
// for writing the answer: a slow client cannot hold the single-threaded loop,
// and a slow snapshot or ledger write never cuts off an answer already earned.
const DEADLINE: Duration = Duration::from_secs(10);
// The decision form is a few hundred bytes; a larger body is never read.
const BODY_CAP: usize = 64 * 1024;

struct Timed<'a> {
    stream: &'a TcpStream,
    until: Instant,
}
impl Timed<'_> {
    fn left(&self) -> std::io::Result<Duration> {
        let left = self.until.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "HTTP deadline",
            ));
        }
        Ok(left)
    }
}
impl Read for Timed<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.set_read_timeout(Some(self.left()?))?;
        let mut stream = self.stream;
        stream.read(buf)
    }
}
impl Write for Timed<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.set_write_timeout(Some(self.left()?))?;
        let mut stream = self.stream;
        stream.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        let mut stream = self.stream;
        stream.flush()
    }
}

fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|b| char::from(*b)).collect()
}
fn line(reader: &mut impl BufRead) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.take(65537).read_until(b'\n', &mut bytes)?;
    if bytes.len() > 65536 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "HTTP line too long",
        ));
    }
    Ok(bytes)
}
enum Parsed {
    Closed,
    Request(ERequest),
    Answered(Outcome),
}
fn parse(timed: Timed<'_>, config: &Config) -> std::io::Result<Parsed> {
    let mut reader = BufReader::new(timed);
    let request_line = line(&mut reader)?;
    if request_line.is_empty() {
        return Ok(Parsed::Closed);
    }
    let value = latin1(&request_line);
    let parts = value.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 3 || !matches!(parts[2], "HTTP/1.0" | "HTTP/1.1") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "HTTP request line",
        ));
    }
    let mut fields: Vec<(String, String)> = Vec::new();
    for index in 0..=100 {
        let raw = line(&mut reader)?;
        if raw == b"\r\n" || raw == b"\n" || raw.is_empty() {
            break;
        }
        if index == 100 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "HTTP header count",
            ));
        }
        let value = latin1(&raw);
        if value.starts_with([' ', '\t']) {
            if let Some((_, prev)) = fields.last_mut() {
                prev.push_str(&value);
            }
            continue;
        }
        let Some((name, value)) = value.trim_end_matches(['\r', '\n']).split_once(':') else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "HTTP header",
            ));
        };
        fields.push((name.to_ascii_lowercase(), value.trim().to_owned()));
    }
    let mut headers = BTreeMap::<String, String>::new();
    for (name, value) in fields {
        if let Some(old) = headers.get_mut(&name) {
            match name.as_str() {
                "content-type" | "content-length" => {}
                "cookie" => {
                    old.push_str("; ");
                    old.push_str(value.trim());
                }
                _ => {
                    old.push(',');
                    old.push_str(value.trim());
                }
            }
        } else {
            headers.insert(name, value.trim().to_owned());
        }
    }
    let host = headers.remove("host").unwrap_or_default();
    let origin = headers.remove("origin");
    let content_type = headers.remove("content-type").unwrap_or_default();
    let mut request = ERequest {
        method: parts[0].as_bytes().to_vec(),
        path: request::decode_path(parts[1]),
        host: host.clone().into_bytes(),
        origin: origin.clone().map(String::into_bytes),
        content_type: content_type.clone().into_bytes(),
        cookie: headers.remove("cookie").map(String::into_bytes),
        body: None,
    };
    // The kernel's gate decides Host, sign-in, method + session before a body
    // byte is read; the transport checks below keep a refused body unread.
    if let Some(outcome) = ckc_kernel::contract::ui_gate(&request, config.port, &config.session) {
        return request::gated(outcome)
            .map(Parsed::Answered)
            .map_err(std::io::Error::other);
    }
    let expected = format!("127.0.0.1:{}", config.port);
    let may_read = parts[0] == "POST"
        && host == expected
        && origin
            .as_ref()
            .is_none_or(|v| v == &format!("http://{expected}"))
        && content_type == "application/x-www-form-urlencoded";
    if may_read
        && let Some(length) = headers
            .get("content-length")
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|n| *n <= BODY_CAP)
    {
        let mut bytes = Vec::new();
        if reader
            .by_ref()
            .take(length as u64)
            .read_to_end(&mut bytes)
            .is_ok_and(|n| n == length)
        {
            request.body = Some(bytes);
        }
    }
    Ok(Parsed::Request(request))
}
fn send(stream: &TcpStream, response: &request::Response) -> Result<()> {
    let mut timed = Timed {
        stream,
        until: Instant::now() + DEADLINE,
    };
    let mut header = format!("HTTP/1.0 {} {}\r\n", response.status, response.reason());
    for (name, value) in response.headers()? {
        header.push_str(&format!("{name}: {value}\r\n"));
    }
    header.push_str(&format!("Content-Length: {}\r\n\r\n", response.body.len()));
    timed
        .write_all(header.as_bytes())
        .and_then(|()| timed.write_all(&response.body))
        .map_err(|e| format!("ui: socket: {e}"))
}
// Close without dropping the answer: a client still sending (a body the gate
// refused unread) would turn a plain close into a reset that discards the
// response, so the server half-closes, then drains briefly (≤ 128 KiB, 1 s).
fn linger(stream: &TcpStream) {
    let _ = stream.shutdown(Shutdown::Write);
    let mut rest = Timed {
        stream,
        until: Instant::now() + Duration::from_secs(1),
    };
    let mut sink = [0; 8192];
    for _ in 0..16 {
        match rest.read(&mut sink) {
            Ok(n) if n > 0 => {}
            _ => break,
        }
    }
}
fn handle(stream: &TcpStream, root: &Path, config: &Config) -> Result<()> {
    let result = answer(stream, root, config);
    linger(stream);
    result
}
fn answer(stream: &TcpStream, root: &Path, config: &Config) -> Result<()> {
    let reader = Timed {
        stream,
        until: Instant::now() + DEADLINE,
    };
    let request = match parse(reader, config) {
        Ok(Parsed::Request(request)) => request,
        Ok(Parsed::Answered(Outcome::Response(response))) => return send(stream, &response),
        Ok(Parsed::Answered(Outcome::Crash)) | Ok(Parsed::Closed) => return Ok(()),
        Err(_) => return send(stream, &request::Response::unreadable()),
    };
    let response = match request::respond(root, &request, config)? {
        Outcome::Response(response) => response,
        Outcome::Crash => return Err("ui: unexpected serve fault".into()),
    };
    send(stream, &response)
}
pub(super) fn run(root: &Path, port: u16) -> Result<()> {
    let listener =
        TcpListener::bind((Ipv4Addr::LOCALHOST, port)).map_err(|e| format!("ui: serve: {e}"))?;
    let mut config = Config::new(port);
    config.token = random_hex(32)
        .map_err(|e| format!("ui: serve: {e}"))?
        .into_bytes();
    config.session = random_hex(32)
        .map_err(|e| format!("ui: serve: {e}"))?
        .into_bytes();
    let opener = Arc::new(Opener::new(port, &config.session)?);
    println!("ui: serving http://127.0.0.1:{port}/");
    opener.announce();
    let again = Arc::clone(&opener);
    std::thread::spawn(move || {
        for line in std::io::stdin().lines() {
            if line.is_err() {
                break;
            }
            again.announce();
        }
    });
    for stream in listener.incoming() {
        let stream = stream.map_err(|e| format!("ui: socket: {e}"))?;
        if let Err(error) = handle(&stream, root, &config) {
            eprintln!("{error}");
        }
    }
    Ok(())
}
