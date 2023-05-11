fn main() { 
  cxx_build::bridges(vec!["src/main.rs"/*, "src/handle.rs"*/]) // returns a cc::Build
            .file("src/test.cc")
            .flag_if_supported("-std=c++11")
            .flag_if_supported("-g")
            .compile("test");
}