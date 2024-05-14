use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::{
    mpsc,
    oneshot::{error::RecvError, Sender},
    Mutex,
};

/// Event Notification
#[derive(Debug, Serialize, Deserialize)]
pub struct Notification(pub serde_json::Map<String, serde_json::Value>);

/// Response from the bulb.
pub type Response = Vec<String>;
pub type NotifyChan = Arc<Mutex<Option<mpsc::Sender<Notification>>>>;
pub type RespChan = Arc<Mutex<HashMap<u64, Sender<Result<Response, BulbError>>>>>;

pub struct Reader {
    notify_chan: NotifyChan,
    resp_chan: RespChan,
}

impl Reader {
    pub fn new(resp_chan: RespChan, notify_chan: NotifyChan) -> Self {
        Reader {
            notify_chan,
            resp_chan,
        }
    }

    pub async fn start(self, reader: OwnedReadHalf) -> Result<(), ::std::io::Error> {
        let reader = BufReader::new(reader);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            log::info!("recv <- {}", &line);
            let r: JsonResponse = serde_json::from_slice(&line.into_bytes())?;
            match r {
                JsonResponse::Result { id, result } => {
                    if let Some(sender) = self.resp_chan.lock().await.remove(&id) {
                        if sender.send(Ok(result)).is_err() {
                            log::error!("Could not send result (msg_id={})", id)
                        }
                    }
                }
                JsonResponse::Error {
                    id,
                    error: ErrDetails { code, message },
                } => {
                    if let Some(sender) = self.resp_chan.lock().await.remove(&id) {
                        if sender
                            .send(Err(BulbError::ErrResponse(code, message)))
                            .is_err()
                        {
                            log::error!("Could not send error (msg_id={})", id)
                        }
                    }
                }
                JsonResponse::Notification { params, .. } => {
                    if let Some(sender) = &mut *self.notify_chan.lock().await {
                        if sender.send(Notification(params)).await.is_err() {
                            log::error!("Could not send notification")
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

/// Error Response from the bulb.
#[derive(Debug)]
pub enum BulbError {
    Io(::std::io::Error),
    ErrResponse(i32, String),
    Recv(RecvError),
}

impl Error for BulbError {}

impl fmt::Display for BulbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Recv(e) => e.fmt(f),
            Self::ErrResponse(code, message) => {
                write!(f, "Bulb response error: {} (code {})", message, code)
            }
        }
    }
}

impl From<::std::io::Error> for BulbError {
    fn from(e: ::std::io::Error) -> Self {
        BulbError::Io(e)
    }
}

impl From<RecvError> for BulbError {
    fn from(e: RecvError) -> Self {
        BulbError::Recv(e)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum JsonResponse {
    Result {
        id: u64,
        result: Vec<String>,
    },
    Error {
        id: u64,
        error: ErrDetails,
    },
    Notification {
        method: String,
        params: serde_json::Map<String, serde_json::Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrDetails {
    code: i32,
    message: String,
}
