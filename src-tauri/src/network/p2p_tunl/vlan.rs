use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::{TcpListener, TcpSocket},
  sync::mpsc::{Receiver, Sender}, select,
};

/// The part connecting to the game client
pub async fn listen(port: u16, mut rx: Receiver<Vec<u8>>, tx: Sender<Vec<u8>>) {
  let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
    .await
    .expect("could not bind vlan socket");
  let (mut socket, _addr) = listener
    .accept()
    .await
    .expect("could not accept connection to vlan");
  let mut buf = vec![0u8; 10_000_000];
  loop {
    select! {
      maybe_amount = socket.read(&mut buf) => {
        let amount = maybe_amount.expect("vlan socket read error");
        let data = Vec::from(&buf[0..amount]);
        tx.send(data).await.expect("vlan sender fail");
      }
      Some(data) = rx.recv() => {
        socket.write_all(&data).await.expect("could not write all remote vlan data");
      }
    }
  }
}

/// The part connecting to the game server
pub async fn connect(port: u16, mut rx: Receiver<Vec<u8>>, tx: Sender<Vec<u8>>) {
  let socket = TcpSocket::new_v4().expect("could not make socket");
  let mut socket = socket.connect(format!("127.0.0.1:{port}").parse().unwrap()).await.expect("could not connect socket");

  let mut buf = vec![0u8; 10_000_000];
  loop {
    select! {
      maybe_amount = socket.read(&mut buf) => {
        let amount = maybe_amount.expect("vlan socket read error");
        let data = Vec::from(&buf[0..amount]);
        tx.send(data).await.expect("vlan sender fail");
      }
      Some(data) = rx.recv() => {
        socket.write_all(&data).await.expect("could not write all remote vlan data");
      }
    }
  }
}
