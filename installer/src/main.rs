fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if let Err(e) = installer::run_with_callback(args.iter().map(|s| s.as_str()), None) {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
