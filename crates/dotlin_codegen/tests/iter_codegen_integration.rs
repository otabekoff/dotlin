use std::process::Command;
use std::path::PathBuf;
use std::str;

#[test]
fn compile_and_run_iter_tuple_example_codegen() {
    // Workspace root (two levels up from crates/dotlin_codegen)
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest.parent().unwrap().parent().unwrap();

    let example = workspace_root.join("examples").join("iter_tuple_test.lin");
    assert!(example.exists(), "example file missing: {:?}", example);

    // Build and run dotc to compile the example to an executable
    let out_exe = workspace_root.join("test_iter_codegen_out.exe");

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
    // Ensure runtime DLL in workspace `lib/` is present; if missing, build it.
    let lib_dir = workspace_root.join("lib");
    let dll_path = lib_dir.join("dotlin_runtime.dll");
    if cfg!(target_os = "windows") && !dll_path.exists() {
        // Try to build the runtime crate to produce the DLL/import lib
        let build_status = Command::new("cargo")
            .arg("build")
            .arg("-p")
            .arg("dotlin_runtime")
            .arg("--release")
            .current_dir(workspace_root)
            .status()
            .expect("failed to run cargo build for dotlin_runtime");
        assert!(build_status.success(), "building dotlin_runtime failed");
    }

    // Ensure runtime DLL in workspace `lib/` is discoverable at runtime (Windows loader uses PATH)
    let mut cmd = Command::new(out_exe.as_os_str());
    if cfg!(target_os = "windows") {
        // Prepare candidate target directories: prefer CARGO_TARGET_DIR if set
        let mut candidate_targets: Vec<PathBuf> = Vec::new();
        if let Ok(ctd) = std::env::var("CARGO_TARGET_DIR") {
            candidate_targets.push(PathBuf::from(ctd));
        }
        candidate_targets.push(workspace_root.join("target"));

        // Ensure workspace lib is first on PATH
        let mut path_entries: Vec<String> = Vec::new();
        path_entries.push(workspace_root.join("lib").display().to_string());

        // Add release/deps dirs for each candidate target
        for cand in &candidate_targets {
            path_entries.push(cand.join("release").display().to_string());
            path_entries.push(cand.join("release").join("deps").display().to_string());
        }

        // If a dotlin_runtime.dll exists under any candidate target, copy it into lib/
        fn find_dll(dir: &std::path::Path) -> Option<std::path::PathBuf> {
            if !dir.exists() {
                return None;
            }
            if let Ok(entries) = std::fs::read_dir(dir) {
                for e in entries.flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        if let Some(found) = find_dll(&p) { return Some(found); }
                    } else if let Some(n) = p.file_name().and_then(|s| s.to_str()) {
                        if n.to_ascii_lowercase().contains("dotlin_runtime") && n.to_ascii_lowercase().ends_with(".dll") {
                            return Some(p);
                        }
                    }
                }
            }
            None
        }

        for cand in &candidate_targets {
            if let Some(found) = find_dll(cand) {
                let _ = std::fs::create_dir_all(&lib_dir);
                let dest = lib_dir.join("dotlin_runtime.dll");
                let _ = std::fs::copy(&found, &dest);
                break;
            }
        }

        // Merge with existing PATH
        if let Ok(path_var) = std::env::var("PATH") {
            let mut entries = path_entries.join(";");
            entries.push_str(";");
            entries.push_str(&path_var);
            cmd.env("PATH", entries);
        } else {
            cmd.env("PATH", path_entries.join(";"));
        }
    }

    let output = cmd
        .output()
        .expect("failed to run compiled example");

    let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
    let stderr = str::from_utf8(&output.stderr).unwrap_or_default();
    let exit_code = output.status.code();
    // The program computes 10 + 20 = 30 and prints it
    assert!(output.status.success(), "compiled example failed (exit code: {:?})\nstdout: {}\nstderr: {}", exit_code, stdout, stderr);
    assert!(stdout.contains("30"), "unexpected stdout: {}\nstderr: {}\nexit: {:?}", stdout, stderr, exit_code);
}
