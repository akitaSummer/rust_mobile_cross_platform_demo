## Rust cross platform demo

### What do i need

If you want to run this project, you must have Android Studi, Xcode and Rust.

then run:
```
rustup target add aarch64-linux-android x86_64-apple-darwin x86_64-pc-windows-msvc x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

###  How to build 
First, you need run

```
sh ./build.sh
```
and then open Xcode, add ios_code/IOSCodePackage to cross_platform project.