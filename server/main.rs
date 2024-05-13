use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let socket_map: Arc<Mutex<HashMap<(u8,u8,u8,u8,u8,u8), Arc<UdpSocket>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut stdin = tokio::io::stdin();
    loop {
        let mut input = [0; 8];
        let mut stdout = tokio::io::stdout();
        if let Ok(n) = stdin.read_exact(&mut input).await {
            if n == 0 {break} //EOF
        } else {break;}

        let mut tup = (input[0],input[1],input[2],input[3],input[4],input[5]);
        if tup.4 == 0 && tup.5 == 0{//port field = 0
            tup.4 = input[6]; tup.5 = input[7]; //actual port data
            let mut input2 = [0; 1]; //read one byte
            if let Ok(n) = stdin.read_exact(&mut input2).await {
                if n == 0 {
                    break; // EOF
                }else{
                    match input2[0] {
                        0 => {
                            let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
                            socket.connect(SocketAddrV4::new(Ipv4Addr::new(tup.0, tup.1, tup.2, tup.3), u16::from_be_bytes([tup.4, tup.5]))).await.unwrap();
                            eprintln!("[server-info] UDP socket connected from {:?} to {:?}", socket.local_addr().unwrap(), tup);
                            let arcsocket = Arc::new(socket);
                            let arcsocket2 = Arc::clone(&arcsocket);
                            socket_map.lock().await.insert(tup, arcsocket2);
                            tokio::spawn(async move {
                                let mut buf = [0; 1555];
                                loop {
                                    let (bytes_read, _) = arcsocket.recv_from(&mut buf).await.unwrap();
                                    let b = u16::to_be_bytes(bytes_read as u16);
                                    stdout.write_all(&[tup.0, tup.1, tup.2, tup.3, tup.4, tup.5, b[0], b[1]]).await.unwrap();
                                    stdout.write_all(&buf[0..bytes_read]).await.unwrap();
                                    stdout.flush().await.unwrap();
                                }
                            });
                        }
                        _ => {
                            if let Some(_) = socket_map.lock().await.remove(&tup) {
                                eprintln!("[server-info] UDP socket closed: {:?}", tup);
                            } else {
                                eprintln!("[server-warn] tried to remove unknown destination {:?}", tup);
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }else{
            let length = u16::from_be_bytes([input[6], input[7]]);
            let mut data = vec![0; length as usize];
            stdin.read_exact(&mut data).await.expect("[server-error] failed to read data field");
            if let Some(socket) = socket_map.lock().await.get(&tup) {
                socket.send(&data).await.expect("[server-error] failed to send data");
            } else {
                eprintln!("[server-warn] tried to send data to unknown destination {:?}", tup);
            }
        }
    }
}
