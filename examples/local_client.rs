extern crate tokio;
extern crate tuple_space;
extern crate tuple_space_client;

use tuple_space::tuple::Tuple;
use tuple_space_client::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::builder().build("http://localhost:3000").unwrap();

    let tuple = Tuple::builder().add_string("Number").add_integer(5).build();

    client.write_tuple(&tuple).await.unwrap();

    let query_tuple = Tuple::builder()
        .add_string("Number")
        .add_integer_type()
        .build();

    let read_tuple = client.read_tuple(&query_tuple).await.unwrap().unwrap();

    println!("{:?}", read_tuple);

    let take_tuple = client.take_tuple(&query_tuple).await.unwrap().unwrap();
}
