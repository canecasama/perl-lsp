use std::process::Command;
use std::str;

pub fn run_parse_perl(perl_code: String) -> Result<String, String> {
    let script_path = "/home/msoares/git_tree/perl-lsp/perl/parse_perl.pl";

    let params = vec![perl_code];

    let output = Command::new("perl")
        .arg(script_path)
        .args(params)
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
