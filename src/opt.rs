use std::ffi::OsString;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tester")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub target: OsString,

    #[structopt(short = "a", long = "arg", parse(from_os_str))]
    pub args: Vec<OsString>,
}
