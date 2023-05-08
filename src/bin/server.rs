use citadel_sdk::prefabs::server::client_connect_listener::ClientConnectListenerKernel;
use citadel_sdk::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let server = NodeBuilder::default()
        .with_node_type(NodeType::server("127.0.0.1:25021").unwrap())
        .build(ClientConnectListenerKernel::new(
            |mut conn, _c2s_remote| async move {
                let (sink, mut stream) = conn.channel.split();

                while let Some(message) = stream.next().await {
                    let ping = String::from_utf8_lossy(&message.into_buffer()).to_string();
                    println!("{:?}", &ping);
                    if (ping == "ping") {
                        sink.send_message("pong".into()).await.unwrap();
                    }
                }
                Ok(())
            },
        ))
        .unwrap();

    println!("RUNNING on PORT 25021");

    let res = server.await;

    // match res {
    //     Ok(karnel) => {
    //         let st = karnel.on_start().await.unwrap();
    //         println!("{:?}", st);
    //         return ();
    //     }
    //     Err(e) => {
    //         println!("{}", NetworkError::InternalError("error"));
    //     }
    // }
}
