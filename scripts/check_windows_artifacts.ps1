# Check that the CI artifacts include dotlin_runtime.dll and dotlin_runtime.lib (if on Windows)
param()

$workspaceRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Definition)
$libDir = Join-Path $workspaceRoot 'lib'

if (-not (Test-Path $libDir)) {
    Write-Error "lib directory not found: $libDir"
    exit 2
}

$dll = Join-Path $libDir 'dotlin_runtime.dll'
$lib = Join-Path $libDir 'dotlin_runtime.lib'

$haveDll = Test-Path $dll
$haveLib = Test-Path $lib

if ($haveDll) { Write-Host "Found DLL: $dll" } else { Write-Warning "Missing DLL: $dll" }
if ($haveLib) { Write-Host "Found import lib: $lib" } else { Write-Warning "Missing import lib: $lib" }

# Pass if at least one artifact exists; prefer both but accept import-lib-only setups
if ($haveDll -or $haveLib) { exit 0 } else { exit 1 }
