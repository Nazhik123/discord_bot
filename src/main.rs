extern crate serenity;

use std::io::Read;
use std::fs::File;
use serenity::async_trait;
use serenity::model::prelude::Reaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let message_id: &str = "1134469951848202402";
        let role_id1: u64 = 1134450517544808519; 
        let role_id2: u64 = 1134451131288932373; 

        if reaction.message_id.to_string() == message_id{
            let role_id = match reaction.emoji.to_string().as_str() {
                "😩" => role_id1,
                "😁" => role_id2,
                _ => {
                    println!("Неизвестная реакция");
                    return;
                }
            };
            if let Some(user_id) = reaction.user_id {
                if let Some(guild_id) = reaction.guild_id {
                    if let Ok(mut member) = guild_id.member(&ctx, user_id).await {
                        if let Err(why) = member.add_role(&ctx, role_id).await {
                            println!("Ошибка при выдаче роли: {:?}", why);
                        } else {
                            println!("Роль успешно выдана");
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut file = File::open(".token").unwrap();
    let mut token: String = String::new();
    file.read_to_string(&mut token).expect("Error");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}