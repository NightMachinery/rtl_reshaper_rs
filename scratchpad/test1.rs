#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release
// target_bin: ./target/release/script
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "script", version = "0.1.0", edition = "2018"}
//     [dependencies]
//     arabic_reshaper = "0.2.0"
// scriptisto-end


fn main() {
    use arabic_reshaper::arabic_reshape;
    let salam = "hello";
    println!("{}",arabic_reshape(salam));
}
