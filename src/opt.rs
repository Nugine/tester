use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tester")]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub target: PathBuf,
}
