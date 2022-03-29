use std::fmt::Debug;
use crate::compiler::tokens::SrcScope;
use crate::debug::bash_tools::*;

pub struct ErrorInfo<'a, 'b, 'c> {
	pub cause: &'a str,
	pub pointer: &'b str,
	pub context: Option<&'c str>,
	pub help: Option<Help>,
}

pub struct Help {
	pub msg: String,
	pub source: String,
	pub line: usize,
}

#[derive(Debug)]
pub enum CompileErr {
	Error(String),
	Warning(String),
}
impl CompileErr {
	pub fn msg(self) -> String {
		match self {
			Self::Error(msg) => msg,
			Self::Warning(msg) => msg,
		}
	}
	pub fn fatal(self) -> ! {
		println!("{}", self.msg());
		panic!("fatal compiling error");
	}
}

#[derive(Debug)]
pub enum CompileResult<T: Debug> {
	Ok(T),
	Err(CompileErr),
}
impl<T: Debug> CompileResult<T> {
	pub fn unwrap(self) -> T {
		if let Self::Ok(t) = self { t }
		else { panic!("unwrapped {:?}", self) }
	}
	pub fn fatal(self) -> T {
		match self {
			Self::Ok(t) => t,
			Self::Err(err) => err.fatal(),
		}
	}
}

pub fn warning(scope:SrcScope, info:ErrorInfo) -> CompileErr {
	CompileErr::Warning(create_error(
		Fmt::DecorColor(BOLD,YELLOW),
		"warning",
		scope, info
	))
}
pub fn error(scope:SrcScope, info:ErrorInfo) -> CompileErr {
	CompileErr::Error(create_error(
		Fmt::DecorColor(BOLD,RED),
		"error",
		scope, info
	))
}

fn create_error(
	fmt: Fmt,
	ty: &str,
	scope: SrcScope,
	info: ErrorInfo
) -> String {
	let mut msg = format!(
		"{}{}{}: {}{}\n",
		fmt,
		ty,
		Fmt::DecorColor(BOLD,LIGHT_WHITE),
		info.cause,
		Fmt::Reset
	);
	if let Some(context) = info.context {
		msg.push_str("context: ");
		msg.push_str(context);
		msg.push('\n');
	}

	let mut fmt = DebugLines::new();
	underline_scope(
		&mut fmt,
		scope,
		info.pointer,
		Fmt::DecorColor(PLAIN, RED),
		Fmt::DecorColor(BOLD, RED),
	);
	if let Some(help) = info.help {
		fmt.help(help);
	}

	msg.push_str(fmt.fmt().as_str());
	msg
}

fn underline_scope(
	fmt: &mut DebugLines,
	scope: SrcScope,
	pointer: &str,
	source_color: Fmt,
	underline_color: Fmt,
) {
	let lines = scope.lines();
	assert_ne!(lines.len(), 0);
	let first_line_pos = scope.file.get_line_pos(lines[0]);

	fmt.push_line(DebugLine::File {
		path: scope.file.path().clone(),
		line: lines[0],
		col: scope.start-first_line_pos,
	});

	if lines.len() == 1 {
		fmt.push_line(DebugLine::Source {
			line: lines[0],
			source: scope.file.line_source(lines[0]),
			color: source_color
		});

		fmt.push_line(DebugLine::Underline {
			ch: '^',
			offset: scope.start-first_line_pos,
			len: scope.len(),
			msg: pointer.to_owned(),
			color: underline_color,
		});
	}
	else {
		for i in 0..lines.len() {
			fmt.push_line(DebugLine::GroupedSource {
				line: lines[i],
				source: scope.file.line_source(lines[i]),
				color: source_color,
				group_color: underline_color,
			});
		}
		let line_pos = scope.file.get_line_pos(lines[lines.len()-1]);
		fmt.push_line(DebugLine::GroupedUnderline {
			ch: '^',
			offset: 0,
			len: scope.end-line_pos,
			msg: pointer.to_owned(),
			color: underline_color,
			group_color: underline_color,
		});
	}
}

