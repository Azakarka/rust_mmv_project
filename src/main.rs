use clap::Parser;
use mmv::run;
use mmv::Arguments;

fn main() {
    let arguments = Arguments::parse();
    match run(arguments) {
        Ok(()) => {}
        Err(error) => {
            println!("{:}", error);
        }
    }
}
