#![feature(async_closure)]

mod backend;
mod ui;

fn main() {
    use tokio::sync::mpsc;

    dotenv::dotenv().unwrap();

    let (discord, discord_receiver) = backend::Discord::spawn(
        std::env::var("DISCORD_TOKEN").expect("Missing token env var (DISCORD_TOKEN)"),
    );

    let (mut download_url_input, mut download_url_output): (_, mpsc::Receiver<String>) =
        mpsc::channel(100);
    let (mut downloaded_images_input, mut downloaded_images_output) = mpsc::channel(100);

    std::thread::spawn(move || {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            use futures::compat::{Future01CompatExt, Stream01CompatExt};
            use futures::stream::TryStreamExt;
            use reqwest::r#async::Client;

            while let Some(url) = download_url_output.recv().await {
                let mut downloaded_images_input = downloaded_images_input.clone();
                tokio::spawn(async move {
                    let client = Client::new();
                    let file = match client.get(&url).send().compat().await {
                        Ok(file) => file,
                        Err(err) => {
                            if err.is_redirect() {
                                client
                                    .get(err.url().unwrap().clone())
                                    .send()
                                    .compat()
                                    .await
                                    .unwrap()
                            } else {
                                println!("{:?}", err);
                                return;
                            }
                        }
                    }
                    .into_body()
                    .compat()
                    .try_concat()
                    .await;
                    if let Some(decoded) = ui::decode_webp(&file.unwrap().to_vec()[..]) {
                        downloaded_images_input.send(decoded).await.unwrap();
                    }
                });
            }
        });
    });
    ui::run(
        discord_receiver,
        download_url_input,
        downloaded_images_output,
    );
    /*
    loop {
        match discord_receiver.recv() {
            Ok(message) => {
                use backend::BackendMsg;

                match message {
                    BackendMsg::MessageAdd(msg) => {
                        println!("{:?}", msg);
                        for attachment in msg.attachments {
                            download_url_input.send(attachment.url).await.unwrap();
                        }
                    }
                    msg => println!("{:?}", msg),
                }
            }
            Err(err) => println!("{:?}", err),
        }

        if let Some(file) = downloaded_images_output.recv().await {
            println!("File: {:?}", file);
        }
    }
    */
}
