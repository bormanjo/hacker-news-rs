mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hn = client::HNClient::new();
    let item = hn.get_item(37678714).await?;

    println!("{:?}", serde_json::to_string_pretty(&item));

    Ok(())
}
