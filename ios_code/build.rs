use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use swift_bridge_build::{ApplePlatform, CreatePackageConfig};

fn main() {
    let build_api = env::var("BUILD_API").map(|v| v == "1").unwrap_or(true);

    if build_api {
        swift_bridge_build::create_package(CreatePackageConfig {
            bridge_dir: PathBuf::from("./generated"),
            paths: HashMap::from([
                (
                    ApplePlatform::IOS,
                    "../target/aarch64-apple-ios/debug/libios_code.a".into(),
                ),
                (
                    ApplePlatform::Simulator,
                    "../target/universal-ios/debug/libios_code.a".into(),
                ),
                (
                    ApplePlatform::MacOS,
                    "../target/universal-macos/debug/libios_code.a".into(),
                ),
            ]),
            out_dir: PathBuf::from("IOSCodePackage"),
            package_name: "IOSCodePackage".to_string(),
        });
    } else {
        let out_dir = PathBuf::from("./generated");

        let bridges = vec!["src/lib.rs"];
        for path in &bridges {
            println!("cargo:rerun-if-changed={}", path);
        }

        swift_bridge_build::parse_bridges(bridges)
            .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
    }
}
