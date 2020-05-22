use std::fs;

extern crate serenity;
use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

#[group]
#[commands(ping)]
#[commands(ryosuke)]
struct General;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let mut client = Client::new(fs::read_to_string("bot_token").unwrap(), Handler)
        .expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("$$"))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn ryosuke(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "who?")?;
    Ok(())
}
