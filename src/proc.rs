use core::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::mem::size_of;
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};

use crate::err::AllocErr;

pub struct ProcessInfo {
    pub name: String,
    pub id: u32,
}

#[derive(Debug)]
pub struct ProcessSnapshot {}

impl ProcessSnapshot {
    pub fn new() -> Self {
        Self {}
    }

    fn close_handle(&mut self, handle: &HANDLE) {
        unsafe {
            let _ = CloseHandle(*handle);
        }
    }

    fn create_live_snapshot(&self) -> Result<HANDLE, AllocErr> {
        unsafe {
            match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
                Ok(first_handle) => Ok(first_handle),
                Err(_) => Err(AllocErr::ArchitectureMismatch),
            }
        }
    }

    fn remove_name_null_terminators(&self, str: String) -> String {
        String::from(str.trim_matches(char::from(0)))
    }

    pub fn collect_active_processes(&mut self) -> Result<Vec<ProcessInfo>, AllocErr> {
        let snapshot = self.create_live_snapshot()?;
        let mut active_processes = Vec::<ProcessInfo>::new();

        if snapshot != INVALID_HANDLE_VALUE {
            let entry = &mut PROCESSENTRY32W::default();
            entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;

            unsafe {
                match Process32FirstW(snapshot, entry) {
                    Ok(_) => true,

                    /* May seem nonsensical to panic here
                    but we need to keep in mind that the allocator simply cannot function if the first entry
                    cant be resolved.*/
                    Err(_) => {
                        self.close_handle(&snapshot);
                        panic!("{}", AllocErr::InsufficientPermissions)
                    }
                };

                let process_name = String::from_utf16(&entry.szExeFile).unwrap();

                active_processes.push(ProcessInfo {
                    name: self.remove_name_null_terminators(process_name),
                    id: entry.th32ProcessID,
                });

                /* Enumerate and do the rest for the remaining process's */
                loop {
                    match Process32NextW(snapshot, entry) {
                        Ok(_) => {
                            let process_name = String::from_utf16(&entry.szExeFile).unwrap();
                            active_processes.push(ProcessInfo {
                                name: self.remove_name_null_terminators(process_name),
                                id: entry.th32ProcessID,
                            });
                        }

                        /*
                            error handling not needed here because an enumeration failure almost always implies
                            we reached the end of the process list.
                        */
                        Err(_) => break,
                    };
                }
            }

            self.close_handle(&snapshot);
        }

        Ok(active_processes)
    }

    /// Finds the process ID based on the provided process name.
    /// return -1 if the process could not be found.
    pub fn find_id_by_name(&mut self, name: String) -> i32 {
        let p_list = self.collect_active_processes().unwrap();
        let mut process_id: i32 = -1;

        for process in p_list {
            if process.name == name {
                process_id = process.id as i32
            }
        }

        return process_id;
    }

    pub fn find_random_process_id(&mut self) -> u32 {
        let mut process_id = 0;
        let p_list = self.collect_active_processes().unwrap();

        let mut rng = thread_rng();
        if let Some(picked) = p_list.choose(&mut rng) {
            let rand_id = picked.id;
            process_id = rand_id;
        }
        return process_id;
    }
}

impl fmt::Debug for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("\nProcessInfo")
            .field("Process name", &self.name)
            .field("Process ID", &self.id)
            .finish()?;

        write!(f, "")
    }
}
