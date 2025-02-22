use futures::join;

use api::make_bungie_request;

pub mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = make_bungie_request("/Destiny2/Manifest/");
    let b = make_bungie_request("/Destiny2/Manifest/");

    let c = join!(a, b);

    println!("{}", c.0);
    println!("{}", c.1);

    Ok(())
}
