use std::ptr;
use std::env;
use std::ffi::CString;

#[doc = " @struct pdbg_target"]
#[doc = " @brief PowerPC FSI Debugger target"]
#[doc = ""]
#[doc = " An opaque type handle representing a target in the"]
#[doc = " topology. External programs will generally use a pointer to a"]
#[doc = " pdbg_target to interact with the API."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pdbg_target {
    _unused: [u8; 0],
}

#[doc = " The target has not been probed but will be probed if"]
#[doc = " required."]
pub const PDBG_TARGET_UNKNOWN: PdbgTargetStatus = 0;
#[doc = " The target exists and has been probed, will be released by"]
#[doc = " the release call."]
pub const PDBG_TARGET_ENABLED: PdbgTargetStatus = 1;
#[doc = " The target has not been probed and will never be"]
#[doc = " probed. Application code may use this to prevent"]
#[doc = " probing of certain targets if it knows they are"]
#[doc = " unnecessary."]
pub const PDBG_TARGET_DISABLED: PdbgTargetStatus = 2;
#[doc = " The target has not been probed but an error will be"]
#[doc = " reported if it does not exist as it is required for correct"]
#[doc = " operation. Application code may set this."]
pub const PDBG_TARGET_MUSTEXIST: PdbgTargetStatus = 3;
#[doc = " The target has been probed but did not exist. It will never be"]
#[doc = " reprobed."]
pub const PDBG_TARGET_NONEXISTENT: PdbgTargetStatus = 4;
#[doc = " The target was enabled and has now been released."]
pub const PDBG_TARGET_RELEASED: PdbgTargetStatus = 5;
#[doc = " @brief Describes the current status of a particular target"]
#[doc = " instance."]
#[doc = ""]
#[doc = " Each target has an status associated with it read from the device"]
#[doc = " tree.  If no status property is present in the device tree it"]
#[doc = " defaults to PDBG_TARGET_UNKNOWN. The status of a target affects how"]
#[doc = " it and it's children/parent targets are probed."]
#[doc = ""]
#[doc = " Application code may set some targets to specific status. For"]
#[doc = " example an application may use pdbg_target_status_set() to mark a"]
#[doc = " pdbg_target disabled to avoid probing it. Some status state may"]
#[doc = " only be set by core library code."]
pub type PdbgTargetStatus = ::std::os::raw::c_uint;

