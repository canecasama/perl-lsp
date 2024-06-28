use std::process::Command;
use std::str;

pub fn run_parse_perl(filename: String) -> Result<String, String> {
    let output = Command::new("perl")
        .arg(format!("../../perl/parse_perl.pl {}", filename))
        .output()
        .expect("Failed to execute parse_perl.pl");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).unwrap();
        return Ok(stdout.to_string());
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap();
        return Err(stderr.to_string());
    }
}
