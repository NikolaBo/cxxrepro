#![no_main]

mod handle;

#[cxx::bridge(namespace = test::ffi)]
mod ffi {
    #[namespace = "handle::ffi"]
    extern "C++" {
        include!("cxxrepro/src/handle.rs.h");

        type Process = crate::handle::Process;
    }

    extern "Rust" {
        fn print_out(process: &Process);
    }
}

fn print_out(process: &handle::Process) {
    println!("process={}", process.raw);
}