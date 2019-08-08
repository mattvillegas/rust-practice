use regex::Regex;
use std::env::Args;
use std::fs::{read_dir, ReadDir};
use std::io::Result;
use std::path::{Path, PathBuf};

fn get_matches(paths: Vec<PathBuf>, pattern: Regex) -> Vec<PathBuf> {
    unimplemented!()
}
fn get_directores(dir: ReadDir) -> Result<Vec<PathBuf>> {
    unimplemented!()
}

fn parse_args(args: Vec<String>) -> Option<(Regex, PathBuf)> {
    unimplemented!()
}

fn main() {
    println!("Hello, world!");
}
