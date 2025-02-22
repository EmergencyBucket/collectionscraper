use api::make_request;

pub mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = make_request("https://api64.ipify.org?format=text").await;
    println!("{res:#?}");

    Ok(())
}
