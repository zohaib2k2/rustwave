use clap::{command, Arg};
use std::path::PathBuf;

pub fn getcmdmatches() -> clap::ArgMatches {
    command!()
        .about("[🦀 🌊] Rustwave is a subdomain enumeration tool!")
        .arg(
            Arg::new("targets")
                .long("targets")
                .short('t')
                .help("path to target domain file 📄.")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("domain")
                .required(true),
        )
        .arg(
            Arg::new("domain")
                .long("domain")
                .short('d')
                .help("one single target domain 🎯."),
        )
        .arg(
            Arg::new("wordlist")
                .long("wordlist")
                .short('w')
                .help("path to subdomain wordlist 📋."),
        )
        .arg(
            Arg::new("nthreads")
                .long("nthreads")
                .short('n')
                .help("Number of threads 🧵.")
                .value_parser(clap::value_parser!(u8))
                .default_value("4"),
        )
        .get_matches()
}