enum DebugLine {
	Text(String),
	Source {
		line: usize,
		source: String,
		color: Fmt,
	},
	Underline {
		ch: char,
		offset: usize,
		len: usize,
		msg: String,
		color: Fmt,
	},
	GroupedSource {
		line: usize,
		source: String,
		color: Fmt,
		group_color: Fmt,
	},
	GroupedUnderline {
		ch: char,
		offset: usize,
		len: usize,
		msg: String,
		color: Fmt,
		group_color: Fmt,
	},
	File {
		path: String,
		line: usize,
		col: usize,
	},
	SourceSkip,
	Blank,
	Empty,
}
impl DebugLine {
	fn fmt_gutter(text:String, gutter_size:usize) -> String {
		let space = gutter_size - text.len();
		format!(
			"{}{}{} | {}",
			Fmt::DecorColor(BOLD, WHITE),
			" ".repeat(space),
			text,
			Fmt::Reset,
		)
	}

	pub fn gutter(&self) -> Option<String> {
		match self {
			Self::Text(_) => None,
			Self::Source { line, .. } => Some((line+1).to_string()),
			Self::Underline { .. } => Some("".to_owned()),
			Self::GroupedSource { line, .. } => Some((line+1).to_string()),
			Self::GroupedUnderline { .. } => Some("".to_owned()),
			Self::File { .. } => None,
			Self::SourceSkip => Some("...".to_owned()),
			Self::Blank => Some("".to_owned()),
			Self::Empty => None,
		}
	}

	pub fn fmt(&self) -> String {
		match self {
			Self::Text(text) => text.clone(),
			Self::Source { line:_, source, color } => {
				format!("{}{}{}", color, source, Fmt::Reset)
			}
			Self::Underline { ch, offset, len, msg, color } => {
				let mut string = String::new();
				string.push_str(color.to_string().as_str());
				string.push_str(" ".repeat(*offset).as_str());
				string.push_str(ch.to_string().repeat(*len).as_str());
				string.push_str(" ");
				string.push_str(msg.as_str());
				string.push_str(Fmt::Reset.to_string().as_str());
				string
			}
			Self::GroupedSource { line:_, source, color, group_color } => {
				format!("{}| {}{}{}", group_color, color, source, Fmt::Reset)
			}
			Self::GroupedUnderline { ch, offset, len, msg, color, group_color } => {
				let mut string = String::new();
				string.push_str(group_color.to_string().as_str());
				string.push_str("| ");
				string.push_str(color.to_string().as_str());
				string.push_str(" ".repeat(*offset).as_str());
				string.push_str(ch.to_string().repeat(*len).as_str());
				string.push_str(" ");
				string.push_str(msg.as_str());
				string.push_str(Fmt::Reset.to_string().as_str());
				string
			}
			Self::File { path, line, col } => format!(
				"{}file {}:{}:{}{}",
				Fmt::DecorColor(BOLD, LIGHT_BLUE),
				path, line+1, col+1, Fmt::Reset
			),
			Self::SourceSkip => "".to_owned(),
			Self::Blank => "".to_owned(),
			Self::Empty => "".to_owned(),
		}
	}
}

struct DebugLines {
	lines: Vec<DebugLine>,
}
impl DebugLines {
	pub fn new() -> Self {
		DebugLines { lines: Vec::new() }
	}
	pub fn push_line(&mut self, line:DebugLine) {
		self.lines.push(line);
	}
	pub fn help(&mut self, help:Help) {
		self.push_line(DebugLine::Text(format!(
			"{}help: {}{}",
			Fmt::DecorColor(BOLD, PURPLE),
			Fmt::DecorColor(BOLD, LIGHT_WHITE),
			help.msg,
		)));
		self.push_line(DebugLine::Source {
			line: help.line,
			source: help.source,
			color: Fmt::DecorColor(PLAIN, LIGHT_CYAN),
		});
		self.push_line(DebugLine::Blank);
	}

	pub fn fmt(&self) -> String {
		let gutter_size = self.gutter_size();
		let mut string = String::new();

		for line in &self.lines {
			if let Some(gutter) = line.gutter() {
				string.push_str(DebugLine::fmt_gutter(
					gutter, gutter_size
				).as_str());
			}
			string.push_str(line.fmt().as_str());
			string.push('\n');
		}
		string
	}
	fn gutter_size(&self) -> usize {
		let mut gutter_size:usize = 0;
		for line in &self.lines {
			let Some(gutter) = line.gutter() else { continue };
			if gutter.len() > gutter_size {
				gutter_size = gutter.len();
			}
		}
		gutter_size
	}
}
