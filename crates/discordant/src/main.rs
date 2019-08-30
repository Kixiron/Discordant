#![feature(async_closure)]

mod backend;

fn main() {
    dotenv::dotenv().unwrap();

    let (_discord, mut backend_recv, mut url_sender, mut file_recv) = backend::main();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            use futures::stream::StreamExt;

            tokio::spawn(async move {
                while let Some(file) = file_recv.next().await {
                    println!("File: {:?}", file);
                }
            });

            while let Some(message) = backend_recv.next().await {
                use backend::BackendMsg;
                use futures::sink::SinkExt;

                match message {
                    BackendMsg::MessageAdd(msg) => {
                        for attachment in msg.attachments {
                            println!("Sending {} to be downloaded", attachment.proxy_url);
                            url_sender
                                .send(attachment.url)
                                .await
                                .expect("Failed to send url");
                        }
                    }
                    _ => {}
                }
            }
        });
}
