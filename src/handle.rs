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