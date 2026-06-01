use std::os::raw::c_int;

#[repr(C)]
pub enum Status {
    Error = -1,
    Ok = 0,
}
impl From<Status> for c_int {
    fn from(value: Status) -> Self {
        value as Self
    }
}
