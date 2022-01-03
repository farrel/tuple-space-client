# tuple-space-client - An HTTP based Async Tuple Space client for Rust 

## What is a Tuple Space

A tuple space is a method for coordinating data between different processes in an asynchronous manner. Processes write tuples of data to the tuple space and then read or remove data from the tuple space using a tuple as template to match against.

This client is based on the [tuple-space](https://github.com/farrel/tuple-space) crate. The corresponding [tuple-space-server](https://github.com/farrel/tuple-space-server) is used to store tuples.

## Usage
```rust
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

    let query_tuple = Tuple::builder()
        .string("Number")
        .any_integer()
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
```

## License (3-Clause BSD License)

Copyright 2021 Farrel Lifson

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
