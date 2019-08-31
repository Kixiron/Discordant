#![feature(async_closure)]

mod backend;
mod ui;

fn main() {
    dotenv::dotenv().unwrap();

    let (_discord, backend_recv, url_sender, file_recv) = backend::main(
        std::env::var("DISCORD_TOKEN").expect("Missing token env var (DISCORD_TOKEN)"),
    );

    ui::run(backend_recv, url_sender, file_recv);
}
