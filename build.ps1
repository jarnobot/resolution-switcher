# Builds both executables.
# Run from the project root: .\build.ps1

$ErrorActionPreference = "Stop"

Write-Host "Building ResolutionToggle..." -ForegroundColor Cyan
cargo build --release -p resolution-toggle
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "Building ResolutionSwitcher (Tauri)..." -ForegroundColor Cyan
npm run build
if ($LASTEXITCODE -ne 0) { exit 1 }

$toggleSrc  = "target\release\ResolutionToggle.exe"
$toggleDest = "src-tauri\target\release\bundle\nsis\ResolutionToggle.exe"

if (Test-Path $toggleSrc) {
    Copy-Item $toggleSrc -Destination "." -Force
    Write-Host "ResolutionToggle.exe -> project root" -ForegroundColor Green
}

Write-Host "`nDone. Outputs:" -ForegroundColor Green
Write-Host "  Settings app : src-tauri\target\release\bundle\"
Write-Host "  Toggle       : .\ResolutionToggle.exe  (pin this to taskbar)"
