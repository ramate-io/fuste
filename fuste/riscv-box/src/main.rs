use clap::Parser;
use fubox::{Fubox, FuboxError};

#[tokio::main]
async fn main() -> Result<(), FuboxError> {
	let fubox = Fubox::parse();
	fubox.execute().await
}
