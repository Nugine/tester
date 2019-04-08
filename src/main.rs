mod error;
mod opt;
mod output;
mod tester;

#[cfg(test)]
mod test;

use crate::opt::Opt;
use crate::tester::{Tester, TraitTester};

use structopt::StructOpt;

#[inline(always)]
fn err_exit<E: std::fmt::Display>(e: E) -> ! {
    eprintln!("tester: {}", e);
    std::process::exit(1)
}

fn main() {
    let opt = Opt::from_args();

    let json = opt.json;

    use std::io::Write;
    let mut output_file: Box<dyn Write> = match opt.output {
        None => Box::new(std::io::stderr()),
        Some(ref path) => match std::fs::File::create(path) {
            Err(e) => err_exit(e),
            Ok(f) => Box::new(f),
        },
    };

    let tester = Tester::new(opt.target, opt.args);
    match tester.run() {
        Err(err) => {
            if let Err(e) = writeln!(output_file, "tester: {}", err.msg()) {
                err_exit(e)
            }
        }
        Ok(out) => {
            let out_string = if json {
                serde_json::to_string(&out).unwrap()
            } else {
                format!("{}", out)
            };

            if let Err(e) = writeln!(output_file, "{}", out_string) {
                err_exit(e)
            }
        }
    }
}
