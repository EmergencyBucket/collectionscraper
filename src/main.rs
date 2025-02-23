use std::time::SystemTime;

use api::make_bungie_request;

pub mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let now = SystemTime::now();

    println!("Starting at {:?}", now);

    let mut reqs = vec![];

    for i in 0..100 {
        if(i%2==1) {
            reqs.push(make_bungie_request("/Destiny2/3/Profile/4611686018484406952/?components=800"));
        }
        else {
            reqs.push(make_bungie_request("/Destiny2/1/Profile/4611686018473519476/?components=800"));
        }
    }

    let c = trpl::join_all(reqs).await;

    let startparse = SystemTime::now();

    println!("Finished requests at {:?}", startparse);

    //println!("{:?}", c[0].Response.profileCollectibles.data.collectibles.len());

    let now = SystemTime::now();

    let difference = now.duration_since(startparse).unwrap();

    println!("Time taken parsing: {:?}", difference);

    Ok(())
}
