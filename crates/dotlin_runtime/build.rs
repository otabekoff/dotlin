use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);
    
    // Find the target directory by going up from OUT_DIR
    let target_dir = out_dir_path
        .parent()  // Remove /out
        .and_then(|p| p.parent())  // Remove /dotlin_runtime-*
        .and_then(|p| p.parent())  // Remove /build
        .and_then(|p| p.parent())  // Remove /target
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
    let lib_filename = "dotlin_runtime.lib";
    #[cfg(not(windows))]
    let lib_filename = "libdotlin_runtime.a";
    
    // Find the built library in the deps directory
    let deps_dir = target_dir.join(env::var("PROFILE").unwrap()).join("deps");
    
    // Look for the library file in deps directory
    if let Ok(entries) = fs::read_dir(&deps_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                
                if file_name_str.starts_with("libdotlin_runtime") && 
                   (file_name_str.ends_with(".a") || file_name_str.ends_with(".lib")) {
                    let src_path = entry.path();
                    let dest_path = lib_dir.join(lib_filename);
                    
                    // Copy the library to the lib directory
                    match fs::copy(&src_path, &dest_path) {
                        Ok(_) => {
                            println!("Copied {} to lib/ directory", file_name_str);
                            break;
                        }
                        Err(e) => {
                            eprintln!("Failed to copy library: {}", e);
                        }
                    }
                }
            }
        }
    }
    
    // Also set up linker arguments to look in the lib directory
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}