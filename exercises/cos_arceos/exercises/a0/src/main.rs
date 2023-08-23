#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

macro_rules! println_prefix {
    ($prefix:expr, $($arg:tt)*) => {
        #[cfg(feature = "axstd")]
        {
            println!("{}{}", $prefix, format_args!($($arg)*));
        }
    };
}

// TODO: Implement macro println_prefix.
#[cfg(feature = "axstd")]


#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, world!");

    let times = 2;
    println_prefix!("Stdout: ", "Hello, world![{}]", times);

    println!("\n[ArceOS Tutorial]: A0 okay!");
}
