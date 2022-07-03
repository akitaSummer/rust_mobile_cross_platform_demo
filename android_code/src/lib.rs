use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use jni::JNIEnv;

use source_code::my_sm4;

#[no_mangle]
pub extern "C" fn rust_sm4_test(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => {
            return CString::new("rust_sm4_test error".to_owned())
                .unwrap()
                .into_raw()
        }
        Ok(string) => string,
    };

    let test_result = my_sm4(recipient.to_owned());

    CString::new("Hello ".to_owned() + &test_result)
        .unwrap()
        .into_raw()
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {

    use super::*;
    use jni::objects::{JClass, JString};
    use jni::sys::jstring;
    use jni::JNIEnv;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_cross_platform_RustSM4_sm4(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        let res = rust_sm4_test(
            env.get_string(java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        );

        let res_ptr = CString::from_raw(res);
        let output = env
            .new_string(res_ptr.to_str().unwrap())
            .expect("Couldn't create java string!");

        output.into_inner()
    }
}

//cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o ./jniLibs build --release
