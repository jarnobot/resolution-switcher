use std::path::Path;

pub fn setup(settings_exe: &Path) {
    #[cfg(windows)]
    unsafe {
        let _ = try_setup(settings_exe);
    }
}

#[cfg(windows)]
unsafe fn try_setup(settings_exe: &Path) -> windows::core::Result<()> {
    use windows::{
        core::{Interface, PCWSTR, PROPVARIANT},
        Win32::{
            Storage::EnhancedStorage::PKEY_Title,
            System::Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize,
                CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
                StructuredStorage::PropVariantClear,
            },
            UI::Shell::{
                Common::{IObjectArray, IObjectCollection},
                DestinationList, EnumerableObjectCollection, ICustomDestinationList,
                IShellLinkW, ShellLink,
                PropertiesSystem::IPropertyStore,
            },
        },
    };

    let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

    // No explicit AUMID — let Windows use the path-based synthetic ID so it
    // matches the AUMID Windows assigned to the pinned taskbar shortcut.
    let dest_list: ICustomDestinationList =
        CoCreateInstance(&DestinationList, None, CLSCTX_INPROC_SERVER)?;

    let mut min_slots: u32 = 0;
    let _removed: IObjectArray = dest_list.BeginList(&mut min_slots)?;

    let shell_link: IShellLinkW =
        CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
    let path_wide: Vec<u16> = settings_exe
        .to_str()
        .unwrap_or("")
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    shell_link.SetPath(PCWSTR(path_wide.as_ptr()))?;
    shell_link.SetIconLocation(PCWSTR(path_wide.as_ptr()), 0)?;

    // Set the Jump List display title via IPropertyStore
    let prop_store: IPropertyStore = shell_link.cast()?;
    let mut pv = PROPVARIANT::from("Open Settings");
    prop_store.SetValue(&PKEY_Title, &pv)?;
    prop_store.Commit()?;
    PropVariantClear(&mut pv)?;

    let coll: IObjectCollection =
        CoCreateInstance(&EnumerableObjectCollection, None, CLSCTX_INPROC_SERVER)?;
    coll.AddObject(&shell_link)?;
    let arr: IObjectArray = coll.cast()?;
    dest_list.AddUserTasks(&arr)?;
    dest_list.CommitList()?;

    CoUninitialize();
    Ok(())
}
