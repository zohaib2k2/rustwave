use std::path::PathBuf;

pub mod cmdargs;
pub mod subwave;

#[tokio::main]
async fn main() {
    let mut target_domains: Vec<String> = Vec::new();
    let mut wordlists: Vec<String> = Vec::new();

    let cmd_matches = cmdargs::getcmdmatches();

    if cmd_matches.clone().try_contains_id("targets").unwrap() {
        let target_file = cmd_matches.get_one::<PathBuf>("targets").unwrap();
        let target_file = target_file.to_str().unwrap();
        for line in rustwave::read_to_lines(target_file).unwrap() {
            target_domains.push(line.trim().to_string());
        }
    } else {
        let target = cmd_matches.get_one::<String>("domain").unwrap();
        target_domains.push(target.to_string());
    }

    let wordlists_path = cmd_matches.get_one::<String>("wordlist").unwrap();

    for line in rustwave::read_to_lines(wordlists_path).unwrap() {
        wordlists.push(line);
    }

    let nt = cmd_matches.get_one::<u8>("nthreads").unwrap();

    let task_enum = subwave::Wave::new(
        target_domains,
        wordlists,
        subwave::Range { start: 0, end: 10 },
    );
    task_enum.execute(*nt).await;
}
