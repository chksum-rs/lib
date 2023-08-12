use std::env;
use std::path::PathBuf;

use chksum::hash::SHA1;
use chksum::{chksum, Result};

fn main() -> Result<()> {
    // Skip the first argument because it is not necessary to calculate digest of the binary itself
    for arg in env::args().skip(1) {
        // Create path
        let path = PathBuf::from(arg);

        // Calculate digest
        let digest = chksum::<SHA1, _>(&path)?;

        // Print digest
        let path = path.display();
        let digest = digest.to_hex_lowercase();
        println!("{path} {digest}");
    }

    Ok(())
}
