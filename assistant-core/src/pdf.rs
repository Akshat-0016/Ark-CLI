use std::process::Command;
use std::fs;

pub fn read_pdf(path: &str) -> Option<String> {
    let output_path = "/tmp/ark_pdf_extract.txt";

    // Run `pdftotext`
    let status = Command::new("pdftotext")
        .arg(path)
        .arg(output_path)
        .status()
        .ok()?;

    if !status.success() {
        return None;
    }

    // Read extracted text
    fs::read_to_string(output_path).ok()
}
