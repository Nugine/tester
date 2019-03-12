use crate::run;

use std::path::PathBuf;

#[test]
fn test_ok() {
    let r = run(&PathBuf::from("./test/hello"));
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, 0);
    assert_eq!(out.signal, None);
    assert!(out.time < 10);
    assert!(out.memory < 2000);
}

#[test]
fn test_err() {
    let r = run(&PathBuf::from("./test/"));
    assert!(r.is_err());
    let e = r.unwrap_err();
    assert_eq!(e, "No such file: \"./test/\"".to_string());
}

#[test]
fn test_mem() {
    let r = run(&PathBuf::from("./test/mem"));
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, 0);
    assert_eq!(out.signal, None);
    assert!(out.time < 20);
    assert!(out.memory > 9000 && out.memory < 10000);
}
