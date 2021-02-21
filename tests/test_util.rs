use std::env::var;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub fn open_assets_file(name: &str) -> anyhow::Result<std::io::BufReader<std::fs::File>> {
    let path = format!(
        "{}/tests/assets/{}",
        var("CARGO_MANIFEST_DIR").expect("No environment value `CARGO_MANIFEST_DIR`"),
        name
    );
    let file = File::open(path)?;
    return Ok(BufReader::new(file));
}