extern "C" {
    #[doc = " @brief Initialises the targeting system from the given flattened device tree."]
    #[doc = ""]
    #[doc = " @param [in]  fdt The system device tree pointer, NULL to use default"]
    #[doc = " @return true on success, false on failure"]
    #[doc = ""]
    #[doc = " Must be called prior to using any other libpdbg functions."]
    #[doc = ""]
    #[doc = " Device tree can also be specified using PDBG_DTB environment variable"]
    #[doc = " pointing to system device tree.  If system device tree is specified using"]
    #[doc = " PDBG_DTB, then it will override the default device tree or the specified"]
    #[doc = " device tree."]
    #[doc = ""]
    #[doc = " @note This function can only be called once.  If the call fails, then it"]
    #[doc = " indicates failure to identify the system device tree to load. On failure,"]
    #[doc = " it's possible to call this function again with different argument or after"]
    #[doc = " modifying the value of the environment variable PDBG_DTB.  On success, any"]
    #[doc = " subsequent calls with always return success without re-loading the system"]
    #[doc = " device tree."]
    pub fn pdbg_targets_init(fdt: *const ::std::os::raw::c_void) -> bool;


    #[doc = " @brief Gets the head pdbg_target (root node)"]
    #[doc = " @return pdbg_target the root target"]
    pub fn pdbg_target_root() -> *mut pdbg_target;


    #[doc = " @brief Probe a specific target"]
    #[doc = ""]
    #[doc = " @param[in] target the specific pdbg_target to probe"]
    #[doc = " @return pdbg_target_status after probing the given target"]
    #[doc = ""]
    #[doc = " If the target has already been probed (ie. it has a"]
    #[doc = " pdbg_target_status() == PDBG_TARGET_ENABLED, PDBG_TARGET_DISABLED or"]
    #[doc = " PDBG_TARGET_NONEXISTANT) simply returns the current status of the"]
    #[doc = " target."]
    #[doc = ""]
    #[doc = " If a target has not already been probed (ie. it has a"]
    #[doc = " pdbg_target_status() == PDBG_TARGET_UNKNOWN or"]
    #[doc = " PDBG_TARGET_MUSTEXIST) then probe the given target to determine its"]
    #[doc = " state. This may require probing other targets as it will walk up"]
    #[doc = " the tree looking for the first unprobed parent and then probe every"]
    #[doc = " target in the tree between the given target and this parent to"]
    #[doc = " determine the state of the given target."]
    #[doc = ""]
    #[doc = " @note Currently trying to probe a target with a status of"]
    #[doc = " PDBG_TARGET_RELEASED will result in an assert error. This may"]
    #[doc = " change in future if there is a use case for re-probing targets that"]
    #[doc = " have been released."]
    pub fn pdbg_target_probe(target: *mut pdbg_target) -> PdbgTargetStatus;

    #[doc = " @brief Return the pdbg_target with the given path"]
    #[doc = " @param[in] target the pdbg_target for the root path, typically pdbg_target_root() or NULL for the system root"]
    #[doc = " @param[in] path the path to the desired node"]
    #[doc = " @return The pdbg_target at path or NULL if none found"]
    pub fn pdbg_target_from_path(
        target: *mut pdbg_target,
        path: *const ::std::os::raw::c_char,
    ) -> *mut pdbg_target;

    #[doc = " @brief Read a PIB SCOM register"]
    #[doc = " @param[in] target the pdbg_target"]
    #[doc = " @param[in] addr the address offset relative to target"]
    #[doc = " @param[out] val the read data"]
    #[doc = " @return int 0 if successful, -1 otherwise"]
    pub fn pib_read(target: *mut pdbg_target, addr: u64, val: *mut u64) -> ::std::os::raw::c_int;

    #[doc = " @brief Write a PIB SCOM register"]
    #[doc = " @param[in] target the pdbg_target"]
    #[doc = " @param[in] addr the address offset relative to target"]
    #[doc = " @param[out] val the write data"]
    #[doc = " @return int 0 if successful, -1 otherwise"]
    pub fn pib_write(target: *mut pdbg_target, addr: u64, val: u64) -> ::std::os::raw::c_int;
}

pub const LPC_CMD_REG: u64 = 0x90041;
pub const LPC_DATA_REG: u64 = 0x90042;
pub const LPC_STATUS_REG: u64 = 0x90043;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let addr = match args.pop() {
        Some(val) => val,
        None => panic!("address not specified"),
    };

    if &addr[..2] != "0x" {
        panic!("'{}' must be a hex number", addr);
    }

    let addr = u64::from_str_radix(addr.trim_start_matches("0x"), 16)
            .expect("unable to parse num");

    let r = unsafe { pdbg_targets_init(ptr::null()) };
    if !r {
        panic!("pdbg_targets_init failed");
    }

    let target_path = CString::new("/proc0/pib").expect("Failed to create string");

    let target = unsafe { pdbg_target_from_path(pdbg_target_root(), target_path.as_ptr()) };
    if target.is_null() {
        panic!("pdbg_target_from_path failed");
    }

    let status = unsafe { pdbg_target_probe(target) };
    if status != PDBG_TARGET_ENABLED {
        panic!("pdbg_target_probe: {}", status);
    }

    let r = unsafe { pib_write(target, LPC_CMD_REG, 0x80400000F0000000 | addr) };
    if r != 0 {
        panic!("pib_write(0x90041, 0x80400000F0000000 | addr) failed");
    }

    let mut value: u64 = 0;
    let r = unsafe { pib_read(target, LPC_STATUS_REG, &mut value) } ;
    if r != 0 {
        panic!("pib_read(0x90043) failed");
    }
    if value != 0x8000000000000000 {
        panic!("error: {:x}", value);
    }

    let mut value: u64 = 0;
    let r = unsafe { pib_read(target, LPC_DATA_REG, &mut value) } ;
    if r != 0 {
        panic!("pib_read(0x90042) failed");
    }

    println!("{:x}: {:#018x}", addr, value);
}
