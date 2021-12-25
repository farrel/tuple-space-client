extern crate tokio;
extern crate tuple_space;
extern crate tuple_space_client;

use tuple_space::tuple::Tuple;
use tuple_space_client::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::builder().build("http://localhost:8000").unwrap();

    let tuple = Tuple::builder().add_string("Number").add_integer(5).build();

    client.write(&tuple).await.unwrap();
    println!("Wrote: {}", tuple);

    let query_tuple = Tuple::builder()
        .add_string("Number")
        .add_integer_type()
        .build();

    println!("Query: {}", query_tuple);
    let read_tuple = client.read(&query_tuple).await.unwrap().unwrap();
    println!("Read {}", read_tuple);

    println!("Query: {}", query_tuple);
    let take_tuple = client.take(&query_tuple).await.unwrap().unwrap();
    println!("Take {}", take_tuple);

    println!("Query: {}", query_tuple);
    let no_tuple = client.take(&query_tuple).await.unwrap();
    println!("Take {:?}", no_tuple);
}
