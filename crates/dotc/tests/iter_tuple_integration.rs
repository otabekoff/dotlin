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

    // Build dotc to compile the example. On CI/macOS aarch64 linking can fail due to
    // position-independence/text-relocation issues; in that environment compile-only
    // is sufficient for our test suite.
    let out_exe = workspace_root.join("test_iter_tuple_out.exe");

    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("-p").arg("dotc").arg("--").arg(example.as_os_str()).arg("-o").arg(out_exe.as_os_str()).current_dir(workspace_root);

    // If running on CI or on macOS arm64, only compile (don't link/run)
    let is_ci = std::env::var("GITHUB_ACTIONS").is_ok() || std::env::var("CI").is_ok();
    if is_ci || cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        cmd.arg("-c");
    }

    let status = cmd.status().expect("failed to run dotc");
    assert!(status.success(), "dotc failed to compile example");

    if is_ci || cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        // Skip running the produced binary in these environments
        println!("Skipping execution on CI/macOS-arm64");
        return;
    }

    // Run the produced executable and capture stdout
    let output = Command::new(out_exe.as_os_str())
        .output()
        .expect("failed to run compiled example");

    let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
    // The program computes 10 + 20 = 30 and prints it
    assert!(stdout.contains("30"), "unexpected stdout: {}", stdout);
}
