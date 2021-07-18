use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::oneshot::channel;

use crate::reader::{BulbError, RespChan, Response};

use tokio::io::AsyncWriteExt;

pub struct Writer {
    writer: OwnedWriteHalf,
    counter: u64,
    resp_chan: RespChan,
    get_response: bool,
}

struct Message(u64, String);

impl Writer {
    pub fn new(writer: OwnedWriteHalf, resp_chan: RespChan) -> Self {
        Self {
            writer,
            counter: 0,
            resp_chan,
            get_response: true,
        }
    }

    fn get_message_id(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }

    pub fn set_get_response(&mut self, get_response: bool) {
        self.get_response = get_response;
    }

    pub async fn send(
        &mut self,
        method: &str,
        params: &str,
    ) -> Result<Option<Response>, BulbError> {
        let Message(id, content) = self.craft_message(method, params);

        if self.get_response {
            let (sender, receiver) = channel();

            self.resp_chan.lock().await.insert(id, sender);
            self.send_content(&content).await?;

            Ok(Some(receiver.await??))
        } else {
            self.send_content(&content).await?;
            Ok(None)
        }
    }

    fn craft_message(&mut self, method: &str, params: &str) -> Message {
        let id = self.get_message_id();
        Message(
            id,
            format!(
                "{{\"id\":{},\"method\":\"{}\",\"params\":[{}]}}\r\n",
                id, method, params
            )
        )
    }

    async fn send_content(&mut self, content: &str) -> Result<(), ::std::io::Error> {
        self.writer.write_all(content.as_bytes()).await
    }
}
