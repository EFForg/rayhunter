use std::{env, fs};

fn main() {
    let content = rayhunter_daemon::ApiDocs::generate();
    let mut filename = "openapi.json".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        filename = args[1].to_string();
    }

    fs::write(filename, content).unwrap();
}
