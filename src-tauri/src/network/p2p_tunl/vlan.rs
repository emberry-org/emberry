use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::{TcpListener, TcpSocket, TcpStream},
  select,
  sync::{
    mpsc::{Receiver, Sender},
    oneshot,
  },
};

use log::trace;

/// The part connecting to the game client
pub async fn listen(
  local_port: u16,
  mut remote_rx: Receiver<Vec<u8>>,
  remote_tx: Sender<Vec<u8>>,
  mut kill_rx: oneshot::Receiver<()>,
) -> Result<(), &'static str> {
  let listener_l = TcpListener::bind(format!("127.0.0.1:{local_port}"))
    .await
    .expect("could not bind vlan socket");

  loop {
    trace!("starting VLAN Listener");
    let (mut local_trx, _) = listener_l
      .accept()
      .await
      .expect("could not accept connection to vlan");

    spin(&mut local_trx, (&mut remote_rx, &remote_tx), &mut kill_rx).await?;
  }
}

/// The part connecting to the game server
pub async fn connect(
  local_port: u16,
  mut remote_rx: Receiver<Vec<u8>>,
  remote_tx: Sender<Vec<u8>>,
  mut kill_rx: oneshot::Receiver<()>,
) -> Result<(), &'static str> {
  loop {
    // wait for data before we even connect the socket (could be extra signal)
    let data = remote_rx.recv().await.expect("no first msg");
    trace!("connecting VLAN socket");

    let local_s = TcpSocket::new_v4().expect("could not make socket");
    let mut local_trx = local_s
      .connect(format!("127.0.0.1:{local_port}").parse().unwrap())
      .await
      .expect("could not connect socket");

    local_trx
      .write_all(&data)
      .await
      .expect("could not send initial");

    spin(&mut local_trx, (&mut remote_rx, &remote_tx), &mut kill_rx).await?;
  }
}

async fn spin(
  local_trx: &mut TcpStream,
  remote_trx: (&mut Receiver<Vec<u8>>, &Sender<Vec<u8>>),
  kill_rx: &mut oneshot::Receiver<()>,
) -> Result<(), &'static str> {
  let (mut local_rx, mut local_tx) = local_trx.split();
  let (remote_rx, remote_tx) = remote_trx;

  let mut buf = vec![0u8; 4092];
  loop {
    select! {
      opt_amount_l = local_rx.read(&mut buf) => {
        let amount_l = opt_amount_l.expect("vlan socket read error");
        let data_l = Vec::from(&buf[0..amount_l]);
        remote_tx.send(data_l).await.expect("vlan sender fail");
        if amount_l == 0 {
          trace!("VLAN: closed socket, restarting");
          return Ok(());
        }
      }
      Some(data_r) = remote_rx.recv() => {
        if data_r.is_empty() {
          trace!("VLAN: closed remote, restarting");
          return Ok(());
        }
        local_tx.write_all(&data_r).await.expect("could not write all remote vlan data");
      }
      _ = &mut *kill_rx => {
        trace!("vlan died");
        return Err("vlan killed by remote");
      }
    }
  }
}
