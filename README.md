# ggpo-rs

Rust bindings for GGPO, generated by bindgen. Unfinished

## Getting Started
Install prerequisites and clone repo. Tests are in `src/lib.rs` and
can be run with `cargo test`. 

For a top-level view of all bindings generated, use `cargo doc` and travel to the `target/doc` directory. 

### Prerequisites
- All requirements for [GGPO](https://github.com/pond3r/ggpo) and [rust-bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html) must be installed.  
- Windows is the only platform supported by GGPO currently. 

## Project Status

### Completed tasks   
- Dependencies identified and linked 
- Bindings generated by bindgen

### TODO 
- Create 'sanity tests' for bindings
- Create safe interface for bindings 

### Potential Issues? 
- Proper build system for actually compiling GGPO instead of just including the .lib file in the repository 
- Properly serializing/deserializing data from Rust to GGPO and vice versa 