use moonstone::lib_main;

#[tokio::main]
async fn main() {
    if lib_main().await.is_ok() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
