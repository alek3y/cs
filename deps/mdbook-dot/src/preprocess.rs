use mdbook::{
	book::Book,
	errors::Result,
	preprocess::{Preprocessor, PreprocessorContext},
	BookItem,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use regex::{Captures, Regex};
use std::io::Write;
use std::process::{Command, Stdio};

pub struct DotPreprocessor;

impl Preprocessor for DotPreprocessor {
	fn name(&self) -> &str {
		"dot"
	}

	fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
		let mut chapters = Vec::with_capacity(book.sections.len());
		book.for_each_mut(|item| {
			if let BookItem::Chapter(chapter) = item {
				chapters.push(chapter.content.clone());
			}
		});

		let mut contents: Vec<_> = chapters
			.into_par_iter()
			.rev()
			.map(process_chapter)
			.collect();

		book.for_each_mut(|item| {
			if let BookItem::Chapter(chapter) = item {
				chapter.content = contents.pop().unwrap();
			}
		});

		Ok(book)
	}
}

fn sanitize(svg: String) -> String {
	let svg = Regex::new(r"<!DOCTYPE [^>]+>").unwrap().replace(&svg, "");
	let svg = Regex::new(r"<\?xml [^>]+\?>").unwrap().replace(&svg, "");
	let svg = Regex::new(r"\n").unwrap().replace_all(&svg, "");

	svg.trim().into()
}

pub fn process_chapter(raw: String) -> String {
	Regex::new(r"(?s)`{3}dot process\n(|.*?[^\\])`{3}")
		.unwrap()
		.replace_all(&raw, |caps: &Captures| {
			let graph = caps.get(1).unwrap().as_str();
			let svg = sanitize(render_dot(graph));
			format!("<div>{svg}</div>")
		})
		.into()
}

pub fn render_dot(graph: &str) -> String {
	let mut child = Command::new("dot")
		.args(["-Tsvg"])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("Couldn't run dot");

	child
		.stdin
		.as_mut()
		.unwrap()
		.write_all(graph.as_bytes())
		.expect("Couldn't send graph to dot");

	if let Ok(output) = child.wait_with_output() {
		String::from_utf8(output.stdout).unwrap()
	} else {
		String::new()
	}
}
