use mdbook::errors::Result;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::{env, io};

mod preprocess;
use crate::preprocess::DotPreprocessor;

fn main() -> Result<()> {
	let supports_mode = env::args().any(|arg| arg == "supports");

	if !supports_mode {
		let preprocessor = DotPreprocessor;
		let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
		let processed = preprocessor.run(&ctx, book)?;
		serde_json::to_writer(io::stdout(), &processed)?;
	}

	Ok(())
}
