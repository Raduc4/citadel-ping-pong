use citadel_sdk::prefabs::client::single_connection::SingleClientServerConnectionKernel;
use citadel_sdk::prelude::NodeBuilder;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let client_kernel = SingleClientServerConnectionKernel::new_register_defaults(
        "John Doe",
        "john.doe",
        "password",
        "127.0.0.1:25021",
        |connect_success, remote| async move {
            // handle program logic here
            let (sink, mut stream) = connect_success.channel.split();
            sink.send_message("ping".into()).await.unwrap();
            println!("ping");

            while let Some(message) = stream.next().await {
                println!("{:?}", String::from_utf8_lossy(&message.into_buffer()))
            }

            Ok(())
        },
    )
    .unwrap();

    let client = NodeBuilder::default().build(client_kernel).unwrap();
    let result = client.await;
}
