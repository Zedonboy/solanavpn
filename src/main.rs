use std::{collections::HashMap, sync::Arc};

use IPV4::IP;
use tokio::{net::UdpSocket, main, net::{TcpListener, unix::SocketAddr}};
use tokio_tun::{TunBuilder, Tun};
use tokio::sync::mpsc;

mod IPV4;

fn setupTUN() -> Tun {
    let tun = TunBuilder::new()
        .name("solana-vpn-tun")
        .tap(false) // false (default): TUN, true: TAP.
        .packet_info(false) // false: IFF_NO_PI, default is true.
        .up() // or set it up manually using `sudo ip link set <tun-name> up`.
        .try_build().expect("Could not implement the TUN device");
        
    return tun;// or `.try_build_mq(queues)` for multi-queue support.
}
async fn main(){
    
    let tun = setupTUN();
    // let arc_tun = Arc::new(tun);
    let route:HashMap<&str, SocketAddr> = HashMap::new();
    let socket = UdpSocket::bind("0.0.0.0:3000").await.expect("Could not bind to Udp port");
    let mut buf = [0u8; 2048];
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);
    let (mut tun_reader, mut tun_writer) = tokio::io::split(arc_tun);
    
    tokio::spawn(async move {
        loop {
            let (len, addr) = socket.recv_from(&mut  buf).await.expect("Error Reading from inbound socket");
            let data = buf[..len];

            let ip_data = IP::marshal(&data);
            let key = format!("{}:{}", ip_data.src, ip_data.dst);
            route.insert(key.as_str(), addr);
            tx.send(data.to_vec())
            
        }
    });

    tokio::spawn(async move {
        while let Some(ip) = rx.recv().await {
            
        }
    });

    Ok(())
}
