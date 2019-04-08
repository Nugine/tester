mod error;
mod opt;
mod output;
mod tester;

#[cfg(test)]
mod test;

use crate::opt::Opt;
use crate::tester::{Tester, TraitTester};

use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    let tester = Tester::new(opt.target, opt.args);

    match tester.run() {
        Err(err) => {
            eprintln!("tester: {}", err.msg());
            std::process::exit(1);
        }
        Ok(out) => {
            if opt.json {
                eprintln!("{}", serde_json::to_string(&out).unwrap());
            } else {
                eprintln!("{}", out);
            }
        }
    }
}
