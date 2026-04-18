#![windows_subsystem = "windows"]

mod jumplist;

use resolution_core::{Config, Resolution, get_current_resolution, set_resolution};
use std::path::PathBuf;

fn main() {
    let mut config = Config::load().unwrap_or_default();
    let current = match get_current_resolution() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (profile_idx, at_native) = detect_profile(&config, &current);
    let profile = &config.profiles[profile_idx];

    if at_native {
        let _ = set_resolution(&profile.custom, profile.hz);
    } else {
        let _ = set_resolution(&profile.native, profile.hz);
    }

    if profile_idx != config.active_idx {
        config.active_idx = profile_idx;
        let _ = config.save();
    }

    if let Some(settings_exe) = find_settings_exe() {
        jumplist::setup(&settings_exe);
    }
}

fn find_settings_exe() -> Option<PathBuf> {
    // Same directory as this exe (portable / side-by-side layout)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("resolution-switcher-app.exe");
            if p.exists() { return Some(p); }
        }
    }
    // NSIS default: %LOCALAPPDATA%\Programs\<identifier>\
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let p = PathBuf::from(&local)
            .join("Programs")
            .join("com.resolution-switcher.app")
            .join("resolution-switcher-app.exe");
        if p.exists() { return Some(p); }
    }
    // MSI / machine-wide: %PROGRAMFILES%\Resolution Switcher\
    if let Ok(pf) = std::env::var("PROGRAMFILES") {
        let p = PathBuf::from(&pf)
            .join("Resolution Switcher")
            .join("resolution-switcher-app.exe");
        if p.exists() { return Some(p); }
    }
    None
}

fn detect_profile(config: &Config, current: &Resolution) -> (usize, bool) {
    for (i, profile) in config.profiles.iter().enumerate() {
        if current == &profile.native { return (i, true); }
        if current == &profile.custom { return (i, false); }
    }
    (config.active_idx.min(1), true)
}
