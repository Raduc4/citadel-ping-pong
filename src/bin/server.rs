use citadel_sdk::prefabs::server::empty::EmptyKernel;
use citadel_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let server = NodeBuilder::default()
        .with_node_type(NodeType::server("127.0.0.1:25021").unwrap())
        .build(EmptyKernel::default())
        .unwrap();

    println!("RUNNING on PORT 25021");

    let res = server.await;

    match res {
        Ok(karnel) => {
            let st = karnel.on_start().await.unwrap();
            println!("{:?}", st);
            return ();
        }
        Err(e) => {
            println!("{}", NetworkError::InternalError("error"));
        }
    }
}
