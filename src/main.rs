use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use time::PrimitiveDateTime;
use time::macros::format_description;
use serenity::model::prelude::Member as MemberData;

#[group]
#[commands(ping, enroll_pc, enroll_gm, prompt_session, cancel_session, debug_list_players)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    println!("pinging...");
    Ok(())
}

#[command]
async fn enroll_pc(ctx: &Context, msg: &Message) -> CommandResult {
    println!("enrolling player...");
    let (_, user_id) = msg.content.split_once(' ').unwrap();

    // TODO: Check if input is already present in db as pc or gm

    // TODO: Add player as Dynamo item

    let mut message_reply = String::from("📜 Enrolled player: ");
    message_reply.push_str(user_id);
    msg.reply(ctx, message_reply).await?;


    Ok(())
}

#[command]
async fn enroll_gm(ctx: &Context, msg: &Message) -> CommandResult {
    println!("enrolling game master...");
    let (_, user_id) = msg.content.split_once(' ').unwrap();

    // TODO: Check if input is already present in db as pc or gm

    // TODO: Add player as Dynamo item

    let mut message_reply = String::from("📜 Enrolled game master: ");
    message_reply.push_str(user_id);
    msg.reply(ctx, message_reply).await?;


    Ok(())
}

#[command]
async fn prompt_session(ctx: &Context, msg: &Message) -> CommandResult {
    println!("generating session prompt...");
    // splits to input that should be MM-dd HH:mm TZ
    let (_, unparsed_time) = msg.content.split_once(' ').unwrap();
    let time = parse_date(unparsed_time);

    let mut message_reply = format!("
    Howdy everyone! This is a reminder that the next session is going to be on the {} of {}
    at {}:{}. Please react with a ✅, ❌, or 🤔 as a maybe.
    ", time.day(), time.month(), time.hour(), time.minute());
    message_reply.push_str(unparsed_time);
    msg.reply(ctx, message_reply).await?;

    // TODO: Attach emojis to pushed message

    Ok(())
}

fn parse_date(unparsed_date: &str) -> PrimitiveDateTime{
    println!("Parsing {}", unparsed_date);
    // MM-dd HH:mm
    let date_format = format_description!("[month]-[day] [hour]:[minute] [second]");

    // TODO: Add error handling
    let time = PrimitiveDateTime::parse(
        unparsed_date,
        &date_format);
    println!("{}", time.unwrap().month());
    time.unwrap()

}

#[command]
async fn cancel_session(ctx: &Context, msg: &Message) -> CommandResult {
    println!("generating session prompt...");
    // splits to input that should be MM-dd HH:mm TZ
    let (_, unparsed_time) = msg.content.split_once(' ').unwrap();


    let mut message_reply = String::from("
    'Sup everyone, no session on the {day} of {month} at {HH:MM} {timezone}.
    ");
    message_reply.push_str(unparsed_time);
    msg.reply(ctx, message_reply).await?;

    Ok(())
}

#[command]
async fn debug_list_players(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(guild_id) = msg.guild_id {
        let members: Vec<MemberData> = guild_id.members(&ctx.http, None, None).await?;

        let user_data: Vec<(&String, &String, &u64)> = members.iter()
            .map(|m| ( m.nick.as_ref().unwrap_or(&m.user.name), &m.user.name, m.user.id.as_u64()))
            .collect();

        let user_data_message: String = user_data.iter()
            .map(|(nickname, name, id)| format!("Nickname: {nickname}, Username: {name}, ID: {id}", nickname = nickname, name = name, id = id))
            .collect::<Vec<String>>()
            .join("\n");
        msg.reply(ctx, user_data_message).await?;
    }
    Ok(())
}

async fn get_guild_name_list(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(guild_id) = msg.guild_id {
        let members: Vec<MemberData> = guild_id.members(&ctx.http, None, None).await?;

        let user_data: Vec<(String, u64)> = members.iter()
            .map(|m| (m.user.name.clone(), *m.user.id.as_u64()))
            .collect();

        let user_data_message: String = user_data.iter()
            .map(|(name, id)| format!("Username: {name}, ID: {id}", name = name, id = id))
            .collect::<Vec<String>>()
            .join("\n");
        msg.reply(ctx, user_data_message).await?;
    }
    Ok(())
}