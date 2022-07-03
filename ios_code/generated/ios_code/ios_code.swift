public func rust_sm4<GenericIntoRustString: IntoRustString>(_ string: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$rust_sm4({ let rustString = string.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}


