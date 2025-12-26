use std::process::Command;
use std::path::PathBuf;
use std::str;

#[test]
fn compile_and_run_iter_example() {
    // Workspace root (two levels up from crates/dotc)
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest.parent().unwrap().parent().unwrap();

    let example = workspace_root.join("examples").join("iter_test.lin");
    assert!(example.exists(), "example file missing: {:?}", example);

    // Ensure runtime lib exists before compiling the example (linker requires import lib)
    let lib_dir = workspace_root.join("lib");
    let _lib_import = lib_dir.join("dotlin_runtime.lib");
    // Build runtime and copy any produced runtime artifacts into workspace `lib/`
    {
        // Use cargo JSON message-format to locate produced artifacts reliably
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
                                        // Accept MSVC import libs, DLLs, Unix shared libs, macOS dylibs, and static archives
                                        if ln.contains("dotlin_runtime") && (ln.ends_with(".lib") || ln.ends_with(".dll") || ln.ends_with(".so") || ln.ends_with(".dylib") || ln.ends_with(".a")) {
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

        // If MSVC produced an import named `dotlin_runtime.dll.lib`, normalize it to `dotlin_runtime.lib`
        let imported = lib_dir.join("dotlin_runtime.lib");
        if !imported.exists() {
            let alt = lib_dir.join("dotlin_runtime.dll.lib");
            if alt.exists() {
                let _ = std::fs::copy(&alt, &imported);
            }
        }

        if !imported.exists() {
            // also accept static archive or shared library on unix: libdotlin_runtime.a, .so, or .dylib
            let alt2 = lib_dir.join("libdotlin_runtime.a");
            let alt3 = lib_dir.join("libdotlin_runtime.so");
            let alt4 = lib_dir.join("libdotlin_runtime.dylib");
            if alt2.exists() || alt3.exists() || alt4.exists() {
                // nothing to do, linker will find these in lib/
            } else {
                panic!("dotlin_runtime import not found after build. copied: {:?}; cargo stdout: {}", copied, stdout_str);
            }
        }
    }

    // Build and run dotc to compile the example to an executable
    let out_exe = workspace_root.join("test_iter_out.exe");

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
    // Ensure runtime lib/DLL present; if missing, build `dotlin_runtime` and copy artifacts.
    let lib_dir = workspace_root.join("lib");
    let lib_import = lib_dir.join("dotlin_runtime.lib");
    let _dll = lib_dir.join("dotlin_runtime.dll");
    if cfg!(target_os = "windows") && !lib_import.exists() {
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

    // Ensure the produced executable can find the runtime library and is runnable
    let mut cmd = Command::new(out_exe.as_os_str());
    if cfg!(target_os = "windows") {
        // Add workspace lib and candidate target release dirs to PATH
        let mut path_entries: Vec<String> = Vec::new();
        path_entries.push(workspace_root.join("lib").display().to_string());
        path_entries.push(workspace_root.join("target").join("release").display().to_string());
        path_entries.push(workspace_root.join("target").join("release").join("deps").display().to_string());
        if let Ok(path_var) = std::env::var("PATH") {
            let mut entries = path_entries.join(";");
            entries.push_str(";");
            entries.push_str(&path_var);
            cmd.env("PATH", entries);
        } else {
            cmd.env("PATH", path_entries.join(";"));
        }
    }
    // On unix-like systems, set LD_LIBRARY_PATH and ensure the produced file is executable
    if !cfg!(target_os = "windows") {
        let mut path_entries: Vec<String> = Vec::new();
        path_entries.push(workspace_root.join("lib").display().to_string());
        path_entries.push(workspace_root.join("target").join("release").display().to_string());
        path_entries.push(workspace_root.join("target").join("release").join("deps").display().to_string());
        if let Ok(ld) = std::env::var("LD_LIBRARY_PATH") {
            let mut entries = path_entries.join(":");
            entries.push_str(":");
            entries.push_str(&ld);
            cmd.env("LD_LIBRARY_PATH", entries);
        } else {
            cmd.env("LD_LIBRARY_PATH", path_entries.join(":"));
        }

        // Ensure executable bit is set for the produced file
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&out_exe) {
                let mut perms = metadata.permissions();
                let mode = perms.mode();
                if mode & 0o100 == 0 {
                    let _ = std::fs::set_permissions(&out_exe, std::fs::Permissions::from_mode(0o755));
                }
            }
        }
    }

    let output = cmd
        .output()
        .expect("failed to run compiled example");

    let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
    let stderr = str::from_utf8(&output.stderr).unwrap_or_default();
    // The program computes 1 + 2 = 3 and prints it
    if !stdout.contains("3") {
        // Collect extra diagnostics for CI: exit code, stderr, and ldd output on unix
        let status = output.status;
        let mut diag = format!("unexpected stdout: {}\nexit: {:?}\nstderr: {}", stdout, status, stderr);
        if !cfg!(target_os = "windows") {
            if let Ok(ldd_out) = std::process::Command::new("ldd").arg(&out_exe).output() {
                let ldd_stdout = String::from_utf8_lossy(&ldd_out.stdout);
                let ldd_stderr = String::from_utf8_lossy(&ldd_out.stderr);
                diag.push_str("\nldd stdout:\n");
                diag.push_str(&ldd_stdout);
                diag.push_str("\nldd stderr:\n");
                diag.push_str(&ldd_stderr);
            }
            // If the process exited due to a signal (eg SIGSEGV), attempt further diagnostics
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                if let Some(sig) = status.signal() {
                    diag.push_str(&format!("\nprocess signaled: {}\n", sig));
                    // Try to run gdb to get a backtrace if available
                    if let Ok(gdb_out) = std::process::Command::new("gdb")
                        .arg("--batch")
                        .arg("-ex")
                        .arg("set pagination off")
                        .arg("-ex")
                        .arg("run")
                        .arg("-ex")
                        .arg("bt")
                        .arg("--args")
                        .arg(&out_exe)
                        .output()
                    {
                        diag.push_str("\ngdb stdout:\n");
                        diag.push_str(&String::from_utf8_lossy(&gdb_out.stdout));
                        diag.push_str("\ngdb stderr:\n");
                        diag.push_str(&String::from_utf8_lossy(&gdb_out.stderr));
                    }

                    // Inspect any staged runtime libs
                    let lib_dir = workspace_root.join("lib");
                    if lib_dir.exists() {
                        if let Ok(entries) = std::fs::read_dir(&lib_dir) {
                            for ent in entries.flatten() {
                                if let Ok(fname) = ent.file_name().into_string() {
                                    let ln = fname.to_ascii_lowercase();
                                    if ln.contains("dotlin_runtime") && (ln.ends_with(".so") || ln.ends_with(".dylib") || ln.ends_with(".a") ) {
                                        let fpath = lib_dir.join(&fname);
                                        if let Ok(out) = std::process::Command::new("readelf").arg("-Ws").arg(&fpath).output() {
                                            diag.push_str(&format!("\nreadelf -Ws {} stdout:\n", fpath.display()));
                                            diag.push_str(&String::from_utf8_lossy(&out.stdout));
                                        }
                                        if let Ok(out) = std::process::Command::new("objdump").arg("-T").arg(&fpath).output() {
                                            diag.push_str(&format!("\nobjdump -T {} stdout:\n", fpath.display()));
                                            diag.push_str(&String::from_utf8_lossy(&out.stdout));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        panic!("{}", diag);
    }
}
