use anyhow::Result;
use clap::Parser;
use std::net::IpAddr;
use tokio::runtime::Runtime;

#[derive(Debug, Parser)]
struct Args {
    #[arg()]
    addr: IpAddr,
    /// --start_port
    #[arg(long, default_value_t = 1)]
    start_port: u16,
    /// --end_port
    #[arg(long, default_value_t = 1024)]
    end_port: u16,
}
fn main() -> Result<()> {
    let args = Args::parse();
    assert!(args.start_port > 0 && args.end_port >= args.start_port);
    dbg!(&args);
    let rt = Runtime::new()?;
    rt.block_on(async {});

    Ok(())
}
