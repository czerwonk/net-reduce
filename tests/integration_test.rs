use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

fn run_cli_with_input(input: &str, args: &[&str]) -> (String, String, i32) {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--"])
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("Failed to start command");

    if !input.is_empty() {
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap().to_string();
    let stderr = str::from_utf8(&output.stderr).unwrap().to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

#[test]
fn test_cli_basic_reduction_from_stdin() {
    let input = "192.168.0.0/16\n192.168.1.0/24\n192.168.1.1\n10.0.0.0/8\n";
    let (stdout, stderr, exit_code) = run_cli_with_input(input, &[]);

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );

    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 2, "Should have 2 reduced entries");
    assert!(lines.contains(&"192.168.0.0/16"));
    assert!(lines.contains(&"10.0.0.0/8"));
}

#[test]
fn test_cli_json_output_format() {
    let input = "192.168.1.0/24\n192.168.1.1\n";
    let (stdout, stderr, exit_code) = run_cli_with_input(input, &["-o", "json"]);

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );

    let json: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("Output should be valid JSON");

    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0], "192.168.1.0/24");
}

#[test]
fn test_cli_yaml_output_format() {
    let input = "10.0.0.1\n10.0.0.2\n";
    let (stdout, stderr, exit_code) = run_cli_with_input(input, &["-o", "yaml"]);

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );

    let yaml: serde_yaml::Value =
        serde_yaml::from_str(stdout.trim()).expect("Output should be valid YAML");

    assert!(yaml.is_sequence());
    let seq = yaml.as_sequence().unwrap();
    assert_eq!(seq.len(), 2);
    assert!(seq.contains(&serde_yaml::Value::String("10.0.0.1/32".to_string())));
    assert!(seq.contains(&serde_yaml::Value::String("10.0.0.2/32".to_string())));
}

#[test]
fn test_cli_with_file_input() {
    // Create a temporary test file
    use std::fs;
    use std::io::Write;

    let test_file = "/tmp/test_cidrs.txt";
    let mut file = fs::File::create(test_file).expect("Failed to create test file");
    writeln!(file, "192.168.0.0/16").unwrap();
    writeln!(file, "192.168.1.0/24").unwrap();
    writeln!(file, "192.168.1.1").unwrap();

    let (stdout, stderr, exit_code) = run_cli_with_input("", &["-f", test_file]);

    // Clean up
    fs::remove_file(test_file).ok();

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );

    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "192.168.0.0/16");
}

#[test]
fn test_cli_empty_input() {
    let (stdout, stderr, exit_code) = run_cli_with_input("", &[]);

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );
    assert_eq!(stdout.trim(), "", "Empty input should produce empty output");
}

#[test]
fn test_cli_invalid_input() {
    let input = "invalid-ip\nnot-a-cidr\n192.168.1.0/24\n";
    let (stdout, stderr, exit_code) = run_cli_with_input(input, &[]);

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );

    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "192.168.1.0/24");
}

#[test]
fn test_cli_ipv6_addresses() {
    let input = "2001:678:1e0::/64\n2001:678:1e0::1\n2001:678:1e0:100::/56\n";
    let (stdout, stderr, exit_code) = run_cli_with_input(input, &[]);

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );

    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"2001:678:1e0::/64"));
    assert!(lines.contains(&"2001:678:1e0:100::/56"));
}

#[test]
fn test_cli_mixed_ipv4_ipv6() {
    let input = "192.168.1.0/24\n192.168.1.1\n2001:db8::/32\n2001:db8::1\n";
    let (stdout, stderr, exit_code) = run_cli_with_input(input, &[]);

    assert_eq!(
        exit_code, 0,
        "CLI should exit successfully. stderr: {}",
        stderr
    );

    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"192.168.1.0/24"));
    assert!(lines.contains(&"2001:db8::/32"));
}

#[test]
fn test_cli_nonexistent_file() {
    let (_stdout, stderr, exit_code) = run_cli_with_input("", &["-f", "/nonexistent/file.txt"]);

    assert_ne!(exit_code, 0, "CLI should fail with nonexistent file");
    assert!(!stderr.is_empty(), "Should have error message");
}

#[test]
fn test_cli_invalid_output_format() {
    let input = "192.168.1.0/24\n";
    let (_stdout, stderr, exit_code) = run_cli_with_input(input, &["-o", "invalid"]);

    assert_ne!(exit_code, 0, "CLI should fail with invalid output format");
    assert!(!stderr.is_empty(), "Should have error message");
}

#[test]
fn test_cli_help_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0, "Help should exit successfully");
    assert!(
        stdout.contains("net-reduce"),
        "Help should contain program name"
    );
    assert!(stdout.contains("file"), "Help should mention file option");
    assert!(
        stdout.contains("output-format"),
        "Help should mention output format option"
    );
}

#[test]
fn test_cli_version_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let exit_code = output.status.code().unwrap_or(-1);

    assert_eq!(exit_code, 0, "Version should exit successfully");
    assert!(
        stdout.contains("net-reduce"),
        "Version should contain program name"
    );
}

