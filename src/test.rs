use crate::tester::{Tester, TraitTester};

use std::path::PathBuf;

#[test]
fn test_ok() {
    let r = Tester::new(PathBuf::from("./test/hello"), Vec::new()).run();
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
    let r = Tester::new(PathBuf::from("./test/"), Vec::new()).run();
    assert!(r.is_err());
    let e = r.unwrap_err();
    assert_eq!(e.msg(), "Permission denied (os error 13)");
}

#[test]
fn test_mem() {
    let r = Tester::new(PathBuf::from("./test/mem"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time < 20);
    assert!(out.memory > 2500 && out.memory < 3000);
}

#[test]
fn test_ret1() {
    let r = Tester::new(PathBuf::from("./test/ret1"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(1));
    assert_eq!(out.signal, None);
    assert!(out.time < 10);
    assert!(out.memory < 1800);
}

#[cfg(unix)]
#[test]
fn test_seg() {
    let r = Tester::new(PathBuf::from("./test/seg"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, None);
    assert_eq!(out.signal, Some("SIGSEGV".into()));
    assert!(out.time < 10);
    assert!(out.memory < 1800);
}

#[cfg(windows)]
#[test]
fn test_seg() {
    let r = Tester::new(PathBuf::from("./test/seg"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert!(out.code.is_some());
    assert_ne!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time < 10);
    assert!(out.memory < 1500);
}

#[test]
fn test_tim() {
    let r = Tester::new(PathBuf::from("./test/tim"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time > 950 && out.time < 1100);
    assert!(out.memory > 1000 && out.memory < 2000);
}
