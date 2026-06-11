use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn pyx_build_writes_pbc_file() {
    let temp_dir = std::env::temp_dir().join(format!(
        "pyx-build-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    let input = temp_dir.join("main.juz");
    let output = temp_dir.join("main.pbc");
    fs::write(&input, "ini main := { 42 }").expect("fixture should be written");

    let status = Command::new(env!("CARGO_BIN_EXE_pyx"))
        .arg("build")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .status()
        .expect("pyx should run");

    assert!(status.success());

    let bytes = fs::read(&output).expect("output should be readable");
    assert_eq!(&bytes[0..3], b"JUZ");
    assert!(bytes.ends_with(&[0x01, 42, 0x63, 0xff]));
}
