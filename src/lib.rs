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

        let data = match block {
            Some(k) => k,
            None => panic!("ERR! Process rejected memory request."),
        };

        let data_to_write = ["FREE", "PALESTINE"];

        for _k in 0..90 {
            destroy_land!(data.block_ptr, data_to_write.as_ptr() as *const c_void);
        }

        let _where = data.victim_process_id;
        println!("Wrote to process: {} at: {:?}", _where, data.block_ptr);
    }
}
