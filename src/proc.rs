use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Display;
use std::mem::size_of;
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};

#[derive(Debug)]
pub(crate) struct ProcessInfo {
    pub name: [u16; 260],
    pub id: u32,
}

#[derive(Debug)]
pub struct ProcessSnapshot {
    handle: HANDLE,
    collected_processes: Vec<ProcessInfo>,
}

impl ProcessSnapshot {
    pub fn new() -> Self {
        unsafe {
            let process_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

            Self {
                handle: process_handle.unwrap(),
                collected_processes: Vec::<ProcessInfo>::new(),
            }
        }
    }

    fn has_first(&self, entry: &mut PROCESSENTRY32W) -> bool {
        let snapshot = self.handle;
        unsafe {
            match Process32FirstW(snapshot, entry) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }

    fn has_next(&mut self, entry: &mut PROCESSENTRY32W) -> bool {
        let snapshot = self.handle;
        unsafe {
            match Process32NextW(snapshot, entry) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }

    pub fn find_random_process_id(&mut self) -> u32 {
        let mut process_id: u32 = 0;
        let mut process_entry: PROCESSENTRY32W = PROCESSENTRY32W::default();

        let snapshot: HANDLE = self.handle;

        if snapshot != INVALID_HANDLE_VALUE {
            process_entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;

            let entry_ref: &mut PROCESSENTRY32W = &mut process_entry;
            let valid_first = self.has_first(entry_ref);

            if valid_first {
                loop {
                    process_id = entry_ref.th32ProcessID;

                    let exe_name = entry_ref.szExeFile;

                    if process_id != 0 && process_id != 4 {
                        self.collected_processes.push(ProcessInfo {
                            name: exe_name,
                            id: process_id,
                        });
                    }

                    let has_next = self.has_next(entry_ref);
                    if !has_next {
                        break;
                    }
                }
            }
        }

        let mut rng = thread_rng();
        if let Some(picked) = self.collected_processes.choose(&mut rng) {
            let rand_id = picked.id;
            process_id = rand_id;
        }

        return process_id;
    }
}

impl Drop for ProcessSnapshot {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}
