use futures_util::stream::select_all;
use std::env;
use std::sync::Arc;
use twilight_gateway::{create_recommended, Config, EventTypeFlags, StreamExt};
use twilight_http::Client;
use twilight_model::gateway::event::Event;
use twilight_model::gateway::Intents;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_model::id::Id;
use vesper::prelude::*;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let application_id = env::var("DISCORD_APPLICATION_ID")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let http_client = Arc::new(Client::new(token.clone()));

    let config = Config::new(token.clone(), Intents::empty());
    let shards = create_recommended(&http_client, config, |_, builder| builder.build())
        .await
        .unwrap();

    let mut stream = select_all(shards);

    // Wrap the framework in an Arc, so we can share it across tasks.
    let framework = Arc::new(
        Framework::builder(http_client, Id::new(application_id), ())
            .command(state)
            .build(),
    );

    while let Some(event) = stream.next_event(EventTypeFlags::all()).await {
        match event {
            Err(error) => {
                eprintln!("Gateway failed to receive message, error: {error:?}");
                continue;
            }
            Ok(event) => match event {
                Event::Ready(_) => {
                    // We have to register the commands for them to show in discord.
                    framework.register_global_commands().await.unwrap();
                }
                Event::InteractionCreate(interaction) => {
                    let framework_clone = Arc::clone(&framework);
                    tokio::spawn(async move {
                        framework_clone.process(interaction.0).await;
                    });
                }
                _ => (),
            },
        }
    }
}

#[command]
#[description = "Says where the client is running"]
async fn state(ctx: &SlashContext<()>) -> DefaultCommandResult {
    ctx.interaction_client
        .create_response(
            ctx.interaction.id,
            &ctx.interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    content: Some(String::from(
                        "Running inside a multithreaded tokio runtime!",
                    )),
                    ..Default::default()
                }),
            },
        )
        .await?;

    Ok(())
}
