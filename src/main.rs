use api::make_bungie_request;

pub mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = make_bungie_request("/Destiny2/Manifest/").await;
    println!("{res:#?}");

    Ok(())
}
