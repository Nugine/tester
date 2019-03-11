mod error;
mod sys;

#[cfg(test)]
mod test;

use std::path::{Path, PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tester")]
struct Opt {
    #[structopt(parse(from_os_str))]
    target: PathBuf,
}

pub fn run(path: &Path) -> Result<(), ()> {
    let pid = sys::exec(&path).map_err(|e| {
        eprintln!("{:?}", e);
    })?;

    let output = sys::wait(pid).map_err(|e| {
        eprintln!("{:?}", e);
    })?;

    eprintln!("code: {}", output.code);

    if let Some(sig) = output.signal {
        eprintln!("signal: {}", sig);
    }

    eprintln!("time: {} ms", output.time);

    if output.memory > 10240 {
        eprintln!("memory: {} MB", output.memory / 1024);
    } else {
        eprintln!("memory: {} KB", output.memory);
    }

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if run(&opt.target).is_err() {
        std::process::exit(1)
    }
}
