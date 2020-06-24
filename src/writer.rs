use tokio::prelude::*;

use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync;

use tokio::sync::{oneshot::Sender, Mutex};

use crate::reader::Response;

use std::collections::HashMap;
use std::sync::Arc;

pub struct Writer {
    writer : OwnedWriteHalf,
    counter : u64,
    resp_chan: Arc<Mutex<HashMap<u64, Sender<Response>> >>,
    get_response: bool,
}

struct Message(u64, String);

impl Writer {

    pub fn new(writer: OwnedWriteHalf, resp_chan: Arc<Mutex<HashMap<u64, Sender<Response>>>>) -> Self {
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

    pub async fn send(&mut self, method: &str, params: &str) -> Option<Response> {
        let Message(id, content) = self.craft_message(method, params);

        if self.get_response {
            let (sender, receiver) = sync::oneshot::channel();

            self.resp_chan.lock().await.insert(id, sender);

            if let Err(e) = self.send_content(&content).await {
                return Some(
                    Response::Error(1300, format!("IO error: {}", e))
                )
            }

            Some(receiver.await.unwrap_or(
                Response::Error(1200, "no response".to_owned())
            ))
        } else {
            if let Err(e) = self.send_content(&content).await {
                return Some(
                    Response::Error(1300, format!("IO error: {}", e))
                )
            }
            None
        }

    }

    fn craft_message(&mut self, method: &str, params: &str) -> Message {
        let id = self.get_message_id();
        Message(
            id,
            format!(
                r#"{{ "id": {}, "method": "{}", "params": [{} ] }}"#,
                id, method, params
            ) + "\r\n",
        )
    }

    async fn send_content(&mut self, content: &str) -> Result<(), std::io::Error> {
        self.writer.write_all(content.as_bytes()).await
    }

}
