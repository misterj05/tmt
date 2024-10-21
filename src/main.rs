mod mouse;
mod twitch;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let broadcaster_id =
        env::var("TMT_BROADCASTER_ID").expect("TMT_BROADCASTER_ID environment variable not set");
    let mouse_reward_id =
        env::var("TMT_MOUSE_REWARD_ID").expect("TMT_MOUSE_REWARD_ID environment variable not set");
    let scroll_reward_id = env::var("TMT_SCROLL_REWARD_ID")
        .expect("TMT_SCROLL_REWARD_ID environment variable not set");
    let token_str = env::var("TMT_TWITCH_ACCESS_TOKEN")
        .expect("TMT_TWITCH_ACCESS_TOKEN environment variable not set");

    twitch::start_twitch_listener(
        broadcaster_id,
        mouse_reward_id,
        scroll_reward_id,
        &token_str,
    )
    .await;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
