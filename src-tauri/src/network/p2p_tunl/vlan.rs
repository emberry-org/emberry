use tokio::{
  io::{AsyncReadExt, AsyncWriteExt, ReadBuf},
  net::{TcpListener, TcpSocket},
  select,
  sync::mpsc::{Receiver, Sender},
};

use log::trace;

/// The part connecting to the game client
pub async fn listen(port: u16, mut rx: Receiver<Vec<u8>>, tx: Sender<Vec<u8>>) {
  trace!("starting VLAN Listener");

  let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
    .await
    .expect("could not bind vlan socket");
  let (mut socket, _addr) = listener
    .accept()
    .await
    .expect("could not accept connection to vlan");
  let mut buf = vec![0u8; 4092];
  loop {
    select! {
      maybe_amount = socket.read(&mut buf) => {
        let amount = maybe_amount.expect("vlan socket read error");
        let data = Vec::from(&buf[0..amount]);
        tx.send(data).await.expect("vlan sender fail");
        if amount == 0 {
          trace!("VLAN: closed");
          return;
        }
      }
      Some(data) = rx.recv() => {
        if data.is_empty() {
          tx.send(vec![]).await.expect("vlan sender fail");
          return;
        }
        socket.write_all(&data).await.expect("could not write all remote vlan data");
      }
    }
  }
}

/// The part connecting to the game server
pub async fn connect(port: u16, mut rx: Receiver<Vec<u8>>, tx: Sender<Vec<u8>>) {

  let data = rx.recv().await.expect("no first msg");
  trace!("connecting VLAN socket");

  let socket = TcpSocket::new_v4().expect("could not make socket");
  let mut socket = socket
    .connect(format!("127.0.0.1:{port}").parse().unwrap())
    .await
    .expect("could not connect socket");
  
  socket.write_all(&data).await.expect("could not send initial");

  let mut buf = vec![0u8; 4092];
  loop {
    select! {
      maybe_amount = socket.read(&mut buf) => {
        let amount = maybe_amount.expect("vlan socket read error");
        let data = Vec::from(&buf[0..amount]);
        tx.send(data).await.expect("vlan sender fail");
        if amount == 0 {
          trace!("VLAN: closed");
          return;
        }
      }
      Some(data) = rx.recv() => {
        if data.is_empty() {
          tx.send(vec![]).await.expect("vlan sender fail");
          trace!("VLAN: closed");
          return;
        }
        socket.write_all(&data).await.expect("could not write all remote vlan data");
      }
    }
  }
}
