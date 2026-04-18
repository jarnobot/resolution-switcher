# Resolution Switcher

Two Windows utilities:

- **ResolutionSwitcher.exe** — settings UI to configure two display profiles (native + custom resolution per monitor)
- **ResolutionToggle.exe** — silent one-click toggle between native and custom; pin this to your taskbar

Config is stored at `%APPDATA%\ResolutionSwitcher\profiles.json`.

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 18+
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on Windows 11)
- Microsoft C++ Build Tools (via Visual Studio Installer)

## Setup

```powershell
npm install
```

## Generate icons (one-time)

Create a 1024×1024 PNG at `app-icon.png`, then:

```powershell
npx tauri icon app-icon.png
```

## Dev mode

```powershell
npm run dev
```

## Build both executables

```powershell
.\build.ps1
```

Or separately:

```powershell
# Toggle binary
cargo build --release -p resolution-toggle

# Settings app
npm run build
```

## Taskbar setup

1. Run `.\build.ps1`
2. Right-click `ResolutionToggle.exe` → **Pin to taskbar**
3. One click toggles between your saved native and custom resolution
