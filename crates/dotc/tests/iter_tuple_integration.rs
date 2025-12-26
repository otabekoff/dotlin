use std::process::Command;
use std::path::PathBuf;
use std::str;

#[test]
fn compile_and_run_iter_tuple_example() {
    // Workspace root (two levels up from crates/dotc)
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest.parent().unwrap().parent().unwrap();

    let example = workspace_root.join("examples").join("iter_tuple_test.lin");
    assert!(example.exists(), "example file missing: {:?}", example);

    // Build and run dotc to compile the example to an executable
    let out_exe = workspace_root.join("test_iter_tuple_out.exe");

    let status = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("dotc")
        .arg("--")
        .arg(example.as_os_str())
        .arg("-o")
        .arg(out_exe.as_os_str())
        .current_dir(workspace_root)
        .status()
        .expect("failed to run dotc");

    assert!(status.success(), "dotc failed to compile example");

    // Run the produced executable and capture stdout
    let output = Command::new(out_exe.as_os_str())
        .output()
        .expect("failed to run compiled example");

    let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
    // The program computes 10 + 20 = 30 and prints it
    assert!(stdout.contains("30"), "unexpected stdout: {}", stdout);
}
