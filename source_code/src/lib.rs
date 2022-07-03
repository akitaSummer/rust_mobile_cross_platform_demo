use libsm::sm4::{self, cipher::Sm4Cipher, Mode};
use std::str;

pub fn my_sm4(value: String) -> String {
    let key: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let cipher = sm4::Cipher::new(&key, Mode::Cbc).unwrap();
    // let cipher = Sm4Cipher::new(&key).unwrap();

    let data = value.as_bytes();

    let ct = cipher.encrypt(data, &key).unwrap();

    let pt = cipher.decrypt(&ct, &key).unwrap();

    let result = str::from_utf8(&pt).unwrap();

    result.into()
}

#[test]
fn test_fn() {
    let string = String::from("1234");
    let test_result = my_sm4(string);
    assert_eq!(test_result, "1234".to_owned());
}
