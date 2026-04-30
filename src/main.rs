use std::vec::Vec;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

use regex::Regex;
use clap::Parser;

// command line stuff
#[derive(Parser)]
#[command(version = "0.0.1")]
#[command(about = "CLI utility for checking the validity of links in markdown files")]
struct Cli {
    /// path to file or directory to be checked
    path: String,

    /// path to root (in the case of a website)
    root: String,

    /// recursively walk directory
    #[arg(short, long)]
    recursive: Option<bool>,
}

// internal datastructures
enum Link {
    File(PathBuf),
    Url(String),
}

impl Link {
    fn from_string(content: &str) -> Option<Link> {
	let url_reg = Regex::new(r"^[a-z0-9+.-]+:").unwrap();

	if url_reg.is_match(&content) {
	    return Some(Link::Url(content.to_string()));
	} else if !content.starts_with("#"){
	    let path = PathBuf::from(content.to_string());
	    return Some(Link::File(path));
	}

	None
    }
}

pub trait Walkable {
    //fn walk_dir(&mut self, dir: &PathBuf);
    fn get_dir(&mut self, dir: &PathBuf);
}

type Files = HashSet<PathBuf>;

impl Walkable for Files {
    //fn walk_dir(&mut self, dir: &PathBuf){
    //	todo!("recursive not yet implemented");
    //}

    fn get_dir(&mut self, dir: &PathBuf) {
	let entries = dir.read_dir().expect("read_dir call failed");
	for entry in entries {
	    if let Ok(entry) = entry {
		let file = PathBuf::from(entry.path());
		if file.is_dir() { continue; }
		else { self.insert(file); }
	    }
	}
    }
}

fn get_links_from_file(filepath: &PathBuf) -> Vec<Link> {
    let file = File::open(filepath);
    let mut content = String::new();
    file.expect("open file")
	.read_to_string(&mut content)
	.unwrap();

    let reg = Regex::new(r"\[(?P<text>[^\]]+)\]\((?P<url>[^)]+)\)").unwrap();
    let mut link_vec = Vec::new();

    for cap in reg.captures_iter(&content) {
	let link = match Link::from_string(&cap["url"]) {
	    Some(x) => x,
	    None => continue,
	};
	link_vec.push(link);
    }

    link_vec
}

fn main() {
    let cli = Cli::parse();

    let filepath = PathBuf::from(cli.path);
    let mut files = Files::new();

    if filepath.is_dir() {
	files.get_dir(&filepath);
    } else {
	files.insert(filepath);
    }

    let mut file_map = HashMap::new();
    for file in files {
	let link_vec = get_links_from_file(&file);
	file_map.insert(file, link_vec);
    }

    for (file, links) in file_map {
	println!("Found a file: {}", file.display());
	if links.len() == 0 { println!("  no links found"); }
	for link in links {
	    match link {
		Link::File(path) => println!("  file: {}", path.to_str().unwrap()),
		Link::Url(path)  => println!("  url: {}", path),
	    }
	}
    }

}
