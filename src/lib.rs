use std::{
    os::raw::{c_int, c_uchar, c_uint},
    sync::{LazyLock, Mutex, mpsc::Receiver},
};

use global_hotkey::{
    GlobalHotKeyManager,
    hotkey::HotKey,
};

use crate::{
    code::{
        digit_to_modifier, digit_to_mycode, enum_modefiers_to_tauri_modefiers,
        mycode_to_global_code,
    },
    error::Status,
};
mod code;
mod error;

static mut GLM: *mut GlobalHotKeyManager = std::ptr::null_mut();

pub type CallbackFn = extern "C" fn();

pub fn read_glm() -> Option<&'static GlobalHotKeyManager> {
    if unsafe { GLM.is_null() } {
        return None;
    }
    Some(unsafe { &*GLM })
}
pub fn glm_mut() -> Option<&'static mut GlobalHotKeyManager> {
    if unsafe { GLM.is_null() } {
        return None;
    }
    Some(unsafe { &mut *GLM })
}
///用于绑定快捷键,返回值：0 -> 成功 1 -> 失败
/// # Safety
/// 你必须保证key的范围在0-214之间（包括），modifiers的范围在 0-13之间（包括）,GLM必须初始化
#[unsafe(no_mangle)]
pub unsafe extern "C" fn register(modifiers: c_uchar, key: c_uchar, result: *mut c_uint) -> c_int {
    let Some(md) = digit_to_modifier(modifiers) else {
        return Status::Error.into();
    };
    let Some(glm) = glm_mut() else {
        return Status::Error.into();
    };
    let md = enum_modefiers_to_tauri_modefiers(md);
    let Some(my_code) = digit_to_mycode(key) else {
        return Status::Error.into();
    };
    let code = mycode_to_global_code(my_code);
    let hotkey = HotKey::new(Some(md), code);
    let Ok(_) = glm.register(hotkey) else {
        return Status::Error.into();
    };
    unsafe { *result = hotkey.id };
    Status::Ok.into()
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn handle(keyid: c_uint) -> c_int {
//     let receiver = GlobalHotKeyEvent::receiver();
// }
///初始化全局GLM，返回值：0->初始化成功 -1初始化失败
/// # Safety
/// 该方法无特殊安全要求,但是在使用GLM前你必须调用该方法，且GLM不是线程安全的。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_glm() -> c_int {
    if unsafe { GLM.is_null() } {
        let Ok(glm) = GlobalHotKeyManager::new() else {
            return -1;
        };
        unsafe { *GLM = glm };
        return Status::Ok.into();
    }
    Status::Error.into()
}
