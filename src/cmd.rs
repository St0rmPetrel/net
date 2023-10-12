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
        let ping = ping::Ping::new_default(&self.host)?;

        let _dur = ping.echo(0);

        Ok(())
    }
}
