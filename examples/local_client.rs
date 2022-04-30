extern crate tokio;
extern crate tuple_space;
extern crate tuple_space_client;

use tuple_space::tuple::Tuple;
use tuple_space_client::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::builder().build("http://localhost:8000").unwrap();

    let tuple = Tuple::builder().string("Number").integer(5).build();

    client.write(&tuple).await.unwrap();
    println!("Wrote: {}", tuple);

    println!("Size: {}", client.size().await.unwrap());

    let query_tuple = Tuple::query().string("Number").any_integer().build();

    let read_tuple = client.read(&query_tuple).await.unwrap().unwrap();
    println!("Read {}", read_tuple);

    println!("Size: {}", client.size().await.unwrap());

    let take_tuple = client.take(&query_tuple).await.unwrap().unwrap();
    println!("Take {}", take_tuple);

    println!("Size: {}", client.size().await.unwrap());

    let no_tuple = client.take(&query_tuple).await.unwrap();
    println!("Take {:?}", no_tuple);
}
