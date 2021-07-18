use tokio::net::TcpStream;
use tokio::net::UdpSocket;
use tokio::task::spawn;

use tokio::sync::mpsc;

use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::Bulb;

const MULTICAST_ADDR: &str = "239.255.255.250:1982";
const LOCAL_ADDR: &str = "0.0.0.0:1982";

#[derive(Debug)]
pub struct DiscoveredBulb {
    pub uid: u64,
    pub response_address: SocketAddr,
    pub properties: HashMap<String, String>,
}

impl DiscoveredBulb {
    pub async fn connect(&self) -> Result<Bulb, Box<dyn Error>> {
        let addr = self.properties.get("Location").unwrap();
        let addr = addr.trim_start_matches("yeelight://");

        let stream = TcpStream::connect(addr).await?;

        Ok(Bulb::attach_tokio(stream))
    }
}

/// Returns id and JSON data from Bulb response
fn parse(buf: &[u8], len: usize) -> Option<(u64, HashMap<String, String>)> {
    let s = ::std::str::from_utf8(&buf[0..len]).ok()?;

    let mut hs = HashMap::new();
    let mut lines = s.split("\r\n");

    let head = lines.next();

    if head != Some("HTTP/1.1 200 OK") {
        // TODO: use Result and return Error
        return None;
    }

    for line in lines {
        let mut spl = line.split(": ");
        if let Some(key) = spl.next() {
            if let Some(value) = spl.next() {
                hs.insert(key.to_string(), value.to_string());
            }
        }
    }

    if let Some(id) = hs.get("id") {
        let id = id.trim_start_matches("0x");
        let id = u64::from_str_radix(id, 16).ok()?;
        return Some((id, hs));
    }

    return None;
}

async fn relay(recv: Arc<UdpSocket>, send: mpsc::Sender<DiscoveredBulb>) -> ! {
    let mut buf = [0; 2048];
    loop {
        if let Ok((len, addr)) = recv.recv_from(&mut buf).await {
            if let Some((id, info)) = parse(&buf, len) {
                send.send(DiscoveredBulb {
                    uid: id,
                    response_address: addr,
                    properties: info,
                })
                .await
                .unwrap_or_default();
            }
        }
    }
}

pub async fn find_bulbs() -> Result<mpsc::Receiver<DiscoveredBulb>, std::io::Error> {
    let sock = create_socket().await?;
    let soc_send = Arc::new(sock);
    let soc_recv = soc_send.clone();

    send_payload(soc_send).await?;
    let (send, recv) = mpsc::channel(10);

    spawn(relay(soc_recv, send));

    Ok(recv)
}

async fn create_socket() -> Result<UdpSocket, std::io::Error> {
    let addr: SocketAddr = LOCAL_ADDR.parse().unwrap();
    UdpSocket::bind(addr).await
}

async fn send_payload(socket: Arc<UdpSocket>) -> Result<usize, std::io::Error> {
    let payload = format!(
        "M-SEARCH * HTTP/1.1\r\n
        HOST: {}\r\n
        MAN: \"ssdp:discover\"\r\n
        ST: wifi_bulb\r\n",
        MULTICAST_ADDR
    );
    let addr: SocketAddr = MULTICAST_ADDR.parse().unwrap();
    socket.send_to(payload.as_bytes(), &addr).await
}
