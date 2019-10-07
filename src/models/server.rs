use ws::{Sender as WsSender};
use crate::models::channel_message::ChannelMessage;

use crossbeam_channel::{Sender, Receiver};

pub struct Server {
    pub out: WsSender,
    pub child_thread_started: bool,
    pub queue_sender: Sender<ChannelMessage>,
    pub queue_receiver: Receiver<ChannelMessage>
}
