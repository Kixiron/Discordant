mod backend_message;
mod cache;
mod event_handler;

pub use backend_message::BackendMsg;

use futures::channel::mpsc::{self, Receiver, Sender};
use serenity::prelude::Mutex;
use std::{sync::Arc, thread};

pub fn main() -> (
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

        runtime.block_on(async_main(url_recv, file_sender));
    });

    (discord, backend_recv, url_sender, file_recv)
}

async fn async_main(mut url_recv: Receiver<String>, file_sender: Sender<Vec<u8>>) {
    use futures::{
        sink::SinkExt,
        stream::{StreamExt, TryStreamExt},
    };
    use hyper::{client::Client, Uri};
    use std::str::FromStr;

    let https = hyper_tls::HttpsConnector::new().unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    while let Some(url) = url_recv.next().await {
        let client = client.clone();
        let url = Uri::from_str(&url).expect("Failed to parse URL");

        let mut file_sender = file_sender.clone();

        tokio::spawn(async move {
            let file = match client.get(url).await {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Http Error: {:?}", err);
                    return;
                }
            }
            .into_body()
            .try_concat()
            .await;

            file_sender
                .send(file.unwrap().to_vec())
                .await
                .expect("Failed to send file");
        });
    }
}

pub struct Discord {
    http: Arc<serenity::http::raw::Http>,
    shard_manager: Arc<Mutex<serenity::client::bridge::gateway::ShardManager>>,
    voice_manager: Arc<Mutex<serenity::client::bridge::voice::ClientVoiceManager>>,
}

impl Discord {
    pub fn spawn(token: impl AsRef<str>) -> (Self, Receiver<BackendMsg>) {
        let mut client = serenity::client::Client::new(token, event_handler::Handler)
            .expect("Err creating client");

        let (sender, receiver) = mpsc::channel::<BackendMsg>(100);
        let discord = Self {
            http: Arc::clone(&client.cache_and_http.http),
            shard_manager: Arc::clone(&client.shard_manager),
            voice_manager: Arc::clone(&client.voice_manager),
        };

        {
            let mut data = client.data.write();

            data.insert::<SenderKey>(Arc::new(SendWrap(sender)));
        }

        thread::Builder::new()
            .name("Backend".to_string())
            .spawn(move || {
                if let Err(err) = client.start() {
                    println!("Client error: {:?}", err);
                }
            })
            .expect("Failed to spawn Serenity thread");

        (discord, receiver)
    }

    #[inline]
    pub fn add_group_member(&self, group_id: u64, user_id: u64) -> Result<(), serenity::Error> {
        (*self.http).add_group_recipient(group_id, user_id)
    }

    #[inline]
    pub fn add_role(
        &self,
        guild_id: u64,
        user_id: u64,
        role_id: u64,
    ) -> Result<(), serenity::Error> {
        (*self.http).add_member_role(guild_id, user_id, role_id)
    }

    #[inline]
    pub fn send_message(
        &self,
        channel_id: u64,
        content: &str,
    ) -> Result<serenity::model::channel::Message, serenity::Error> {
        (*self.http).send_message(channel_id, &serde_json::json!({ "content": content }))
    }

    #[inline]
    pub fn restart(&mut self) {
        let mut manager = self.shard_manager.lock();
        for shard in &manager.shards_instantiated() {
            manager.restart(*shard);
        }
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
struct SendWrap(Sender<BackendMsg>);

unsafe impl Send for SendWrap {}
unsafe impl Sync for SendWrap {}

struct SenderKey;
impl serenity::prelude::TypeMapKey for SenderKey {
    type Value = Arc<SendWrap>;
}
