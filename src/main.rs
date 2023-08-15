extern crate serenity;

use std::io::Read;
use std::fs::File;
use serenity::async_trait;
use serenity::model::prelude::Reaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::Member;
use serenity::model::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, new_member: Member) {
        let welcome_channel_id: u64 = 1136916857765498900; 

        let channel_id = ChannelId(welcome_channel_id);
        if let Err(why) = channel_id.say(&ctx.http, format!("Welcome to the server, {}!", new_member.user.name)).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let message_id: &str = "1134469951848202402";
        let role_id1: u64 = 1134450517544808519;
        let role_id2: u64 = 1134451131288932373; 
        let role_id3: u64 = 1136917527025434706;

        if reaction.message_id.to_string() == message_id{
            let role_id = match reaction.emoji.to_string().as_str() {
                "😩" => role_id1,
                "😁" => role_id2,
                "🤔" => role_id3,
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

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let message_id: &str = "1134469951848202402";
        let role_id1: u64 = 1134450517544808519;
        let role_id2: u64 = 1134451131288932373; 
        let role_id3: u64 = 1136917527025434706;

        if reaction.message_id.to_string() == message_id {
            let role_id = match reaction.emoji.to_string().as_str() {
                "😩" => role_id1,
                "😁" => role_id2,
                "🤔" => role_id3,
                _ => return,
            };
            if let Some(user_id) = reaction.user_id {
                if let Some(guild_id) = reaction.guild_id {
                    if let Ok(mut member) = guild_id.member(&ctx, user_id).await {
                        if let Err(why) = member.remove_role(&ctx, role_id).await {
                            println!("Ошибка удаления роли: {:?}", why);
                        } else {
                            println!("Роль успешно удалена");
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