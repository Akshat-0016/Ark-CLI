use std::fs;
use std::process::Command;

pub fn read_pdf(path: &str) -> Option<String> {
    let output_path = "/tmp/ark_pdf_extract.txt";

    let status = Command::new("pdftotext")
        .arg(path)
        .arg(output_path)
        .status()
        .ok()?;

    if !status.success() {
        return None;
    }

    fs::read_to_string(output_path).ok()
}
