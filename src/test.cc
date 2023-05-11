#include "cxxrepro/src/handle.rs.h"
#include "cxxrepro/src/main.rs.h"

int main() {
  rust::cxxbridge1::Box<handle::ffi::Process> p = handle::ffi::get_process();
  test::ffi::print_out(*p);

  return 0;
}