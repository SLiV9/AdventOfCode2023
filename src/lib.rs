/**/

mod lib
{
	pub mod ring_buffer;
}
pub use lib::*;

#[macro_export]
macro_rules! run {
	($f:ident($x:expr)) => {
		let start = std::time::Instant::now();
		let output = $f($x);
		let runtime_in_ms = start.elapsed().as_secs_f64() * 1000.0;
		let path = std::path::Path::new(std::file!());
		let binname = path
			.parent()
			.unwrap()
			.file_name()
			.unwrap()
			.to_str()
			.unwrap();
		let partname = stringify!($f);
		println!(
			"### {binname} part {partname}: {output} ### (took \
			 {runtime_in_ms:.1} ms)"
		);

		if cfg!(not(debug_assertions))
		{
			use std::io::Write;
			let mut log = std::fs::OpenOptions::new()
				.append(true)
				.open("meta/runtime.log.txt")
				.unwrap();
			write!(log, "{binname} part {partname}: {runtime_in_ms:.1} ms\n")
				.unwrap();
		}
	};
}
