use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);

    // Find the target directory by going up from OUT_DIR
    let target_dir = out_dir_path
        .parent() // Remove /out
        .and_then(|p| p.parent()) // Remove /dotlin_runtime-*
        .and_then(|p| p.parent()) // Remove /build
        .and_then(|p| p.parent()) // Remove /target
        .map(|p| p.to_path_buf())
        .expect("Could not find target directory");

    // Determine the workspace root (where Cargo.toml is)
    let workspace_root = target_dir.parent().expect("Could not find workspace root");

    // Determine the library directory
    let lib_dir = workspace_root.join("lib");

    // Ensure lib directory exists
    fs::create_dir_all(&lib_dir).expect("Could not create lib directory");

    // The library name and extension based on the target OS
    #[cfg(windows)]
    let _lib_filename = "dotlin_runtime.lib"; // import library produced alongside DLL
    #[cfg(not(windows))]
    let _lib_filename = "libdotlin_runtime.a";

    // Search recursively under several likely output directories for import libraries or DLLs
    let mut found_import: Option<PathBuf> = None;
    let mut found_dll: Option<PathBuf> = None;

    fn visit_dir(dir: &Path, found_import: &mut Option<PathBuf>, found_dll: &mut Option<PathBuf>) {
        if !dir.exists() {
            return;
        }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_dir(&path, found_import, found_dll);
                } else if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    let lname = name.to_ascii_lowercase();
                    if lname.contains("dotlin_runtime") {
                        if lname.ends_with(".lib") || lname.ends_with(".dll.lib") {
                            if found_import.is_none() {
                                *found_import = Some(path.clone());
                            }
                        } else if lname.ends_with(".dll") {
                            if found_dll.is_none() {
                                *found_dll = Some(path.clone());
                            }
                        } else if lname.ends_with(".a") || lname.ends_with(".rlib") {
                            // treat static archives as import candidates for non-windows
                            if found_import.is_none() {
                                *found_import = Some(path.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    // Primary scan: the inferred target directory (covers most cases)
    visit_dir(&target_dir, &mut found_import, &mut found_dll);

    // Additional common locations: release and release/deps
    let release = target_dir.join("release");
    let release_deps = release.join("deps");
    visit_dir(&release, &mut found_import, &mut found_dll);
    visit_dir(&release_deps, &mut found_import, &mut found_dll);

    // Also check triple-specific layout (e.g., target/<triple>/release and deps)
    if let Ok(triple) = env::var("TARGET") {
        let triple_dir = target_dir.join(&triple).join("release");
        let triple_deps = triple_dir.join("deps");
        visit_dir(&triple_dir, &mut found_import, &mut found_dll);
        visit_dir(&triple_deps, &mut found_import, &mut found_dll);
    }

    // Copy import lib if available
    if let Some(ref import_path) = found_import {
        let dest_path = lib_dir.join("dotlin_runtime.lib");
        match fs::copy(import_path, &dest_path) {
            Ok(_) => println!(
                "Copied import lib {} to lib/ directory",
                import_path.display()
            ),
            Err(e) => eprintln!("Failed to copy import lib {}: {}", import_path.display(), e),
        }
    }

    // Copy DLL if available
    if let Some(ref dll_path) = found_dll {
        let dest_path = lib_dir.join("dotlin_runtime.dll");
        match fs::copy(dll_path, &dest_path) {
            Ok(_) => println!("Copied DLL {} to lib/ directory", dll_path.display()),
            Err(e) => eprintln!("Failed to copy DLL {}: {}", dll_path.display(), e),
        }
    }

    // If neither was found, warn (we already attempted multiple locations)
    if found_import.is_none() && found_dll.is_none() {
        eprintln!("Warning: could not find built dotlin_runtime artifact under {} (searched common release/deps locations)", target_dir.display());
    }

    // Also set up linker arguments to look in the lib directory
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}
