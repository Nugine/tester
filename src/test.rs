use crate::run;

use std::path::PathBuf;

#[test]
fn test() {
    let r = run(&PathBuf::from("./test/hello"));
    assert_eq!(r, Ok(()));
}
