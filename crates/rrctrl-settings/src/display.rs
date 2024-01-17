use std::mem::{size_of, zeroed};
use std::ptr::null_mut;

use windows_sys::Win32::Devices::Display::{
    DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
    DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
    DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
};
use windows_sys::Win32::Foundation::{ERROR_SUCCESS, LUID};

pub struct Display {
    pub device: String,
    pub monitor: String,
}

impl Display {
    pub fn query() -> Result<Vec<Self>, &'static str> {
        unsafe { query_displays() }
    }
}

unsafe fn query_displays() -> Result<Vec<Display>, &'static str> {
    let mut path_count = 0;
    let mut mode_count = 0;

    if GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_count, &mut mode_count)
        != ERROR_SUCCESS
    {
        return Err("GetDisplayConfigBufferSizes error");
    }

    let mut display_paths: Vec<DISPLAYCONFIG_PATH_INFO> = vec![zeroed(); path_count as usize];
    let mut display_modes: Vec<DISPLAYCONFIG_MODE_INFO> = vec![zeroed(); mode_count as usize];

    if QueryDisplayConfig(
        QDC_ONLY_ACTIVE_PATHS,
        &mut path_count,
        display_paths.as_mut_ptr(),
        &mut mode_count,
        display_modes.as_mut_ptr(),
        null_mut(),
    ) != ERROR_SUCCESS
    {
        return Err("QueryDisplayConfig error");
    }

    display_paths.set_len(path_count as usize);
    display_modes.set_len(mode_count as usize);

    let mut displays = Vec::<Display>::with_capacity(display_paths.len());
    for path in display_paths {
        let device = get_device_name(path.sourceInfo.id, path.sourceInfo.adapterId);
        let monitor = get_monitor_name(path.targetInfo.id, path.targetInfo.adapterId);

        if device.is_ok() && monitor.is_ok() {
            displays.push(Display {
                device: device.unwrap_unchecked(),
                monitor: monitor.unwrap_unchecked(),
            });
        }
    }

    Ok(displays)
}

/// Needs "SOURCE" device
unsafe fn get_device_name(id: u32, adapter_id: LUID) -> Result<String, i32> {
    let info = get_device_info::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>(id, adapter_id)?;
    let mut name = String::from_utf16_lossy(&info.viewGdiDeviceName);
    name.retain(|c| c != '\0');
    Ok(name)
}

/// Needs "TARGET" device
unsafe fn get_monitor_name(id: u32, adapter_id: LUID) -> Result<String, i32> {
    let info = get_device_info::<DISPLAYCONFIG_TARGET_DEVICE_NAME>(id, adapter_id)?;
    let mut name = String::from_utf16_lossy(&info.monitorFriendlyDeviceName);
    name.retain(|c| c != '\0');
    Ok(name)
}

trait DisplayConfigInfo {
    fn header(&mut self) -> &mut DISPLAYCONFIG_DEVICE_INFO_HEADER;
    fn ty() -> i32;
}

impl DisplayConfigInfo for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn header(&mut self) -> &mut DISPLAYCONFIG_DEVICE_INFO_HEADER {
        &mut self.header
    }

    fn ty() -> i32 {
        DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME
    }
}

impl DisplayConfigInfo for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn header(&mut self) -> &mut DISPLAYCONFIG_DEVICE_INFO_HEADER {
        &mut self.header
    }

    fn ty() -> i32 {
        DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME
    }
}

unsafe fn get_device_info<T: DisplayConfigInfo>(id: u32, adapter_id: LUID) -> Result<T, i32> {
    let mut info = {
        let mut info: T = zeroed();
        let header = info.header();
        header.size = size_of::<T>() as u32;
        header.adapterId = adapter_id;
        header.id = id;
        header.r#type = T::ty();
        info
    };

    // Rust pointer madness
    let info_ptr = (&mut info as *mut T).cast::<DISPLAYCONFIG_DEVICE_INFO_HEADER>();

    let error = DisplayConfigGetDeviceInfo(info_ptr);
    if error != ERROR_SUCCESS as i32 {
        return Err(error);
    }

    Ok(info)
}
