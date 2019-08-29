#![feature(async_closure)]

mod backend;

use backend::{BackendMsg, Discord};
use futures::channel::mpsc::{self, Receiver, Sender};

fn main() {
    dotenv::dotenv().unwrap();

    let (_discord, mut backend_recv, mut url_sender, mut file_recv) = backend();

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
                use futures::sink::SinkExt;

                match message {
                    BackendMsg::MessageAdd(msg) => {
                        println!("{:?}", msg);
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

fn backend() -> (
    Discord,
    Receiver<BackendMsg>,
    Sender<String>,
    Receiver<Vec<u8>>,
) {
    let (discord, backend_recv) = Discord::spawn(std::env::var("DISCORD_TOKEN").unwrap());

    let (url_sender, url_recv) = mpsc::channel(100);
    let (file_sender, file_recv) = mpsc::channel(100);

    std::thread::spawn(|| {
        let runtime = tokio::runtime::Builder::new()
            .name_prefix("backend-")
            .build()
            .unwrap();

        runtime.block_on(async_backend(url_recv, file_sender));
    });

    (discord, backend_recv, url_sender, file_recv)
}

async fn async_backend(mut url_recv: Receiver<String>, file_sender: Sender<Vec<u8>>) {
    use futures::{stream::{TryStreamExt, StreamExt}, compat::{Future01CompatExt, Stream01CompatExt}, sink::SinkExt};
    use std::str::FromStr;
    use hyper::{client::Client, Uri};

    // let https = hyper_tls::HttpsConnector::new(4).unwrap();
    // let client = Client::builder()
    //     .build::<_, hyper::Body>(https);
    let client = Client::new();

    while let Some(url) = url_recv.next().await {
        println!("Received {} to be downloaded", url);
        let client = client.clone();
        let url = Uri::from_str(&url.replace("https://", "http://")).expect("Failed to parse URL");

        let mut file_sender = file_sender.clone();

        tokio::spawn(async move {
            let file = match client.get(url).compat().await {
                Ok(file) => file,
                Err(err) => {
                    println!("Http Error: {:?}", err);
                    return;
                }
            }
            .into_body()
            .compat()
            .try_concat()
            .await;

            println!("Processed file: {:?}", file);
            file_sender
                .send(file.unwrap().to_vec())
                .await
                .expect("Failed to send file");
        });
    }
}
