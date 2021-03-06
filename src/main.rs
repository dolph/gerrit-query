#[macro_use] extern crate log;
extern crate env_logger;

extern crate argparse;
extern crate users;

struct Options {
    verbose: bool,
    gerrit_review_number: u64,
    gerrit_host: String,
    gerrit_port_number: u16,
    gerrit_username: String,
    identity_file: String,
}

fn main() {
    // Initialize logging.
    env_logger::init().unwrap();

    // Initialize options.
    let mut options = Options {
        verbose: false,
        gerrit_review_number: 1,
        gerrit_host: "".to_string(),
        gerrit_port_number: 29418,
        gerrit_username: "".to_string(),
        identity_file: "~/.ssh/id_rsa".to_string(),
    };

    // Gather information on the current user (for UX).
    let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
    options.gerrit_username = user.name().to_string();
    debug!("Defaulted Gerrit username to UNIX username: {}", options.gerrit_username);

    // Parse command line arguments.
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.set_description(env!("CARGO_PKG_DESCRIPTION"));
        parser.add_option(&["--version"],
            argparse::Print(
                format!(
                    "{} {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"))),
            "Show version information.");
        parser.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], argparse::StoreTrue,
            "Enable verbose output.");
        parser.refer(&mut options.gerrit_host)
            .add_argument("gerrit-host", argparse::Store,
            "Gerrit hostname.");
        parser.refer(&mut options.gerrit_port_number)
            .add_argument("gerrit-port", argparse::Store,
            "Gerrit port number.");
        parser.refer(&mut options.gerrit_username)
            .add_option(&["-u", "--username"], argparse::Store,
            "Gerrit username.");
        parser.refer(&mut options.identity_file)
            .add_option(&["-i", "--identity"], argparse::Store,
            "Path to SSH identity file.");
        parser.refer(&mut options.gerrit_review_number)
            .add_argument("review-number", argparse::Store,
            "Review to archive.");
        parser.parse_args_or_exit();
    }

    info!("Hello, {}.", options.gerrit_username);
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn it_works() {
        main();
    }
}
