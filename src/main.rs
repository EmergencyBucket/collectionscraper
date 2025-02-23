use futures::join;

use api::make_bungie_request;

pub mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = make_bungie_request("/Destiny2/3/Profile/4611686018484406952/?components=800");
    let b = make_bungie_request("/Destiny2/3/Profile/4611686018484406952/?components=800");

    let c = join!(a, b);

    println!(
        "{}",
        gjson::get(&c.0, "Response.profileCollectibles.data.collectibles")
    );
    println!(
        "{}",
        gjson::get(&c.1, "Response.profileCollectibles.data.collectibles")
    );

    Ok(())
}
