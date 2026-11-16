use sb1_ta::{front, hints, source::Program};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [root, composition, out] = args.as_slice() else {
        return Err("usage: ta-hints <root> <composition> <new-out>".into());
    };
    let (root, composition, out) = (
        PathBuf::from(root),
        PathBuf::from(composition),
        PathBuf::from(out),
    );
    fs::create_dir(&out).map_err(|e| e.to_string())?;
    let mut p = Program::load(&root, &composition, true)?;
    p.dc_shape()?;
    let a = hints::analyze(&p)?;
    let verdict = a.admitted();
    front::features(&p, &a.model.abs, a.model.dump(), &out)?;
    front::graph_rows(&p, |i| a.graph.row(i), &a.graph.components, &out)?;
    a.dump(&p, &out)?;
    let hint = hints::export(&p, a)?;
    // Re-expanded F is a transport control, not an independent checking path.
    let decoded = out.join("hint-front");
    fs::create_dir(&decoded).map_err(|e| e.to_string())?;
    let abs = sb1_ta::domain::Abstraction::new(&p.arena);
    front::features(&p, &abs, hint.model.dump(), &decoded)?;
    let mut stream =
        BufWriter::new(File::create(out.join("hints.tsv")).map_err(|e| e.to_string())?);
    hint.write_canonical(&p, &mut stream)
        .and_then(|_| stream.flush())
        .map_err(|e| e.to_string())?;
    println!(
        "hint\tversion={}\tsolver_admitted={verdict}\tbase_payload_bytes={}\tbase_sources={}\tk1={}",
        hint.version,
        hint.model.payload_bytes(),
        hint.flow.sources.len(),
        hint.refined.is_some()
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("ta-hints: {e}");
        std::process::exit(2);
    }
}
