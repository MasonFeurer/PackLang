use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

static mut FILES:Option<FileManager> = None;

pub struct FileManager {
	paths: Vec<String>,
	sources: Vec<String>,
}

pub fn init() {
	unsafe { FILES = Some(FileManager {
		paths: Vec::new(),
		sources: Vec::new(),
	}) }
}
pub fn file_mgr() -> &'static FileManager {
	unsafe { FILES.as_ref().unwrap() }
}
pub fn mut_file_mgr() -> &'static mut FileManager {
	unsafe { FILES.as_mut().unwrap() }
}
pub fn load_file_src(path:&str, source:&str) {
	let mgr = mut_file_mgr();
	mgr.paths.push(path.to_owned());
	mgr.sources.push(source.to_owned());
}
pub fn load_file(path:&str) {
	let source = read_file(path);
	load_file_src(path, source.as_str());
}
pub fn ref_file(index:usize) -> FileRef {
	let mgr = file_mgr();
	assert!(index < mgr.paths.len());
	FileRef(index)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FileRef(usize);
impl FileRef {
	pub fn source(&self) -> &'static String {
		&file_mgr().sources[self.0]
	}
	pub fn path(&self) -> &'static String {
		&file_mgr().paths[self.0]
	}

	pub fn line_source(&self, line:usize) -> String {
		let mut line_src = String::new();

		let mut c_line:usize = 0;
		for c in self.source().chars() {
			if c == '\n' { c_line += 1 }
			if c_line > line { break }
			if c_line == line && c != '\n' { line_src.push(c) }
		}
		line_src
	}
	pub fn get_line_pos(&self, line:usize) -> usize {
		let mut c_line:usize = 0;
		for (i, c) in self.source().chars().enumerate() {
			if c_line == line { return i }
			if c == '\n' { c_line += 1 }
		}
		panic!("couldn't find index of line start")
	}
}

pub fn read_file(path:&str) -> String {
	let mut src = String::new();
	let mut file = File::open(path).expect(format!(
		"failed to open file {}", path
	).as_str());
	file.read_to_string(&mut src).expect(format!(
		"failed to read file {}", path
	).as_str());
	src
}
