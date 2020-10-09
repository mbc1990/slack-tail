use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct TailTask {
    pub slack_channel_id: String,
    pub output_tx: Sender<String>
}