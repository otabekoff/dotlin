# Dotlin Installation Script for Windows
# Usage: iwr -useb https://dotlin.dev/install.ps1 | iex

$ErrorActionPreference = 'Stop'

# Configuration
$DotlinVersion = if ($env:DOTLIN_VERSION) { $env:DOTLIN_VERSION } else { "latest" }
$InstallDir = if ($env:DOTLIN_HOME) { $env:DOTLIN_HOME } else { "$env:USERPROFILE\.dotlin" }
$BinDir = "$InstallDir\bin"
$GitHubRepo = "dotlin-lang/dotlin"

# Helper functions
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
    exit 1
}

# Detect architecture
function Get-Architecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        default { Write-Error-Custom "Unsupported architecture: $arch" }
    }
}

# Download and extract Dotlin
function Install-Dotlin {
    $arch = Get-Architecture
    $platform = "windows-$arch"
    
    Write-Info "Detected platform: $platform"
    Write-Info "Installing Dotlin $DotlinVersion to $InstallDir"
    
    # Create installation directory
    New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
    
    # Determine download URL
    if ($DotlinVersion -eq "latest") {
        $downloadUrl = "https://github.com/$GitHubRepo/releases/latest/download/dotlin-$platform.zip"
    } else {
        $downloadUrl = "https://github.com/$GitHubRepo/releases/download/v$DotlinVersion/dotlin-$platform.zip"
    }
    
    Write-Info "Downloading from: $downloadUrl"
    
    # Download
    $tempFile = [System.IO.Path]::GetTempFileName() + ".zip"
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile -UseBasicParsing
    } catch {
        Write-Error-Custom "Failed to download Dotlin: $_"
    }
    
    # Extract
    Write-Info "Extracting archive..."
    try {
        Expand-Archive -Path $tempFile -DestinationPath $InstallDir -Force
    } catch {
        Write-Error-Custom "Failed to extract archive: $_"
    } finally {
        Remove-Item $tempFile -Force
    }
    
    Write-Info "Dotlin installed successfully!"
}

# Update PATH
function Update-Path {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    # Check if PATH already contains Dotlin
    if ($currentPath -like "*$BinDir*") {
        Write-Info "PATH already contains $BinDir"
        return
    }
    
    # Add to user PATH
    Write-Info "Adding $BinDir to PATH"
    $newPath = "$currentPath;$BinDir"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    
    # Update current session
    $env:Path = "$env:Path;$BinDir"
    
    Write-Warn "PATH updated. You may need to restart your terminal."
}

# Verify installation
function Test-Installation {
    $env:Path = "$env:Path;$BinDir"
    
    $dotcPath = Get-Command dotc.exe -ErrorAction SilentlyContinue
    if ($dotcPath) {
        $version = & dotc.exe --version 2>&1 | Select-Object -First 1
        Write-Info "Verification successful: $version"
        Write-Info "Installed components:"
        Get-ChildItem $BinDir | ForEach-Object { Write-Host "  - $($_.Name)" }
    } else {
        Write-Error-Custom "Installation verification failed. dotc.exe not found in PATH."
    }
}

# Main installation flow
function Main {
    Write-Info "Dotlin Installation Script"
    Write-Info "=========================="
    
    Install-Dotlin
    Update-Path
    Test-Installation
    
    Write-Host ""
    Write-Info "Dotlin has been installed successfully!"
    Write-Info "To get started, run: dotc --help"
    Write-Info "For the REPL, run: dotrepl"
    Write-Host ""
    Write-Info "Documentation: https://dotlin.dev/docs"
    Write-Info "Examples: https://github.com/$GitHubRepo/tree/main/examples"
}

# Run main function
Main
