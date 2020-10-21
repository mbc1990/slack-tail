// Represents a message the bot wants to send
#[derive(Debug, Clone)]
pub struct SlackMessageSendTask {
    pub channel_id: String,
    pub message_body: String,  // TODO: Evenetually replace with the serde based model
    pub username: String,
    pub emoji_icon: String
}

unsafe impl Send for SlackMessageSendTask{}
unsafe impl Sync for SlackMessageSendTask{}
