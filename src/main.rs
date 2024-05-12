use anyhow::Result;
use cidr::IpCidr;
use clap::Parser;
use std::net::IpAddr;
use tokio::{
    net::TcpStream,
    runtime::Runtime,
    sync::mpsc::{channel, Sender},
};

#[derive(Debug, Parser)]
struct Args {
    #[arg(conflicts_with("cidr"), required_unless_present("cidr"))]
    addr: Option<IpAddr>,
    #[arg(long)]
    cidr: Option<IpCidr>,
    /// --start-port
    #[arg(long, default_value_t = 1)]
    start_port: u16,
    /// --end-port
    #[arg(long, default_value_t = 1024)]
    end_port: u16,
}
fn main() -> Result<()> {
    let args = Args::parse();
    assert!(args.start_port > 0 && args.end_port >= args.start_port);
    dbg!(&args);
    let rt = Runtime::new()?;
    let (tx, mut rx) = channel(10);
    rt.block_on(async {
        let (mut from_simgle, mut from_cidr);
        let addresses: &mut dyn Iterator<Item = IpAddr> = match (args.addr, args.cidr) {
            (Some(addr), _) => {
                from_simgle = vec![addr].into_iter();
                &mut from_simgle
            }
            (_, Some(cidr)) => {
                from_cidr = cidr.iter().map(|n| n.address());
                &mut from_cidr
            }
            (_, _) => unreachable!(),
        };
        for addr in addresses {
            for port in args.start_port..=args.end_port {
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Err(err) = scan(addr, port, tx).await {
                        eprintln!("error: {err}");
                    }
                });
            }
        }
        // for task in tasks {
        //     task.await.unwrap();
        // }
    });
    drop(tx);
    while let Ok((addr, port)) = rx.try_recv() {
        dbg!(&addr);
        dbg!(&port);
    }
    Ok(())
}
async fn scan(addr: IpAddr, port: u16, results_tx: Sender<(IpAddr, u16)>) -> Result<()> {
    if let Ok(_open) = TcpStream::connect((addr, port)).await {
        if let Err(e) = results_tx.send((addr, port)).await {
            dbg!(e);
        }
    }
    Ok(())
}
