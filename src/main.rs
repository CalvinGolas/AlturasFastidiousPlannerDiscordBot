use std::collections::HashMap;
use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::{Message, Reaction};
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use csv::Reader;
use serde::Deserialize;
use time::PrimitiveDateTime;
use time::macros::format_description;
use serenity::model::prelude::Member as MemberData;
use uuid;
use std::fs::File;
use std::hash::Hash;

#[group]
#[commands(ping, enroll_pc, enroll_gm, prompt_session, cancel_session, debug_list_players)]
struct General;

struct Handler;

#[derive(Debug, Deserialize)]
struct SessionRecord {
    campaign:String,
    date_time:String,
    status:String,
    pole_id:String,
    session_id:String
}

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
    Well I'll be. The session's going to be on this date: {} {}
    at {}:{}. React with a ✅, ❌, or 🤔 as a maybe. Or ya know, don't. I don't care... 😒
    ", time.day(), time.month(), time.hour(), time.minute());

    let session_prompt =  msg.reply(ctx, message_reply).await?;

    // Attach emojis to pushed message
    session_prompt.react(&ctx.http ,'✅').await?;
    session_prompt.react(&ctx.http ,'❌').await?;
    session_prompt.react(&ctx.http ,'🤔').await?;

    // TODO: message await reactions

    Ok(())
}

fn parse_date(unparsed_date: &str) -> PrimitiveDateTime{
    println!("Parsing {}", unparsed_date);
    // MM-dd HH:mm
    let date_format= format_description!("[year] [month]-[day] [hour]:[minute] [second]");
    // convert to primitive date time ready format
    let ready_to_parse_date_string = format!("1985 {} 00", unparsed_date);
    let time = PrimitiveDateTime::parse(
        &ready_to_parse_date_string,
        &date_format);

    time.unwrap()
}

#[command]
async fn cancel_session(ctx: &Context, msg: &Message) -> CommandResult {
    println!("generating session prompt...");
    // splits to input that should be MM-dd HH:mm TZ
    msg.author.name.

    let (_, session_uuid) = msg.content.split_once(' ').unwrap();
    let mut read_file = Reader::from_path("db_stand_in/sessions.csv");

    for result in read_file.deserialize() {
        let session_record: SessionRecord = result;
        if (session_record.session_id == session_uuid && msg.author.name ){


        }
    }

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

        let user_data: Vec<(&String, &String, &u64)> = members.iter()
            .map(|m| ( m.nick.as_ref().unwrap_or(&m.user.name), &m.user.name, m.user.id.as_u64()))
            .collect();

        let mut nickname_id: HashMap<&String, &u64> = HashMap::new();
        let mut name_id: HashMap<&String, &u64> = HashMap::new();
        for (nickname, name, id) in user_data.iter() {
            nickname_id.insert(nickname, id);
            name_id.insert(name,id);
        }
        // TODO: NOT FINISHED GET BACK HERE PLEASE WE ARE ENDING BEFORE YOU FINISH THE THOUGHT
        (nickname_id, name_id)

    }
    Ok(())
}



pub async fn on_reaction_add(ctx: &Context, add_reaction: &Reaction) {
    print!("that's enough monster hunter for now");

}