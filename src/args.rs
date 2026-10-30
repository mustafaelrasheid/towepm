use clap::{Parser, Subcommand};
use const_format::{concatcp, map_ascii_case, Case};

const NAME: &str = "towepm";

#[derive(Parser)]
#[command(name = NAME)]
#[command(version)]
#[command(author = "mustafaelrasheid")]
#[command(
	about = "Package manager for bricks",
	long_about = None
)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	RebuildLock {
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
	},
	Validate {
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
		#[arg(long, default_value = "", env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_ROOTDIR"))]
		root_dir: String,
	},
	Export {
		packages: Vec<String>,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME),"_LIBDIR"))]
		lib_dir: String,
		#[arg(long, default_value = "", env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_ROOTDIR"))]
		root_dir: String,
	},
	Install {
		packages: Vec<String>,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
		#[arg(long, default_value = "", env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_ROOTDIR"))]
		root_dir: String,
		#[arg(long)]
		force: bool,
		#[arg(long)]
		yes: bool,
		#[arg(long)]
		run_scripts: bool,
	},
	Purge {
		packages: Vec<String>,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
		#[arg(long, default_value = "", env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_ROOTDIR"))]
		root_dir: String,
		#[arg(long)]
		yes: bool,
		#[arg(long)]
		run_scripts: bool,
	},
	Convert {
		packages: Vec<String>,
		#[arg(long, num_args=1..)]
		deps: Vec<String>,
	},
	GetDeb {
		packages: Vec<String>,
	},
	View {
		package: String,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
	},
	Tag {
		package: String,
		group: String,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
	},
	Untag {
		package: String,
		group: String,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
	},
	CreateGroup {
		group: String,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
	},
	ListGroup {
		groups: Vec<String>,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
	},
	DeleteGroup {
		group: String,
		#[arg(long, default_value = concatcp!("/var/lib/", NAME), env = concatcp!(map_ascii_case!(Case::Upper, NAME), "_LIBDIR"))]
		lib_dir: String,
	},
} 
