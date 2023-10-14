use anyhow::Result;
use clap::Args;

mod ping;

#[derive(Args)]
pub struct Ping {
    /// destination host or gateway.
    host: String,
}

impl Ping {
    pub fn exec(self) -> Result<()> {
        let mut ping = ping::Ping::new_default(&self.host)?;

        ping.ping()
    }
}
