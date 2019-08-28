mod backend;

fn main() {
    dotenv::dotenv().unwrap();

    let (discord, receiver) = backend::Discord::spawn(std::env::var("DISCORD_TOKEN").unwrap());

    loop {
        if let Ok(message) = receiver.recv() {
            println!("{:?}", message);
        }
    }
}
