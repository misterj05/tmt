use std::sync::Arc;
use tokio::time::{sleep, Duration};
use twitch_api::helix::points::{
    CustomRewardRedemptionStatus, GetCustomRewardRedemptionRequest, UpdateRedemptionStatusBody,
    UpdateRedemptionStatusRequest,
};
use twitch_api::helix::HelixClient;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};

use crate::mouse;

pub async fn check_channel_point_redemptions(
    client: &HelixClient<'_, reqwest::Client>,
    broadcaster_id: &str,
    mouse_reward_id: &str,
    scroll_reward_id: &str,
    token: &UserToken,
) {
    let mouse_request = GetCustomRewardRedemptionRequest::broadcaster_id(broadcaster_id)
        .reward_id(mouse_reward_id)
        .status(CustomRewardRedemptionStatus::Unfulfilled);

    match client.req_get(mouse_request, token).await {
        Ok(response) => {
            for redemption in response.data {
                println!(
                    "Mouse Redemption - User: {}, Status: {:?}, Redeemed At: {:?}, User Input: {}",
                    redemption.user_name,
                    redemption.status,
                    redemption.redeemed_at,
                    redemption.user_input
                );

                mouse::move_cursor();

                fulfill_redemption(
                    client,
                    broadcaster_id,
                    &redemption.id.to_string(),
                    mouse_reward_id,
                    token,
                )
                .await;
            }
        }
        Err(e) => {
            eprintln!("Error fetching mouse redemptions: {:?}", e);
        }
    }

    let scroll_request = GetCustomRewardRedemptionRequest::broadcaster_id(broadcaster_id)
        .reward_id(scroll_reward_id)
        .status(CustomRewardRedemptionStatus::Unfulfilled);

    match client.req_get(scroll_request, token).await {
        Ok(response) => {
            for redemption in response.data {
                println!(
                    "Scroll Wheel Redemption - User: {}, Status: {:?}, Redeemed At: {:?}, User Input: {}",
                    redemption.user_name,
                    redemption.status,
                    redemption.redeemed_at,
                    redemption.user_input
                );

                mouse::scroll_wheel();

                fulfill_redemption(
                    client,
                    broadcaster_id,
                    &redemption.id.to_string(),
                    scroll_reward_id,
                    token,
                )
                .await;
            }
        }
        Err(e) => {
            eprintln!("Error fetching scroll redemptions: {:?}", e);
        }
    }
}

pub async fn start_twitch_listener(
    broadcaster_id: String,
    mouse_reward_id: String,
    scroll_reward_id: String,
    token_str: &str,
) {
    let client: HelixClient<reqwest::Client> = HelixClient::default();

    let token = UserToken::from_token(&client, AccessToken::from(token_str))
        .await
        .expect("Failed to create UserToken");

    let broadcaster_id = Arc::new(broadcaster_id);
    let mouse_reward_id = Arc::new(mouse_reward_id);
    let scroll_reward_id = Arc::new(scroll_reward_id);

    tokio::task::spawn(async move {
        loop {
            check_channel_point_redemptions(
                &client,
                &broadcaster_id,
                &mouse_reward_id,
                &scroll_reward_id,
                &token,
            )
            .await;
            sleep(Duration::from_secs(5)).await;
        }
    });
}

async fn fulfill_redemption(
    client: &HelixClient<'_, reqwest::Client>,
    broadcaster_id: &str,
    redemption_id: &str,
    reward_id: &str,
    token: &UserToken,
) {
    let request = UpdateRedemptionStatusRequest::new(broadcaster_id, reward_id, redemption_id);

    let body = UpdateRedemptionStatusBody::status(CustomRewardRedemptionStatus::Fulfilled);

    match client.req_patch(request, body, token).await {
        Ok(..) => {
            println!("Fulfilled redemption.");
        }
        Err(e) => {
            eprintln!("Error fulfilling redemption: {:?}", e);
        }
    }
}
