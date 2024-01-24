use std::mem::zeroed;
use std::ptr::null_mut;

use windows_sys::Win32::Devices::Display::{
    GetDisplayConfigBufferSizes, QueryDisplayConfig, DISPLAYCONFIG_MODE_INFO,
    DISPLAYCONFIG_PATH_INFO, QDC_ONLY_ACTIVE_PATHS,
};
use windows_sys::Win32::Foundation::ERROR_SUCCESS;

pub struct DisplayConfigs {
    pub paths: Vec<DISPLAYCONFIG_PATH_INFO>,
    pub modes: Vec<DISPLAYCONFIG_MODE_INFO>,
}

impl DisplayConfigs {
    pub fn query() -> Result<Self, &'static str> {
        unsafe {
            let mut path_len = 0;
            let mut mode_len = 0;

            if GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_len, &mut mode_len)
                != ERROR_SUCCESS
            {
                return Err("GetDisplayConfigBufferSizes error");
            }

            let mut configs = DisplayConfigs {
                paths: vec![zeroed(); path_len as usize],
                modes: vec![zeroed(); mode_len as usize],
            };

            if QueryDisplayConfig(
                QDC_ONLY_ACTIVE_PATHS,
                &mut path_len,
                configs.paths.as_mut_ptr(),
                &mut mode_len,
                configs.modes.as_mut_ptr(),
                null_mut(),
            ) != ERROR_SUCCESS
            {
                return Err("QueryDisplayConfig error");
            }

            configs.paths.set_len(path_len as usize);
            configs.modes.set_len(mode_len as usize);

            Ok(configs)
        }
    }
}
