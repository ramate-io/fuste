use clap::Parser;
use fubox::{Fubox, FuboxError};

#[tokio::main]
async fn main() -> Result<(), FuboxError> {
	let fubox = Fubox::parse();
	match fubox.execute().await {
		Ok(()) => Ok(()),
		Err(e) => {
			eprintln!("Error: {}", e);
			Ok(())
		}
	}
}
