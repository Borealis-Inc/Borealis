use serenity::async_trait;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is online", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let mut client = Client::builder(&token, GatewayIntents::GUILDS)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    client.start().await.expect("Client error");
}