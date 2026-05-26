mod alloc;
mod proc;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_void;

    use crate::proc::ProcessSnapshot;

    use super::*;
    use crate::alloc::AllocationWrapper;

    #[test]
    fn it_works() {
        let block = israel_alloc!(5000);

        let mut data = match block {
            Some(k) => k,
            None => panic!("ERR! Process rejected memory request."),
        };

        let data_to_write = b"FREE PALESTINE!";

        let mut ptr_actual = data.block_ptr;
        unsafe {
            for _k in 0..90 {
                destroy_land!(
                    data,
                    data_to_write.as_ptr() as *const c_void,
                    data_to_write.len()
                );

                ptr_actual = ptr_actual.cast::<u8>().add(1).cast::<c_void>();
                data.block_ptr = ptr_actual;
                println!("{:?}", ptr_actual);
            }
        }

        let _where = data.victim_process_id;
        println!("Wrote to process: {} at: {:?}", _where, data.block_ptr);
    }
}
