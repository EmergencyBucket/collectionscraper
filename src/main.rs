use futures::join;

use api::make_bungie_request;

pub mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = SystemTime::now();

    println!("Starting at {:?}", now);

    let a = make_bungie_request("/Destiny2/3/Profile/4611686018484406952/?components=800");
    let b = make_bungie_request("/Destiny2/3/Profile/4611686018484406952/?components=800");

    let c = join!(a, b);

    let startparse = SystemTime::now();

    println!("Finished requests at {:?}", startparse);

    println!(
        "{}",
        gjson::get(&c.0, "Response.profileCollectibles.data.collectibles")
    );
    println!(
        "{}",
        gjson::get(&c.1, "Response.profileCollectibles.data.collectibles")
    );

    let now = SystemTime::now();

    let difference = now.duration_since(startparse).unwrap();

    println!("Time taken parsing: {:?}", difference);

    Ok(())
}
