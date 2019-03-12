mod error;
mod sys;

#[cfg(test)]
mod test;

use sys::*;

use std::path::{Path, PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tester")]
struct Opt {
    #[structopt(parse(from_os_str))]
    target: PathBuf,
}

pub fn run(path: &Path) -> Result<WaitOutput, String> {
    if !path.is_file() {
        return Err(format!("No such file: {:?}", path));
    }

    exec(&path)
        .map_err(|e| format!("{:?}\n", e))
        .and_then(|pid| wait(pid).map_err(|e| format!("{:?}\n", e)))
}

fn main() {
    let opt = Opt::from_args();

    match run(&opt.target) {
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
        Ok(out) => eprintln!("{}", out),
    }
}
