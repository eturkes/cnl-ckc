use super::common::*;
use flate2::{Compression, GzBuilder};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn error(detail: impl AsRef<str>) -> Failure {
    violation("dist", detail)
}
fn octal(field: &mut [u8], value: i128) -> Result {
    let digits = if value < 0 {
        format!("-{:o}", value.unsigned_abs())
    } else {
        format!("{value:o}")
    };
    if digits.len() >= field.len() {
        return Err(error("tar numeric field overflow"));
    }
    let end = field.len() - 1;
    field[..end].fill(b'0');
    field[end - digits.len()..end].copy_from_slice(digits.as_bytes());
    field[end] = 0;
    Ok(())
}
fn header(name: &[u8], size: usize, epoch: i64, kind: u8) -> Result<[u8; 512]> {
    let mut h = [0; 512];
    h[..name.len()].copy_from_slice(name);
    octal(&mut h[100..108], 0o644)?;
    octal(&mut h[108..116], 0)?;
    octal(&mut h[116..124], 0)?;
    octal(&mut h[124..136], size as i128)?;
    octal(&mut h[136..148], epoch as i128)?;
    h[148..156].fill(b' ');
    h[156] = kind;
    h[257..263].copy_from_slice(b"ustar\0");
    h[263..265].copy_from_slice(b"00");
    octal(&mut h[329..337], 0)?;
    octal(&mut h[337..345], 0)?;
    let checksum: i128 = h.iter().map(|b| i128::from(*b)).sum();
    octal(&mut h[148..155], checksum)?;
    Ok(h)
}
fn write(out: &mut impl Write, bytes: &[u8]) -> Result {
    out.write_all(bytes).map_err(|e| error(e.to_string()))
}
fn record(out: &mut impl Write, name: &[u8], bytes: &[u8], epoch: i64, kind: u8) -> Result<usize> {
    write(out, &header(name, bytes.len(), epoch, kind)?)?;
    write(out, bytes)?;
    let pad = (512 - bytes.len() % 512) % 512;
    write(out, &[0; 512][..pad])?;
    Ok(512 + bytes.len() + pad)
}

pub(super) fn build(files: &BTreeMap<String, Vec<u8>>, epoch: i64) -> Result<Vec<u8>> {
    let mut out = GzBuilder::new()
        .mtime(0)
        .operating_system(255)
        .write(Vec::new(), Compression::new(9));
    let mut total = 0;
    for (name, data) in files {
        let name = name.as_bytes();
        if name.len() > 100 {
            let mut long = name.to_vec();
            long.push(0);
            total += record(&mut out, b"././@LongLink", &long, 0, b'L')?;
        }
        total += record(&mut out, &name[..name.len().min(100)], data, epoch, b'0')?;
    }
    write(&mut out, &[0; 1024])?;
    total += 1024;
    write(&mut out, &[0; 10240][..(10240 - total % 10240) % 10240])?;
    out.finish().map_err(|e| error(e.to_string()))
}

fn number(field: &[u8]) -> Result<usize> {
    let text = std::str::from_utf8(field).map_err(|_| error("archive invalid numeric field"))?;
    usize::from_str_radix(text.trim_matches([' ', '\0']), 8)
        .map_err(|_| error("archive invalid numeric field"))
}

// Validate the emitted regular-file/LongLink subset before materializing the bag.
pub(super) fn extract(raw: &[u8], bag: &str, destination: &Path) -> Result<usize> {
    let mut tar = Vec::new();
    flate2::read::GzDecoder::new(raw)
        .read_to_end(&mut tar)
        .map_err(|e| error(format!("archive gzip: {e}")))?;
    let prefix = format!("{bag}/");
    let tags = [
        "bagit.txt",
        "manifest-sha256.txt",
        "tagmanifest-sha256.txt",
        "README-dist.md",
        "NOTICE",
        "release-manifest.tsv",
    ];
    let mut at = 0usize;
    let mut long: Option<String> = None;
    let mut seen = BTreeSet::new();
    loop {
        let end = at
            .checked_add(512)
            .ok_or_else(|| error("archive offset overflow"))?;
        let h = tar
            .get(at..end)
            .ok_or_else(|| error("archive truncated header"))?;
        if h.iter().all(|b| *b == 0) {
            if long.is_some() {
                return Err(error("archive long name without member"));
            }
            return Ok(seen.len());
        }
        let sum: usize = h
            .iter()
            .enumerate()
            .map(|(i, b)| usize::from(if (148..156).contains(&i) { b' ' } else { *b }))
            .sum();
        if number(&h[148..156])? != sum {
            return Err(error("archive header checksum mismatch"));
        }
        let size = number(&h[124..136])?;
        let body_end = end
            .checked_add(size)
            .ok_or_else(|| error("archive offset overflow"))?;
        let body = tar
            .get(end..body_end)
            .ok_or_else(|| error("archive truncated member"))?;
        at = body_end
            .checked_add((512 - size % 512) % 512)
            .ok_or_else(|| error("archive offset overflow"))?;
        if at > tar.len() {
            return Err(error("archive truncated padding"));
        }
        if h[156] == b'L' {
            let name = body
                .strip_suffix(&[0])
                .ok_or_else(|| error("archive invalid long name"))?;
            long = Some(
                std::str::from_utf8(name)
                    .map_err(|_| error("archive invalid member name"))?
                    .to_owned(),
            );
            continue;
        }
        let name = match long.take() {
            Some(name) => name,
            None => std::str::from_utf8(h[..100].split(|b| *b == 0).next().unwrap_or_default())
                .map_err(|_| error("archive invalid member name"))?
                .to_owned(),
        };
        if h[156] != b'0' && h[156] != 0 {
            return Err(error(format!(
                "archive member is not a regular file: {name}"
            )));
        }
        super::pipeline_release::path_text(name.as_bytes())?;
        let relative = name
            .strip_prefix(&prefix)
            .ok_or_else(|| error(format!("archive member outside bag root: {name}")))?;
        if !tags.contains(&relative) && !relative.starts_with("data/guidelines/") {
            return Err(error(format!("archive member outside layout: {relative}")));
        }
        if !seen.insert(name.clone()) {
            return Err(error(format!("archive duplicate member: {name}")));
        }
        let target = destination.join(name);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| error(e.to_string()))?;
        }
        fs::write(target, body).map_err(|e| error(e.to_string()))?;
    }
}
