use source_code::my_sm4;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn rust_sm4(string: String) -> String;
    }
}

fn rust_sm4(string: String) -> String {
    my_sm4(string)
}
