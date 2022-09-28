use moonstone::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    moonstone::run().await
}
