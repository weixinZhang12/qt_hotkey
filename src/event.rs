use std::os::raw::{c_int, c_uchar, c_uint};

use global_hotkey::{GlobalHotKeyEvent, HotKeyState};

use crate::error::Status;
pub struct U8(u8);
impl From<U8> for c_uchar {
    fn from(value: U8) -> Self {
        value.0
    }
}
impl TryFrom<U8> for HotKeyState {
    type Error = Status;

    fn try_from(value: U8) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(HotKeyState::Pressed),
            1 => Ok(HotKeyState::Released),
            _ => Err(Status::Fail),
        }
    }
}

#[repr(C)]
pub struct MyHotKeyEvent {
    id: c_uint,
    state: c_uchar,
}
impl From<GlobalHotKeyEvent> for MyHotKeyEvent {

    fn from(value: GlobalHotKeyEvent) -> Self {
        let state: U8 = value.state.into();
        let state: c_uchar = state.into();
        Self {
            id: value.id,
            state,
        }
    }
}

impl From<HotKeyState> for U8 {
    fn from(value: HotKeyState) -> Self {
        match value {
            HotKeyState::Pressed => U8(0),
            HotKeyState::Released => U8(1),
        }
    }
}
