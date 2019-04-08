use std::ffi::OsString;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tester")]
pub struct Opt {
    #[structopt(parse(from_os_str), help = "command to run")]
    pub target: OsString,

    #[structopt(short = "j", long = "json", help = "Json output")]
    pub json: bool,

    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        help = "output file path (default stderr)"
    )]
    pub output: Option<OsString>,

    #[structopt(last = true, parse(from_os_str), help = "arguments to be passed")]
    pub args: Vec<OsString>,
}
