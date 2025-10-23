pub mod run;

use clap::Parser;
use clap_markdown_ext::Markdown;

#[derive(Debug, thiserror::Error)]
pub enum FuboxError {
	#[error("Encountered an error while generating documentation: {0}")]
	MarkdownError(#[from] anyhow::Error),
	#[error("Encountered an error while running the program: {0}")]
	RunError(#[from] run::RunError),
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum Fubox {
	/// Generate CLI documentation
	#[clap(subcommand)]
	Markdown(Markdown),
	/// Run a RISC-V program in the box
	#[clap(subcommand)]
	Run(run::Run),
}

impl Fubox {
	pub async fn execute(&self) -> Result<(), FuboxError> {
		match self {
			Fubox::Markdown(markdown) => {
				markdown.execute::<Self>().await?;
			}
			Fubox::Run(run) => {
				run.execute().await?;
			}
		}

		Ok(())
	}
}
