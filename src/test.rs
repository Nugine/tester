use crate::tester::{Tester, TraitTester};

use std::path::PathBuf;

#[test]
fn test_ok() {
    let r = Tester::new(&PathBuf::from("./test/hello")).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time < 10);
    assert!(out.memory < 2000);
}

#[test]
fn test_err() {
    let r = Tester::new(&PathBuf::from("./test/")).run();
    assert!(r.is_err());
    let e = r.unwrap_err();
    assert_eq!(e.msg(), "Permission denied (os error 13)");
}

#[test]
fn test_mem() {
    let r = Tester::new(&PathBuf::from("./test/mem")).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time < 20);
    assert!(out.memory > 9000 && out.memory < 10000);
}

#[test]
fn test_tim() {
    let r = Tester::new(&PathBuf::from("./test/tim")).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time > 950 && out.time < 1100);
    assert!(out.memory > 1000 && out.memory < 2000);
}
