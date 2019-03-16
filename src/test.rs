use crate::tester::{Tester, TraitTester};

use std::ffi::OsString;

#[test]
fn test_ok() {
    let r = Tester::new(OsString::from("./test/hello"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time.user < 10);
    assert!(out.memory < 2000);
}

#[test]
fn test_err() {
    let r = Tester::new(OsString::from("./test/"), Vec::new()).run();
    assert!(r.is_err());
    let e = r.unwrap_err();
    assert_eq!(e.msg(), "Permission denied (os error 13)");
}

#[test]
fn test_mem() {
    let r = Tester::new(OsString::from("./test/mem"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time.real < 20);
    assert!(out.memory > 2500 && out.memory < 3000);
}

#[test]
fn test_ret1() {
    let r = Tester::new(OsString::from("./test/ret1"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(1));
    assert_eq!(out.signal, None);
    assert!(out.time.user < 10);
    assert!(out.memory < 1800);
}

#[cfg(unix)]
#[test]
fn test_seg() {
    let r = Tester::new(OsString::from("./test/seg"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, None);
    assert_eq!(out.signal, Some("SIGSEGV".into()));
    assert!(out.time.user < 10);
    assert!(out.memory < 1800);
}

#[test]
fn test_tim() {
    let r = Tester::new(OsString::from("./test/tim"), Vec::new()).run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time.user > 950 && out.time.user < 1150);
    assert!(out.memory > 1000 && out.memory < 2000);
}

#[test]
fn test_ping() {
    let r = Tester::new(
        OsString::from("ping"),
        vec!["-c".into(), "5".into(), "www.baidu.com".into()],
    )
    .run();
    assert!(r.is_ok());
    let out = r.unwrap();
    dbg!(&out);
    assert_eq!(out.code, Some(0));
    assert_eq!(out.signal, None);
    assert!(out.time.real > 3500 && out.time.real < 5500);
    assert!(out.memory > 2500 && out.memory < 3500);
}
