use tokio::net::UdpSocket;
use tokio::net::udp::SendHalf;
use tokio::net::udp::RecvHalf;
use tokio::task::spawn;

use tokio::sync::mpsc;

use std::net::SocketAddr;
use std::collections::HashMap;

const MULTICAST_ADDR: &str = "239.255.255.250:1982";
const LOCAL_ADDR: &str = "0.0.0.0:1982";

fn parse(buf: &[u8], len: usize) -> Option<(u64, HashMap<String, String>)> {
    let s = ::std::str::from_utf8(&buf[0..len]).ok()?;

    let mut hs = HashMap::new();
    let mut lines = s.split("\r\n");

    let head = lines.next();

    if head != Some("HTTP/1.1 200 OK") {
        panic!("HTTP response NOT OK: {:?}", head)
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
        println!("{}", id);
        return Some((id, hs));
    }

    return None
}

async fn relay(mut recv: RecvHalf, mut send: mpsc::Sender<(u64, HashMap<String, String>)>) -> ! {
    let mut buf = [0; 2048];
    loop {
        if let Ok((len, addr)) = recv.recv_from(&mut buf).await {
            if let Some((id, info)) = parse(&buf, len) {
            }
        }
    }
}

pub async fn find_bulbs() -> Result<mpsc::Receiver<(u64, HashMap<String, String>)>, std::io::Error>{
    let (soc_recv, soc_send) = create_socket().await?.split();
    send_payload(soc_send).await?;
    let (send, recv) = mpsc::channel(10);

    spawn(relay(soc_recv, send));

    Ok(recv)
}

async fn create_socket() -> Result<UdpSocket, std::io::Error> {
    let addr: SocketAddr = LOCAL_ADDR.parse().unwrap();
    UdpSocket::bind(addr).await
}

async fn send_payload(mut socket: SendHalf) -> Result<usize, std::io::Error>{
    let payload = format!(
        "M-SEARCH * HTTP/1.1\r\n
        HOST: {}\r\n
        MAN: \"ssdp:discover\"\r\n
        ST: wifi_bulb\r\n", MULTICAST_ADDR);
    let addr: SocketAddr = MULTICAST_ADDR.parse().unwrap();
    socket.send_to(payload.as_bytes(), &addr).await
}
