#![feature(async_closure)]

mod backend;

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async_main());
}

async fn async_main() {
    use tokio::sync::mpsc;

    dotenv::dotenv().unwrap();

    let (_discord, mut receiver) = backend::Discord::spawn(std::env::var("DISCORD_TOKEN").unwrap());

    let (mut download_url_input, mut download_url_output) = mpsc::channel(100);
    let (mut downloaded_images_input, mut downloaded_images_output) = mpsc::channel(100);

    tokio::spawn(async move {
        use reqwest::r#async::Client;

        let client = Client::new();

        loop {
            use futures::compat::{Future01CompatExt, Stream01CompatExt};
            use futures::stream::TryStreamExt;
    
            if let Some(url) = download_url_output.recv().await {
                let file = match client.get(&url).send().compat().await {
                    Ok(file) => file,
                    Err(err) => {
                        if err.is_redirect() {
                            client.get(err.url().unwrap().clone()).send().compat().await.unwrap()
                        } else {
                            println!("{:?}", err);
                            continue;
                        }
                    }
                }.into_body().compat().try_concat().await;
                
                downloaded_images_input.send(file.unwrap().to_vec()).await.unwrap();
            }
        }
    });

    tokio::spawn(async move {
        if let Some(file) = downloaded_images_output.recv().await {
            println!("File: {:?}", file);
        }
    });

    loop {
        use futures::stream::StreamExt;

        if let Some(message) = receiver.next().await {
            use backend::BackendMsg;

            match message {
                BackendMsg::MessageAdd(msg) => {
                    println!("{:?}", msg);
                    for attachment in msg.attachments {
                        download_url_input.send(attachment.url).await.unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
