#![no_main]

mod handle {
    pub struct Process {
        pub raw: u32,
      }
      
      use cxx::{type_id, ExternType};
      
      unsafe impl ExternType for Process {
        type Id = type_id!("handle::ffi::Process");
        type Kind = cxx::kind::Opaque;
      }
      
      #[cxx::bridge(namespace = handle::ffi)]
      pub mod ffi {
        extern "Rust" {
            type Process;
            fn get_process() -> Box<Process>;
        }
      }
      
      fn get_process() -> Box<Process> {
        Box::new(Process { raw: 3 })
      }
}

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