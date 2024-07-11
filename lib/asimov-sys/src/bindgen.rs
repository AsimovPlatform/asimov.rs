/* automatically generated by rust-bindgen 0.69.4 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiInstance {
    _unused: [u8; 0],
}
pub type AsiInstance_ptr = *mut AsiInstance;
pub type AsiVersion = u64;
pub const AsiResult_ASI_SUCCESS: AsiResult = 0;
pub const AsiResult_ASI_TIMEOUT_EXPIRED: AsiResult = 1;
pub const AsiResult_ASI_ERROR_NOT_IMPLEMENTED: AsiResult = -1;
pub const AsiResult_ASI_ERROR_PRECONDITION_VIOLATED: AsiResult = -2;
pub const AsiResult_ASI_ERROR_HOST_MEMORY_EXHAUSTED: AsiResult = -3;
pub const AsiResult_ASI_ERROR_DEVICE_MEMORY_EXHAUSTED: AsiResult = -4;
pub type AsiResult = ::std::os::raw::c_int;
extern "C" {
    pub fn asiCreateInstance(
        reserved: *const ::std::os::raw::c_void,
        instance: *mut *mut AsiInstance_ptr,
    ) -> AsiResult;
}
extern "C" {
    pub fn asiDestroyInstance(instance: *mut AsiInstance_ptr) -> AsiResult;
}
extern "C" {
    pub fn asiGetLicenseeString() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn asiGetVersion() -> u64;
}
extern "C" {
    pub fn asiGetVersionString() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn asiInitLibrary(
        init_data: *mut ::std::os::raw::c_void,
        print_callback: *mut ::std::os::raw::c_void,
    ) -> AsiResult;
}
