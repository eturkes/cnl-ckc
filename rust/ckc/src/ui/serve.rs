use super::common::*;
use super::request::{self, Config, Outcome};
use ckc_kernel::ERequest;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::path::Path;
use std::time::Duration;

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
fn parse(stream: &mut TcpStream, port: u16) -> std::io::Result<Option<ERequest>> {
    let mut reader = BufReader::new(stream);
    let request_line = line(&mut reader)?;
    if request_line.is_empty() {
        return Ok(None);
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
            if !matches!(name.as_str(), "content-type" | "content-length") {
                old.push(',');
                old.push_str(value.trim());
            }
        } else {
            headers.insert(name, value.trim().to_owned());
        }
    }
    let host = headers.remove("host").unwrap_or_default();
    let origin = headers.remove("origin");
    let content_type = headers.remove("content-type").unwrap_or_default();
    let mut body = None;
    // These transport checks avoid reading a rejected body. The kernel still
    // decides every guard and refusal over the final request.
    let expected = format!("127.0.0.1:{port}");
    let may_read = parts[0] == "POST"
        && host == expected
        && origin
            .as_ref()
            .is_none_or(|v| v == &format!("http://{expected}"))
        && content_type == "application/x-www-form-urlencoded";
    if may_read {
        if let Some(length) = headers
            .get("content-length")
            .and_then(|s| s.parse::<usize>().ok())
        {
            let mut bytes = Vec::new();
            if bytes.try_reserve_exact(length).is_ok() {
                bytes.resize(length, 0);
                if reader.read_exact(&mut bytes).is_ok() {
                    body = Some(bytes);
                }
            }
        }
    }
    Ok(Some(ERequest {
        method: parts[0].as_bytes().to_vec(),
        path: request::decode_path(parts[1]),
        host: host.into_bytes(),
        origin: origin.map(String::into_bytes),
        content_type: content_type.into_bytes(),
        body,
    }))
}
fn handle(stream: &mut TcpStream, root: &Path, config: &Config) -> Result<()> {
    let request = match parse(stream, config.port) {
        Ok(Some(request)) => request,
        Ok(None) => return Ok(()),
        Err(_) => {
            stream
                .write_all(
                    b"HTTP/1.0 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .map_err(|e| format!("ui: socket: {e}"))?;
            return Ok(());
        }
    };
    let response = match request::respond(root, &request, config)? {
        Outcome::Response(response) => response,
        Outcome::Crash => return Err("ui: unexpected serve fault".into()),
    };
    let mut header = format!("HTTP/1.0 {} {}\r\n", response.status, response.reason());
    for (name, value) in response.headers()? {
        header.push_str(&format!("{name}: {value}\r\n"));
    }
    header.push_str(&format!("Content-Length: {}\r\n\r\n", response.body.len()));
    stream
        .write_all(header.as_bytes())
        .and_then(|()| stream.write_all(&response.body))
        .map_err(|e| format!("ui: socket: {e}"))
}
pub(super) fn run(root: &Path, port: u16) -> Result<()> {
    let listener =
        TcpListener::bind((Ipv4Addr::LOCALHOST, port)).map_err(|e| format!("ui: serve: {e}"))?;
    let mut config = Config::new(port);
    config.token = random_hex(32)
        .map_err(|e| format!("ui: serve: {e}"))?
        .into_bytes();
    println!("ui: serving http://127.0.0.1:{port}/");
    std::io::stdout()
        .flush()
        .map_err(|e| format!("ui: serve: {e}"))?;
    for stream in listener.incoming() {
        let mut stream = stream.map_err(|e| format!("ui: socket: {e}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| format!("ui: socket: {e}"))?;
        stream
            .set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| format!("ui: socket: {e}"))?;
        if let Err(error) = handle(&mut stream, root, &config) {
            eprintln!("{error}");
        }
    }
    Ok(())
}
