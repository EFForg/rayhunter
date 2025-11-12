#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(e) = installer::main_cli().await {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
