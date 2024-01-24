use std::mem::{size_of, zeroed};

use windows_sys::Win32::Devices::Display::{
    DisplayConfigGetDeviceInfo, DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_HEADER,
    DISPLAYCONFIG_PATH_SOURCE_INFO, DISPLAYCONFIG_PATH_TARGET_INFO,
    DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME,
};
use windows_sys::Win32::Foundation::{ERROR_SUCCESS, LUID};

pub fn get_device_name(info: DISPLAYCONFIG_PATH_SOURCE_INFO) -> Result<[u16; 32], i32> {
    unsafe {
        let info = get_device_info::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>(info.id, info.adapterId)?;
        Ok(info.viewGdiDeviceName)
    }
}

pub fn get_monitor_name(info: DISPLAYCONFIG_PATH_TARGET_INFO) -> Result<[u16; 64], i32> {
    unsafe {
        let info = get_device_info::<DISPLAYCONFIG_TARGET_DEVICE_NAME>(info.id, info.adapterId)?;
        Ok(info.monitorFriendlyDeviceName)
    }
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
