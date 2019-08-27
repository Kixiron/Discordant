use serenity::{
    http::raw::Http, model::channel::Message, model::gateway::Ready, model::id::ChannelId,
    prelude::*,
};

use std::sync::Arc;
use std::{env, thread, time::Duration};

lazy_static::lazy_static! {
    static ref HTTP: Arc<RwLock<Option<Arc<Http>>>> = Arc::new(RwLock::new(None));
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            println!(
                "{} is connected on shard {}/{}!",
                ready.user.name, shard[0], shard[1],
            );
        }

        *HTTP.write() = Some(ctx.http.clone());
    }

    fn message(&self, ctx: Context, new_message: Message) {
        if let Some(channel) = new_message.channel(ctx.cache) {
            if channel.id() == ChannelId(511257336569135144) {
                println!("{:?}", new_message.content);
            }
        }
    }
}

fn main() {
    dotenv::dotenv().unwrap();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    let manager = client.shard_manager.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(30));

        let lock = manager.lock();
        let shard_runners = lock.runners.lock();

        for (id, runner) in shard_runners.iter() {
            println!(
                "Shard ID {} is {} with a latency of {:?}",
                id, runner.stage, runner.latency,
            );
        }
    });

    thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(5));
        use std::io::prelude::*;

        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            if let Some(ref http) = *HTTP.read() {
                ChannelId(511257336569135144)
                    .send_message(http, |m| {
                        m.content(line);
                        m
                    })
                    .unwrap();
            } else {
                println!("Couldn't get http");
            }
        }
    });

    thread::spawn(|| {
        let mut app = App::new(DataModel { counter: 0 }, AppConfig::default()).unwrap();
        let window = app
            .create_window(WindowCreateOptions::default(), css::native())
            .unwrap();

        app.run(window).unwrap();
    });

    if let Err(why) = client.start_shards(1) {
        println!("Client error: {:?}", why);
    }
}

use azul::{
    prelude::*,
    widgets::{button::Button, label::Label},
};

struct DataModel {
    counter: usize,
}

impl Layout for DataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("Update counter")
            .dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        Dom::new(NodeType::Div).with_child(label).with_child(button)
    }
}

fn update_counter(
    app_state: &mut AppState<DataModel>,
    _event: &mut CallbackInfo<DataModel>,
) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    Redraw
}
