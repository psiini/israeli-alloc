# israeli-alloc
Allocate memory on a random victim program's address space.<br>


### Israel's horrific crimes against the Palestinians are absolutely disgusting and vile. Donate today and help a child in need.
### [🇵🇸 Gaza Crisis Appeal 🇵🇸](https://www.almustafausa.org/appeals/emergency/palestine/?gad_source=1&gad_campaignid=20948761641&gbraid=0AAAAACdpV8WDABG774oT22PQM1N00kIV-&gclid=CjwKCAjwidXQBhAZEiwA4egw6D250nDDIJKFrsO44bk0ovnx6WQMC6VFc_othZaG4b0AdqKe3wZBSBoCKAMQAvD_BwE)
### 

Usage:

```rs

// Optionally, you may opt out of random process picking and use your own `ProcessMemoryInfo`.
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
```

### <b>Formal Warning
This library is a research tool as well as a political statement. It is not recommended you embed this into software that is to be distributed to the public.
<br>

I am <u>NOT</u> responsible for any catastrophic result that results in the use of this.

<b>
