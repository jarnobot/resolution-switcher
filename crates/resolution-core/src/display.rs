use crate::config::Resolution;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DisplayError {
    #[error("Failed to read current display settings")]
    EnumFailed,
    #[error("Resolution change failed with code {0}")]
    ChangeFailed(i32),
}

#[cfg(windows)]
pub fn get_current_resolution() -> Result<Resolution, DisplayError> {
    use std::mem;
    use windows::Win32::Graphics::Gdi::{
        EnumDisplaySettingsExW, DEVMODEW, ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_FLAGS,
    };

    let mut dm: DEVMODEW = unsafe { mem::zeroed() };
    dm.dmSize = mem::size_of::<DEVMODEW>() as u16;

    let ok = unsafe {
        EnumDisplaySettingsExW(None, ENUM_CURRENT_SETTINGS, &mut dm, ENUM_DISPLAY_SETTINGS_FLAGS(0))
    };

    if ok.as_bool() {
        Ok(Resolution {
            w: dm.dmPelsWidth,
            h: dm.dmPelsHeight,
        })
    } else {
        Err(DisplayError::EnumFailed)
    }
}

#[cfg(windows)]
pub fn set_resolution(res: &Resolution, hz: u32) -> Result<(), DisplayError> {
    use std::mem;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Gdi::{
        ChangeDisplaySettingsExW, CDS_UPDATEREGISTRY, DEVMODE_FIELD_FLAGS, DEVMODEW,
        DISP_CHANGE_SUCCESSFUL,
    };

    // DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY
    const DM_DISPLAY_FLAGS: u32 = 0x00080000 | 0x00100000 | 0x00400000;

    let mut dm: DEVMODEW = unsafe { mem::zeroed() };
    dm.dmSize = mem::size_of::<DEVMODEW>() as u16;
    dm.dmFields = DEVMODE_FIELD_FLAGS(DM_DISPLAY_FLAGS);
    dm.dmPelsWidth = res.w;
    dm.dmPelsHeight = res.h;
    dm.dmDisplayFrequency = hz;

    let result = unsafe {
        ChangeDisplaySettingsExW(None, Some(&dm), HWND::default(), CDS_UPDATEREGISTRY, None)
    };

    if result == DISP_CHANGE_SUCCESSFUL {
        Ok(())
    } else {
        Err(DisplayError::ChangeFailed(result.0))
    }
}

#[cfg(not(windows))]
pub fn get_current_resolution() -> Result<Resolution, DisplayError> {
    Err(DisplayError::EnumFailed)
}

#[cfg(not(windows))]
pub fn set_resolution(_res: &Resolution, _hz: u32) -> Result<(), DisplayError> {
    Err(DisplayError::ChangeFailed(-1))
}
