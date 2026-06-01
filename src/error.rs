use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug)]
pub enum Status {
    Fail = -1,
    Success = 0,
}
impl From<Status> for c_int {
    fn from(value: Status) -> Self {
        value as Self
    }
}
