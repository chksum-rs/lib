use anyhow::Result;
use chksum_build::{setup, BuildScript};

fn main() -> Result<()> {
    setup(&BuildScript)
}
