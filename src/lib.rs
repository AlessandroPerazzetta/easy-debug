/*!
## This crate provide prints macros that are not compiled in releases builds.
## Macros are enabled using ENV variable APP_DEBUG

### Basic usage
```
use easy_debug::{dbg_print, dbg_println, dbg_eprint, dbg_eprintln};

let debug_str_val = "debug string value";

dbg_print!("debug_str_val");
dbg_println!("debug_str_val = {}", debug_str_val);
dbg_eprint!("I'm printing to the Standard Error");
dbg_eprintln!("Print to Standard Error");

```

### Macros names with aliasing
```
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
```
use easy_debug::{
    dbg_xprint as dbg_xp,
    dbg_xprintln as dbg_xpln,
    dbg_xeprint as dbg_xep,
    dbg_exprintln as dbg_xepln,
};

let debug_str_val = "debug string value";

dbg_xp!("debug_str_val");
dbg_xpln!("debug_str_val = {}", debug_str_val);
dbg_xep!("I'm printing to the Standard Error");
dbg_xepln!("Print to Standard Error");

```
*/

/// ----------------------------------------------------------------
/// BASIC print, println, eprint, eprintln functions
/// ----------------------------------------------------------------

/// Prints to the standard ouput only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.print.html](https://doc.rust-lang.org/std/macro.print.html) for more info.
#[macro_export]
macro_rules! dbg_print {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  print!("[DEBUG]: {}", format!($($message),*));
        }
    };
}

#[macro_export]
macro_rules! dbg_println {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  println!("[DEBUG]: {}", format!($($message),*));
        }
    };
}


/// Prints to the standard error only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.eprint.html](https://doc.rust-lang.org/std/macro.eprint.html) for more info.
#[macro_export]
macro_rules! dbg_eprint {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  eprint!("[DEBUG_E]: {}", format!($($message),*));
        }
    };
}

#[macro_export]
macro_rules! dbg_eprintln {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  eprintln!("[DEBUG_E]: {}", format!($($message),*));
        }
    };
}

/// ----------------------------------------------------------------
/// EXTENDED print, println, eprint, eprintln functions
/// Print also module name that is calling function
/// ----------------------------------------------------------------

/// Prints to the standard ouput only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.print.html](https://doc.rust-lang.org/std/macro.print.html) for more info.
#[macro_export]
macro_rules! dbg_xprint {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  print!("[DEBUG {}]: {}", module_path!(), format!($($message),*));
        }
    };
}

#[macro_export]
macro_rules! dbg_xprintln {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  println!("[DEBUG {}]: {}", module_path!(), format!($($message),*));
        }
    };
}

/// Prints to the standard error only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.eprint.html](https://doc.rust-lang.org/std/macro.eprint.html) for more info.
#[macro_export]
macro_rules! dbg_xeprint {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  eprint!("[DEBUG_E {}]: {}", module_path!(), format!($($message),*));
        }
    };
}

#[macro_export]
macro_rules! dbg_xeprintln {
    ($($message:expr),*) => {
        if std::env::var("APP_DEBUG").is_ok() {
            #[cfg(debug_assertions)]  eprintln!("[DEBUG_E {}]: {}", module_path!(), format!($($message),*));
        }
    };
}