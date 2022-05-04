use std::collections::{HashMap};

use structopt::StructOpt;
use bluer::{
    Address,
    rfcomm::{
        SocketAddr,
        Stream,
    },
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


// TODO migrate to Clap v3
#[derive(Debug, StructOpt)]
#[structopt(name = "buds", about = "CLI to comunicate with bluetooth Buds")]
struct Opt {
    /// Device mac address
    #[structopt(short, long)]
    mac: String,

    /// Device RFCOMM channel
    #[structopt(short, long)]
    channel: Option<u8>,
}

struct StreamWrapper {
    stream: Stream,
}

impl StreamWrapper {
    fn new(stream: Stream) -> Self {
        Self { stream }
    }

    async fn send(&mut self, data: &str) -> bluer::Result<usize> {
        Ok(self.stream.write(format!("\r\n{}\r\n", data).as_bytes()).await?)
    }

    async fn read(&mut self) -> bluer::Result<String> {
        let mut buf = vec![0; 4096];
        let n = self.stream.read(&mut buf).await?;
        let line = String::from_utf8_lossy(&buf[..n]);
        Ok(line.to_string())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    let opt = Opt::from_args();
    let address: Address = opt.mac.parse()?;

    let channel = match opt.channel {
        Some(channel) => channel,
        None => 0,
    };

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    // TODO handle error when channel = 0
    let peer_sa = SocketAddr::new(address, channel);
    let raw_stream = Stream::connect(peer_sa).await?;
    let mut stream = StreamWrapper::new(raw_stream);

    loop {
        let line = stream.read().await?;

        if line.contains("+BRSF") {
            stream.send("+BRSF: 1024").await?;
            stream.send("OK").await?;
        } else if line.contains("CIND=") {
            stream.send("+CIND: (\"battchg\",(0-5))").await?;
            stream.send("OK").await?;
        } else if line.contains("CIND?") {
            stream.send("+CIND: 5").await?;
            stream.send("OK").await?;
        } else if line.contains("XAPL=") {
            stream.send("+XAPL=iPhone,7").await?;
            stream.send("OK").await?;
        } else if line.contains("IPHONEACCEV") {
            let parts = line.trim().split(",").skip(1).collect::<Vec<&str>>();
            if parts.len() > 1 && parts.len() % 2 == 0 {
                let data = parts.iter().zip(parts.iter().skip(1)).collect::<HashMap<_, _>>();
                if let Some(val) = data.get(&"1") {
                    let level: usize = match val.parse::<usize>() {
                        Ok(v) => (v + 1) * 10,
                        Err(_) => 0,
                    };
                    println!("Battery level {}", level);
                }
            } else {
                println!("Battery level unknown");
            }
            break;
        } else {
            stream.send("OK").await?;
        }
    }

    drop(stream);

    Ok(())
}
