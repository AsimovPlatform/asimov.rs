/* automatically generated by rust-bindgen 0.70.1 */

#[doc = " A handle encapsulating SDK state."]
pub type AsiInstance = u64;
#[doc = " An SDK version number as a packed integer."]
pub type AsiVersion = u64;
#[repr(i32)]
#[doc = " The set of possible flow execution states."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum AsiFlowExecutionState {
    ASI_FLOW_EXECUTION_STATE_UNKNOWN = 0,
    ASI_FLOW_EXECUTION_STATE_STARTED = 1,
    ASI_FLOW_EXECUTION_STATE_COMPLETED = 2,
    ASI_FLOW_EXECUTION_STATE_FAILED = 3,
}
#[repr(i32)]
#[doc = " The set of possible port states."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum AsiPortState {
    ASI_PORT_STATE_OPEN = 1,
    ASI_PORT_STATE_CONNECTED = 2,
    ASI_PORT_STATE_CLOSED = 3,
}
#[repr(i32)]
#[doc = " The set of possible port types."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum AsiPortType {
    ASI_PORT_TYPE_INPUT = 1,
    ASI_PORT_TYPE_OUTPUT = 2,
}
#[repr(i32)]
#[doc = " Result codes returned by SDK functions."]
#[must_use]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum AsiResult {
    ASI_SUCCESS = 0,
    ASI_TIMEOUT_EXPIRED = 1,
    ASI_ERROR_NOT_IMPLEMENTED = -1,
    ASI_ERROR_PRECONDITION_VIOLATED = -2,
    ASI_ERROR_HOST_MEMORY_EXHAUSTED = -3,
    ASI_ERROR_DEVICE_MEMORY_EXHAUSTED = -4,
    ASI_ERROR_SIZE_INSUFFICIENT = -5,
}
#[repr(i32)]
#[doc = " The set of possible SDK structure types."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum AsiStructureType {
    ASI_TYPE_UNKNOWN = 0,
}
#[doc = " The common header for all SDK structures."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiStructureHeader {
    pub type_: AsiStructureType,
    pub next: *mut AsiStructureHeader,
}
impl Default for AsiStructureHeader {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A block definition."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiBlockDefinition {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub input_port_count: u32,
    pub output_port_count: u32,
    pub parameter_count: u32,
}
impl Default for AsiBlockDefinition {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiBlockExecuteInfo {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub params: [::core::ffi::c_char; 2048usize],
}
impl Default for AsiBlockExecuteInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A block execution."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiBlockExecution {
    pub header: AsiStructureHeader,
    pub timestamp: u64,
    pub pid: u64,
    pub state: AsiFlowExecutionState,
    pub name: [::core::ffi::c_char; 64usize],
}
impl Default for AsiBlockExecution {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A block parameter."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiBlockParameter {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub default_value: [::core::ffi::c_char; 64usize],
}
impl Default for AsiBlockParameter {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A block port."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiBlockPort {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub type_: AsiPortType,
}
impl Default for AsiBlockPort {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A block usage (in a system or flow)."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiBlockUsage {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub type_: [::core::ffi::c_char; 64usize],
}
impl Default for AsiBlockUsage {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A flow connection."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiFlowConnection {
    pub header: AsiStructureHeader,
    pub source_block: [::core::ffi::c_char; 64usize],
    pub source_port: [::core::ffi::c_char; 64usize],
    pub target_block: [::core::ffi::c_char; 64usize],
    pub target_port: [::core::ffi::c_char; 64usize],
}
impl Default for AsiFlowConnection {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " Defines the flow being created."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiFlowCreateInfo {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
}
impl Default for AsiFlowCreateInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A flow definition."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiFlowDefinition {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub block_count: u32,
}
impl Default for AsiFlowDefinition {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiFlowExecuteInfo {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub params: [::core::ffi::c_char; 2048usize],
}
impl Default for AsiFlowExecuteInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A flow execution."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiFlowExecution {
    pub header: AsiStructureHeader,
    pub timestamp: u64,
    pub pid: u64,
    pub state: AsiFlowExecutionState,
    pub name: [::core::ffi::c_char; 64usize],
}
impl Default for AsiFlowExecution {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiFlowUpdateInfo {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub new_name: [::core::ffi::c_char; 64usize],
}
impl Default for AsiFlowUpdateInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiModelDownloadInfo {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
}
impl Default for AsiModelDownloadInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A model manifest."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiModelManifest {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub size: u64,
}
impl Default for AsiModelManifest {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A model tensor."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiModelTensor {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
}
impl Default for AsiModelTensor {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiModuleEnableInfo {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub enabled: bool,
}
impl Default for AsiModuleEnableInfo {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " A module registration."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AsiModuleRegistration {
    pub header: AsiStructureHeader,
    pub name: [::core::ffi::c_char; 64usize],
    pub block_count: u32,
}
impl Default for AsiModuleRegistration {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[must_use]
    #[doc = " Clones an existing flow definition."]
    pub fn asiCloneFlow(instance: AsiInstance, request: *const AsiFlowUpdateInfo) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Creates a new flow definition."]
    pub fn asiCreateFlow(
        instance: AsiInstance,
        createInfo: *const AsiFlowCreateInfo,
        flow: *mut AsiFlowDefinition,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Creates a new SDK instance."]
    pub fn asiCreateInstance(
        reserved: *const ::core::ffi::c_void,
        instance: *mut AsiInstance,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Destroys an SDK instance."]
    pub fn asiDestroyInstance(instance: AsiInstance) -> AsiResult;
}
extern "C" {
    #[must_use]
    pub fn asiDownloadModel(
        instance: AsiInstance,
        request: *const AsiModelDownloadInfo,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    pub fn asiEnableModule(instance: AsiInstance, request: *const AsiModuleEnableInfo)
        -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates block parameters."]
    pub fn asiEnumerateBlockParameters(
        instance: AsiInstance,
        block: *const AsiBlockDefinition,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiBlockParameter,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates block ports."]
    pub fn asiEnumerateBlockPorts(
        instance: AsiInstance,
        block: *const AsiBlockDefinition,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiBlockPort,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates block definitions."]
    pub fn asiEnumerateBlocks(
        instance: AsiInstance,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiBlockDefinition,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates flow blocks."]
    pub fn asiEnumerateFlowBlocks(
        instance: AsiInstance,
        flow: *const AsiFlowDefinition,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiBlockUsage,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates flow connections."]
    pub fn asiEnumerateFlowConnections(
        instance: AsiInstance,
        flow: *const AsiFlowDefinition,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiFlowConnection,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates flow executions."]
    pub fn asiEnumerateFlowExecutions(
        instance: AsiInstance,
        flow: *const AsiFlowDefinition,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiFlowExecution,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates flow definitions."]
    pub fn asiEnumerateFlows(
        instance: AsiInstance,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiFlowDefinition,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates model tensors."]
    pub fn asiEnumerateModelTensors(
        instance: AsiInstance,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiModelTensor,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates models."]
    pub fn asiEnumerateModels(
        instance: AsiInstance,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiModelManifest,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Enumerates module registrations."]
    pub fn asiEnumerateModules(
        instance: AsiInstance,
        buffer_capacity: u32,
        actual_count: *mut u32,
        buffer: *mut AsiModuleRegistration,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    pub fn asiExecuteBlock(
        instance: AsiInstance,
        request: *const AsiBlockExecuteInfo,
        execution: *mut AsiBlockExecution,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    pub fn asiExecuteFlow(
        instance: AsiInstance,
        request: *const AsiFlowExecuteInfo,
        execution: *mut AsiFlowExecution,
    ) -> AsiResult;
}
extern "C" {
    #[doc = " Returns the SDK build's licensee string, if any."]
    pub fn asiGetLicenseeString() -> *const ::core::ffi::c_char;
}
extern "C" {
    #[doc = " Returns the SDK's current version as a packed integer."]
    pub fn asiGetVersion() -> AsiVersion;
}
extern "C" {
    #[doc = " Returns the SDK's current version as a string."]
    pub fn asiGetVersionString() -> *const ::core::ffi::c_char;
}
extern "C" {
    #[must_use]
    #[doc = " Initializes the SDK's global state."]
    pub fn asiInitLibrary(
        reserved: *const ::core::ffi::c_void,
        print_callback: *mut ::core::ffi::c_void,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    pub fn asiPollFlowExecution(
        instance: AsiInstance,
        execution: *const AsiFlowExecution,
        state: *mut AsiFlowExecutionState,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Removes an existing flow definition."]
    pub fn asiRemoveFlow(instance: AsiInstance, flow: *const AsiFlowDefinition) -> AsiResult;
}
extern "C" {
    #[must_use]
    pub fn asiStartFlowExecution(
        instance: AsiInstance,
        request: *const AsiFlowExecuteInfo,
        execution: *mut AsiFlowExecution,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    pub fn asiStopFlowExecution(
        instance: AsiInstance,
        execution: *const AsiFlowExecution,
    ) -> AsiResult;
}
extern "C" {
    #[must_use]
    #[doc = " Mutates an existing flow definition."]
    pub fn asiUpdateFlow(instance: AsiInstance, request: *const AsiFlowUpdateInfo) -> AsiResult;
}
