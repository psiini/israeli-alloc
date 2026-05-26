# israeli-alloc
Allocate memory on a random victim programs address space.<br>

```rs
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
```

<b>Warning: Sometimes a process might reject an allocation request, hence a panic. Simply retry the allocation and hope the next one is valid.</b>
