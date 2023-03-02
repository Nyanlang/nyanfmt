use nyanfmt::format;
use std::fs::{read_to_string, write};

use clap::Parser;

#[derive(Parser)]
#[command(
	version,
	about = "Format nyanlang code\n\nBy default, output will be written to stdout.\nUse --write flag to actually save the output."
)]
struct Arg {
	file: String,

	/// write the formatted result to the file
	#[arg(short, long)]
	write: bool,
}

fn main() {
	let opt = Arg::parse();

	let code =
		read_to_string(&opt.file).expect(&format!("Can't read {}", &opt.file));
	let result = format(code);

	if opt.write {
		write(&opt.file, result)
			.expect(&format!("Can't write to {}", &opt.file));
	} else {
		println!("{result}");
	}
}
