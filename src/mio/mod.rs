//! automata mio async API
//!
//!
//!

pub mod pump;
pub mod sensor;

use serde::{Deserialize, Serialize};
use async_std::fs;
use async_std::io;
use async_std::os::unix::fs::symlink;
use async_std::os::unix::net::UnixDatagram;
use async_std::prelude::*;

// use log::info;
use std::path::{PathBuf,Path};

#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Mio {
    path: PathBuf,
}
impl Mio {
    pub fn directory(&self) -> PathBuf {
        rootdir().join(self.name())
    }
    pub fn name(&self) -> String {
        format!("mio")
    }
    pub fn datagramm(&self) -> PathBuf {
        self.directory().join("datagram.socket")
    }
}

pub fn workdir() -> PathBuf {
    super::workdir().join("mio")
}

pub fn rootdir() -> PathBuf {
    let path = PathBuf::from("/var/run/automata/mio");
    path
}

// pub async fn create(id: u64) -> io::Result<Mio> {
//     use yansi::Paint;
//     let mio = Mio { id };
//     let path = mio.directory();
//     if !path.exists() {
//         fs::DirBuilder::new()
//             .recursive(true)
//             .create(path.as_path())
//             .await?;
//         info!("{:} new creat", Paint::cyan(format!("MIO:{}", id)));
//     }
//     Ok(mio)
// }


pub async fn label(path: &Path)-> io::Result<String> {
    let label = path.join("label");
    fs::read_to_string(&label).await
}


pub async fn state(mio: &Mio) -> io::Result<String> {
    let path = mio.directory().join("state");
    let mut file = fs::File::open(path.as_path()).await?;
    let mut contents = Vec::new();
    let _n = file.read(&mut contents).await?;
    let res = String::from_utf8(contents).unwrap();
    Ok(res)
}

pub async fn link(mio: &Mio, owner: PathBuf) -> io::Result<()> {
    let path = mio.directory().join("owner");
    symlink(path.as_path(), owner.as_path()).await?;
    let link = owner.join(format!("mio12"));
    let path = mio.directory();
    symlink(path.as_path(), link.as_path()).await?;
    Ok(())
}

pub async fn datagram(mio: &Mio) -> io::Result<UnixDatagram> {
    let path = mio.directory().join("socket");
    let socket = UnixDatagram::bind(path.as_path()).await?;
    Ok(socket)
}



#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task;

    #[test]
    fn create_send_recv() -> io::Result<()> {
        task::block_on(async {
            // let mio    = create(23).await?;
            // let socket = datagram(&mio).await?;
            //  let path = mio.directory().join("socket");
            // let reciever = UnixDatagram::bind("/tmp/socket").await?;
            // let mut buf = vec![0; 1024];
            // let n = socket.recv(&mut buf).await?;

            // let sender = UnixDatagram::unbound()?;
            // socket.send(b"hello world").await?;
            // let mut buf = vec![0u8; 1024];
            // let (n, peer) =reciever.recv_from(&mut buf).await?;
            // let p = buf.as_slice();
            // println!("n={} peer={:?} {:?}",n,peer,&buf);
            // assert_eq!(&buf[..n], b"hello world");
            // let socket1 = UdpSocket::bind("127.0.0.1:0").await?;
            // let socket2 = UdpSocket::bind("127.0.0.1:0").await?;

            // socket1.connect(socket2.local_addr()?).await?;
            // socket2.connect(socket1.local_addr()?).await?;

            // socket1.send(THE_MERCHANT_OF_VENICE).await?;

            // let mut buf = [0u8; 1024];
            // let n = socket2.recv(&mut buf).await?;
            // assert_eq!(&buf[..n], THE_MERCHANT_OF_VENICE);

            Ok(())
        })
    }
}
