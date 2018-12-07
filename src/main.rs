use std::io;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue};

fn main(){
    let mut verbose = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Send data into the void");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Enable verbose mode");
        ap.parse_args_or_exit();
    }

    let n = io::copy(&mut io::stdin(), &mut io::sink()).unwrap();
    if verbose {
        eprintln!("{} bytes send into the void", n)
    }
}
