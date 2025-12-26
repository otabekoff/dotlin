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

        // Ensure runtime lib exists before compiling the example (linker requires import lib)
        let lib_dir = workspace_root.join("lib");
        let _lib_import = lib_dir.join("dotlin_runtime.lib");
            // Build runtime and copy any produced runtime artifacts into workspace `lib/`
            {
                let out = Command::new("cargo")
                    .arg("build")
                    .arg("-p")
                    .arg("dotlin_runtime")
                    .arg("--release")
                    .arg("--message-format=json")
                    .current_dir(workspace_root)
                    .output()
                    .expect("failed to run cargo build for dotlin_runtime");
                assert!(out.status.success(), "building dotlin_runtime failed");

                std::fs::create_dir_all(&lib_dir).expect("failed to create lib dir");
                let mut copied: Vec<String> = Vec::new();
                let stdout_str = String::from_utf8_lossy(&out.stdout).to_string();
                for line in stdout_str.lines() {
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
                        if val.get("reason").and_then(|r| r.as_str()) == Some("compiler-artifact") {
                            if let Some(filenames) = val.get("filenames").and_then(|f| f.as_array()) {
                                for fname in filenames {
                                    if let Some(path_str) = fname.as_str() {
                                        let p = std::path::PathBuf::from(path_str);
                                        if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                                            let ln = name.to_ascii_lowercase();
                                            if ln.contains("dotlin_runtime") && (ln.ends_with(".lib") || ln.ends_with(".dll") || ln.ends_with(".a") || ln.ends_with(".dylib")) {
                                                let _ = std::fs::copy(&p, lib_dir.join(name));
                                                copied.push(name.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Normalize MSVC import lib name if necessary
                let imported = lib_dir.join("dotlin_runtime.lib");
                if !imported.exists() {
                    let alt = lib_dir.join("dotlin_runtime.dll.lib");
                    if alt.exists() {
                        let _ = std::fs::copy(&alt, &imported);
                    }
                }

                if !imported.exists() {
                    // allow static archive on unix/mac
                    let alt2 = lib_dir.join("libdotlin_runtime.a");
                    let alt3 = lib_dir.join("libdotlin_runtime.dylib");
                    if !alt2.exists() && !alt3.exists() {
                        panic!("dotlin_runtime import not found after build. copied: {:?}; cargo stdout: {}", copied, stdout_str);
                    }
                }
        }

        // Build dotc to compile the example to an executable
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

        // If dotc reported success but the output exe wasn't created, capture diagnostics.
        if !out_exe.exists() {
            eprintln!("Expected compiled exe not found: {:?}", out_exe);
            // Re-run dotc to capture stdout/stderr for debugging
            let out = Command::new("cargo")
                .arg("run")
                .arg("-p")
                .arg("dotc")
                .arg("--")
                .arg(example.as_os_str())
                .arg("-o")
                .arg(out_exe.as_os_str())
                .current_dir(workspace_root)
                .output()
                .expect("failed to run dotc for diagnostics");

            eprintln!("dotc stdout:\n{}", String::from_utf8_lossy(&out.stdout));
            eprintln!("dotc stderr:\n{}", String::from_utf8_lossy(&out.stderr));

            // List workspace root and target folders for visibility
            if let Ok(entries) = std::fs::read_dir(workspace_root) {
                eprintln!("Workspace root listing:");
                for ent in entries.flatten() {
                    eprintln!(" - {:?}", ent.path());
                }
            }
            let target_dir = workspace_root.join("target");
            if let Ok(entries) = std::fs::read_dir(&target_dir) {
                eprintln!("Target dir listing:");
                for ent in entries.flatten() {
                    eprintln!(" - {:?}", ent.path());
                }
            }

            panic!("dotc did not produce expected executable: {:?}", out_exe);
        }

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

    if !output.status.success() {
        eprintln!("=== EXECUTABLE FAILED ===");
        eprintln!("Exit status: {:?}", output.status);
        eprintln!("Exit code: {:?}", output.status.code());

        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            if let Some(signal) = output.status.signal() {
                eprintln!("Killed by signal: {} ({})", signal,
                    match signal {
                        6 => "SIGABRT",
                        11 => "SIGSEGV (segmentation fault)",
                        _ => "unknown",
                    });
            }
        }

        eprintln!("\n--- STDOUT ---");
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("\n--- STDERR ---");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        eprintln!("\n--- Attempting to run with gdb (if available) ---");
        let _ = Command::new("gdb")
            .args(&["-batch", "-ex", "run", "-ex", "bt", "--args"])
            .arg(&out_exe)
            .status();

        // Show staged lib contents
        eprintln!("Workspace lib dir: {:?}", lib_dir);
        if let Ok(entries) = std::fs::read_dir(&lib_dir) {
            for ent in entries.flatten() {
                eprintln!(" - {:?}", ent.path());
            }
        }

        // On Unix-like systems, print dynamic deps and examine staged libs
        #[cfg(target_family = "unix")]
        {
            use std::process::Command as Cmd;
            // ldd (Linux) or otool (macOS)
            if cfg!(target_os = "macos") {
                let _ = Cmd::new("otool").arg("-L").arg(&out_exe).status();
            } else {
                let _ = Cmd::new("ldd").arg(&out_exe).status();
            }

            // Show file information for architecture/calling-convention clues
            let _ = Cmd::new("file").arg(&out_exe).status();

            if let Ok(entries) = std::fs::read_dir(&lib_dir) {
                for ent in entries.flatten() {
                    let p = ent.path();
                    if p.is_file() {
                        eprintln!("--- readelf -Ws {:?} ---", p);
                        let _ = Cmd::new("readelf").arg("-Ws").arg(&p).status();
                        eprintln!("--- objdump -T {:?} ---", p);
                        let _ = Cmd::new("objdump").arg("-T").arg(&p).status();
                    }
                }
            }
        }

        panic!("compiled example failed (exit code: {:?})", exit_code);
    }

    // The program computes 10 + 20 = 30 and prints it
    assert!(stdout.contains("30"), "unexpected stdout: {}\nstderr: {}\nexit: {:?}", stdout, stderr, exit_code);
}

#[test]
fn test_basic_hello_world_codegen() {
    // Workspace root
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest.parent().unwrap().parent().unwrap();

    let src = workspace_root.join("test_basic_hello_world.lin");
    let _ = std::fs::write(&src, r#"fun main() { println("Hello"); }"#);

    let out_exe = workspace_root.join("test_basic_hello_world_out.exe");

    let status = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("dotc")
        .arg("--")
        .arg(src.as_os_str())
        .arg("-o")
        .arg(out_exe.as_os_str())
        .current_dir(workspace_root)
        .status()
        .expect("failed to run dotc");

    assert!(status.success(), "dotc failed to compile basic example");

    let output = Command::new(&out_exe)
        .output()
        .expect("failed to run basic example");

    assert!(output.status.success(), "basic hello world failed to run: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Hello"), "unexpected stdout");
}
