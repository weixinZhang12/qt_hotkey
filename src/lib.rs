use std::os::raw::{c_int, c_uchar, c_uint};

use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey};

use crate::{
    code::{digit_to_modifier, digit_to_mycode},
    error::Status,
    event::MyHotKeyEvent,
};
mod code;
mod error;
mod event;

static mut GLM: *mut GlobalHotKeyManager = std::ptr::null_mut();

pub type CallbackFn = extern "C" fn(event: MyHotKeyEvent);

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
pub unsafe extern "C" fn key_register(modifiers: c_uchar, key: c_uchar, id: *mut c_uint) -> c_int {
    let Some(md) = digit_to_modifier(modifiers) else {
        return Status::Fail.into();
    };
    let Some(glm) = glm_mut() else {
        return Status::Fail.into();
    };
    let Some(code) = digit_to_mycode(key) else {
        return Status::Fail.into();
    };
    let hotkey = HotKey::new(Some(md), code);
    let Ok(_) = glm.register(hotkey) else {
        return Status::Fail.into();
    };
    unsafe { *id = hotkey.id };
    Status::Success.into()
}
///# Safety
/// 该方法传入的函数必须是线程安全的函数
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle(fun: CallbackFn) -> c_int {
    use global_hotkey::GlobalHotKeyEvent;

    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        fun(event.into());
    }
    
    Status::Success.into()
}

///初始化全局GLM，返回值：0->初始化成功 -1初始化失败
/// # Safety
/// 该方法无特殊安全要求,但是在使用GLM前你必须调用该方法，且GLM不是线程安全的。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_glm() -> c_int {
    if unsafe { GLM.is_null() } {
        let Ok(glm) = GlobalHotKeyManager::new() else {
            return -1;
        };
        unsafe { GLM = Box::into_raw(Box::new(glm)) };
        return Status::Success.into();
    }
    Status::Fail.into()
}


