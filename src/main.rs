// to disable std library of rust (but it still includes the c runtime library)
#![no_std]

// to disable main entry point
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/*
target triple -> 
os_rust % rustc --version --verbose
rustc 1.65.0 (897e37553 2022-11-02)
binary: rustc
commit-hash: 897e37553bba8b42751c67658967889d11ecd120
commit-date: 2022-11-02
host: x86_64-apple-darwin
release: 1.65.0
LLVM version: 15.0.0
*/

/* 
to build the binary
1) first download the bare metal target thumbv7em-none-eabihf
    rustup target add thumbv7em-none-eabihf
2) then build using the installed target:
    cargo build --target thumbv7em-none-eabihf
*/