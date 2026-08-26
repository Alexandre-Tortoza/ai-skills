#![forbid(unsafe_code)]

use std::env;

const HELP: &str = "ai-skills\n\nUsage:\n  ai-skills --help\n\nThe command surface will be introduced by subsequent foundation issues.";

fn main() {
    if env::args_os().nth(1).as_deref() == Some("--help".as_ref()) {
        println!("{HELP}");
        return;
    }

    println!("{HELP}");
}
