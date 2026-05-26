pub mod alloc;
pub(crate) mod proc;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_void;

    use super::*;
    use crate::alloc::AllocationWrapper;

    #[test]
    fn it_works() {
        let provided_block = match israel_alloc!(5000, None) {
            Some(k) => k,
            None => panic!("Process may need elevated permissions before allocation."),
        };

        let to_write = b"FREE PALESTINE!";
        match destroy_land!(
            provided_block,
            to_write as *const u8 as *const c_void,
            to_write.len()
        ) {
            Ok(_) => {
                let base_address = provided_block.block_ptr;
                let process_id = provided_block.victim_process_id;
                println!(
                    "Memory allocated @{:?} on process-id: {}",
                    base_address, process_id
                )
            }
            Err(err) => panic!("{}", err),
        }
    }
}
