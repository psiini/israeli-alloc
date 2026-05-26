# israeli-alloc
Allocate memory on a random victim programs address space.<br>

```rs
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
```

<b>Warning: Sometimes a process might reject an allocation request, hence a panic. Simply retry the allocation and hope the next one is valid.</b>
