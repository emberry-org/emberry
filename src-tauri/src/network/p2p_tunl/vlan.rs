use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::{TcpListener, TcpSocket},
  select,
  sync::mpsc::{Receiver, Sender},
};

use log::trace;

/// The part connecting to the game client
pub async fn listen(local_port: u16, mut remote_rx: Receiver<Vec<u8>>, remote_tx: Sender<Vec<u8>>) {
  trace!("starting VLAN Listener");

  let listener_l = TcpListener::bind(format!("127.0.0.1:{local_port}"))
    .await
    .expect("could not bind vlan socket");
  let (mut local_trx, _) = listener_l
    .accept()
    .await
    .expect("could not accept connection to vlan");

  let (mut local_rx, mut local_tx) = local_trx.split();
  let mut buf = vec![0u8; 4092];
  loop {
    select! {
      opt_amount_l = local_rx.read(&mut buf) => {
        let amount_l = opt_amount_l.expect("vlan socket read error");
        let data_l = Vec::from(&buf[0..amount_l]);
        remote_tx.send(data_l).await.expect("vlan sender fail");
        if amount_l == 0 {
          trace!("VLAN: closed");
          return;
        }
      }
      Some(data_r) = remote_rx.recv() => {
        if data_r.is_empty() {
          remote_tx.send(vec![]).await.expect("vlan sender fail");
          return;
        }
        local_tx.write_all(&data_r).await.expect("could not write all remote vlan data");
      }
    }
  }
}

/// The part connecting to the game server
pub async fn connect(
  local_port: u16,
  mut remote_rx: Receiver<Vec<u8>>,
  remote_tx: Sender<Vec<u8>>,
) {
  let data = remote_rx.recv().await.expect("no first msg");
  trace!("connecting VLAN socket");

  let local_s = TcpSocket::new_v4().expect("could not make socket");
  let mut local_trx = local_s
    .connect(format!("127.0.0.1:{local_port}").parse().unwrap())
    .await
    .expect("could not connect socket");

  let (mut local_rx, mut local_tx) = local_trx.split();
  local_tx
    .write_all(&data)
    .await
    .expect("could not send initial");

  let mut buf = vec![0u8; 4092];
  loop {
    select! {
      opt_amount_l = local_rx.read(&mut buf) => {
        let amount_l = opt_amount_l.expect("vlan socket read error");
        let data_l = Vec::from(&buf[0..amount_l]);
        remote_tx.send(data_l).await.expect("vlan sender fail");
        if amount_l == 0 {
          trace!("VLAN: closed");
          return;
        }
      }
      Some(data_r) = remote_rx.recv() => {
        if data_r.is_empty() {
          remote_tx.send(vec![]).await.expect("vlan sender fail");
          trace!("VLAN: closed");
          return;
        }
        local_tx.write_all(&data_r).await.expect("could not write all remote vlan data");
      }
    }
  }
}
