pub enum ErrorKind {
    SysError(String),
    NixError(nix::Error),
    UnknownError(String),
}

pub struct Error {
    kind: ErrorKind,
}

use ErrorKind::*;

impl Error {
    pub fn sys_error(msg: String) -> Self {
        Self {
            kind: SysError(msg),
        }
    }

    pub fn nix_error(err: nix::Error) -> Self {
        Self {
            kind: NixError(err),
        }
    }

    pub fn unknown_error(msg: String) -> Self {
        Self {
            kind: UnknownError(msg),
        }
    }
}
