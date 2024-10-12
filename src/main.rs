use crate::client::Client;

mod client;

#[tokio::main]
async fn main() {

    println!("connecting");

    let mut client = Client::connect(String::from("fe8705ba-0ac6-4ebb-bb5c-d6f3798dd2dc"),String::from("dis2sBw_nqkYEFdttYbMVQ")).await;

    println!("listening");

    loop {
        client.get_next_event().await;
    }


}

