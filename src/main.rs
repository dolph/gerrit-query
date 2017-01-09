extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Print};

fn main() {
    let mut verbose = false;

    { // Limit the scope of borrows by the ap.refer() method.
        let mut parser = ArgumentParser::new();
        parser.set_description("A skeleton for Rust-based projects.");
        parser.add_option(&["--version"],
            Print(format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))),
            "Show version information.");
        parser.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Enable verbose output.");
        parser.parse_args_or_exit();
    }

    if verbose {
        println!("Hello, world.");
    }
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn it_works() {
        main();
    }
}
