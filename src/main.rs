#![feature(async_closure)]

mod backend;

#[tokio::main]
async fn main() {
    use tokio::sync::mpsc;

    dotenv::dotenv().unwrap();

    let (discord, receiver) = backend::Discord::spawn(std::env::var("DISCORD_TOKEN").unwrap());

    // tokio::spawn(async move {
    //     loop {
    //         if let Err(_) = discord.send_message(
    //             599389401914671104,
    //             "You fool. You absolute buffoon. You think you can challenge me in my own realm? You think you can rebel against my authority? You dare come into my house and upturn my dining chairs, ping me and spill coffee grounds in my Keurig? You thought you were safe in your chain mail armor behind that screen of yours. I will take these <@!599134688052772894>s and destroy you. I didn’t want a ping war, but i didn’t start it.")
    //         {
    //             std::thread::sleep(std::time::Duration::from_secs(5));
    //         } else {
    //             std::thread::sleep(std::time::Duration::from_secs(1));
    //         }
    //     }
    // });

    let (mut download_url_input, mut download_url_output) = mpsc::channel(100);
    let (mut downloaded_images_input, mut downloaded_images_output) = mpsc::channel(100);

    tokio::spawn(async move {
        use futures::compat::{Future01CompatExt, Stream01CompatExt};
        use futures::stream::TryStreamExt;
        use reqwest::r#async::Client;

        let client = Client::new();

        while let Some(url) = download_url_output.recv().await {
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
    });

    loop {
        match receiver.recv() {
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
}
