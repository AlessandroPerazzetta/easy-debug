# easy-debug &emsp;
[![crates.io](https://img.shields.io/crates/v/debug_print.svg)](https://crates.io/crates/easy_debug)
[![Documentation](https://docs.rs/debug_print/badge.svg)](https://docs.rs/easy_debug)
[![License](https://img.shields.io/badge/license-MIT-0fff0f.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/license-APACHE-0fff0f.svg)](https://www.apache.org/licenses/LICENSE-2.0)
![line](https://github.com/AlessandroPerazzetta/easy-debug)

# A crate to easy debug rust code


## Features

* ENV variable activation
* Debug only, in Release automatically disabled


## Install

```toml
easy-debug = "*"
```

or using cargo-edit
```sh
cargo easy-debug
```

## Usage

```sh
Run command (APP_DEBUG=1) to enable debug: 

APP_DEBUG=1 cargo run
```

### Basic usage
```rust
use easy_debug::{dbg_print, dbg_println, dbg_eprint, dbg_eprintln};

let debug_str_val = "debug string value";

dbg_print!("debug_str_val");
dbg_println!("debug_str_val = {}", debug_str_val);
dbg_eprint!("I'm printing to the Standard Error");
dbg_eprintln!("Print to Standard Error");
```

### Macros names with aliasing 
```rust
use easy_debug::{
    dbg_print as dbg_p,
    dbg_println as dbg_pln,
    dbg_eprint as dbg_ep,
    dbg_eprintln as dbg_epln,
};

let debug_str_val = "debug string value";

dbg_p!("debug_str_val");
dbg_pln!("debug_str_val = {}", debug_str_val);
dbg_ep!("I'm printing to the Standard Error");
dbg_epln!("Print to Standard Error");
```

### Extended macros names with aliasing
```rust
use easy_debug::{
    dbg_xprint as dbg_xp,
    dbg_xprintln as dbg_xpln,
    dbg_xeprint as dbg_xep,
    dbg_xeprintln as dbg_xepln,
};

let debug_str_val = "debug string value";

dbg_xp!("debug_str_val");
dbg_xpln!("debug_str_val = {}", debug_str_val);
dbg_xep!("I'm printing to the Standard Error");
dbg_xepln!("Print to Standard Error");
```

## License
Licensed under either of [Apache License Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT license](https://opensource.org/licenses/MIT) at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.