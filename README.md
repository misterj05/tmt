# Twitch Mouse Troll

## What is TMT?

TMT is a Linux application for Twitch streamers that allows viewers to troll the streamer's mouse. When viewers redeem specific channel point rewards, this application makes the mouse move or scroll a random amount.

## What You Need

To use this application, you'll need:
- To be on **Linux**.
- **dotool**: This is a tool that helps the application control mouse and keyboard actions. The reason this is used instead of something like **xdotool** or **ydotool** is because it does not depend on X or Wayland exclusively or need a daemon service running to work.

## Setting Up Custom Rewards

Right now, you need to manually create two custom channel point rewards through the Twitch API:

1. **Mouse Movement Reward**: Set up a reward that triggers mouse movements.
2. **Scroll Reward**: Set up a reward that triggers scrolling actions.

### API Examples
**Example using curl to make the channel point rewards.**
>curl -X POST 'https://api.twitch.tv/helix/channel_points/custom_rewards' \
-H 'Client-ID: YOUR_CLIENT_ID' \
-H 'Authorization: Bearer YOUR_ACCESS_TOKEN' \
-H 'Content-Type: application/json' \
-d '{
    "broadcaster_id": "YOUR_BROADCASTER_ID",
    "title": "Your Reward Title",
    "cost": 5000,
    "prompt": "Your Reward Description",
    "is_enabled": true,
    "is_user_input_required": false
}'

You also need to fetch their **Reward IDs**.
**Example using curl to display all information about your custom rewards.**
>curl -X GET 'https://api.twitch.tv/helix/channel_points/custom_rewards?broadcaster_id=YOUR_BROADCASTER_ID' \
-H 'Client-ID: YOUR_TWITCH_CLIENT_ID' \
-H 'Authorization: Bearer YOUR_ACCESS_TOKEN'


You can find further guidance on how to create and manage these rewards in the [Twitch API documentation](https://dev.twitch.tv/docs/api).
