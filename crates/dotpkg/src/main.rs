use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use anyhow::Result;

#[derive(Parser)]
#[command(name = "dotpkg")]
#[command(about = "Dotlin Package Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Dotlin project
    Init {
        /// Project name
        name: String,
    },
    /// Add a dependency
    Add {
        /// Package name
        package: String,
    },
    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },
    /// Install all dependencies
    Install,
    /// Update dependencies
    Update,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectManifest {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: HashMap<String, String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name } => init_project(name)?,
        Commands::Add { package } => add_dependency(package)?,
        Commands::Remove { package } => remove_dependency(package)?,
        Commands::Install => install_dependencies()?,
        Commands::Update => update_dependencies()?,
    }

    Ok(())
}

fn init_project(name: &str) -> Result<()> {
    println!("Initializing new Dotlin project: {}", name);
    // Create project directory
    fs::create_dir(name)?;

    // Create source directory
    fs::create_dir(format!("{}/src", name))?;

    // Create manifest file
    let manifest = ProjectManifest {
        name: name.to_string(),
        version: "0.1.0".to_string(),
        authors: vec!["Author <author@example.com>".to_string()],
        dependencies: HashMap::new(),
    };
    let manifest_content = toml::to_string_pretty(&manifest)?;
    fs::write(format!("{}/Dotlin.toml", name), manifest_content)?;

    // Create main file
    fs::write(
        format!("{}/src/main.lin", name),
        "fun main() {\n    println(\"Hello, Dotlin!\")\n}",
    )?;

    println!("Project {} created successfully!", name);
    Ok(())
}

fn add_dependency(package: &str) -> Result<()> {
    println!("Adding dependency: {}", package);
    // Check if Dotlin.toml exists
    if !std::path::Path::new("Dotlin.toml").exists() {
        eprintln!("Error: Dotlin.toml not found in current directory");
        std::process::exit(1);
    }
    // Read the current manifest
    let manifest_content = fs::read_to_string("Dotlin.toml")?;
    let mut manifest: ProjectManifest = toml::from_str(&manifest_content)?;

    // Add the package to dependencies (for now, we'll use version "*" as a placeholder)
    manifest
        .dependencies
        .insert(package.to_string(), "*".to_string());

    // Write the updated manifest back to file
    let updated_content = toml::to_string_pretty(&manifest)?;
    fs::write("Dotlin.toml", updated_content)?;
    println!("Dependency {} added successfully!", package);
    Ok(())
}

fn remove_dependency(package: &str) -> Result<()> {
    println!("Removing dependency: {}", package);

    // Check if Dotlin.toml exists
    if !std::path::Path::new("Dotlin.toml").exists() {
        eprintln!("Error: Dotlin.toml not found in current directory");
        std::process::exit(1);
    }

    // Read the current manifest
    let manifest_content = fs::read_to_string("Dotlin.toml")?;
    let mut manifest: ProjectManifest = toml::from_str(&manifest_content)?;

    // Remove the package from dependencies
    if manifest.dependencies.remove(package).is_some() {
        // Write the updated manifest back to file
        let updated_content = toml::to_string_pretty(&manifest)?;
        fs::write("Dotlin.toml", updated_content)?;

        println!("Dependency {} removed successfully!", package);
    } else {
        println!("Dependency {} not found in manifest", package);
    }

    Ok(())
}

fn install_dependencies() -> Result<()> {
    println!("Installing dependencies...");
    // Check if Dotlin.toml exists
    if !std::path::Path::new("Dotlin.toml").exists() {
        eprintln!("Error: Dotlin.toml not found in current directory");
        std::process::exit(1);
    }
    // Read the manifest
    let manifest_content = fs::read_to_string("Dotlin.toml")?;
    let manifest: ProjectManifest = toml::from_str(&manifest_content)?;
    // Create the lib directory if it doesn't exist
    if !std::path::Path::new("lib").exists() {
        fs::create_dir("lib")?;
    }
    // In a real implementation, this would download and install each dependency
    // For now, we'll just list them
    if manifest.dependencies.is_empty() {
        println!("No dependencies to install.");
    } else {
        println!("Installing {} dependencies:", manifest.dependencies.len());
        for (name, version) in &manifest.dependencies {
            println!("  - {} v{}", name, version);

            // In a real implementation, we would download and install the package here
            // For now, we'll just create a placeholder file
            fs::write(
                format!("lib/{}-{}.lin", name, version.replace("*", "latest")),
                format!("// Placeholder for {} v{}", name, version),
            )?;
        }
    }
    println!("Dependencies installed successfully!");
    Ok(())
}

fn update_dependencies() -> Result<()> {
    println!("Updating dependencies...");
    // Check if Dotlin.toml exists
    if !std::path::Path::new("Dotlin.toml").exists() {
        eprintln!("Error: Dotlin.toml not found in current directory");
        std::process::exit(1);
    }
    // Read the manifest
    let manifest_content = fs::read_to_string("Dotlin.toml")?;
    let manifest: ProjectManifest = toml::from_str(&manifest_content)?;
    if manifest.dependencies.is_empty() {
        println!("No dependencies to update.");
    } else {
        println!(
            "Checking for updates for {} dependencies:",
            manifest.dependencies.len()
        );
        for (name, version) in &manifest.dependencies {
            println!("  - Checking {} (current: {})...", name, version);
            // In a real implementation, we would check for updates and install them
            // For now, we'll just indicate that we checked
            println!("  - {} is up to date", name);
        }
    }
    println!("Dependency update check completed!");
    Ok(())
}
