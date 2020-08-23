# rust_playground
Learning and testing the basics of Rust

## Install
- Rustup (version mgr)--> grab curl from https://www.rust-lang.org/learn/get-started 
- restart terminal and check 'rustup --version', can also check compiler ('rustc --version') and pkg mgr ('cargo --version')

### Initialize project with cargo 
- Option 1 - create new cargo project with folder called project1:
   'cargo new project1'
- Option 2 - create new cargo project within existing folder:
    'cargo init'

## VSCode extensions
- Better TOML
- Rust for Visual Studio Code

## Run

### Compile and run directly with rustc 
- create file with entry point 'main' function e.g. main.rs
- type into terminal:
   'rustc main.rs'  
- this creates an executable in the file which we then have to run:
    './main'

### Run with cargo
- type into terminal:
    'cargo run'

