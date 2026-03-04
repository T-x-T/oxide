#![allow(clippy::needless_return, unused)]

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

mod blockentity;
mod blocks;
mod entities;
mod items;
mod loot_tables;
mod tags;

fn main() {
	blockentity::generate();
	entities::generate();
	items::generate();
	blocks::generate();
	tags::generate();
	loot_tables::generate();
}

fn convert_to_upper_camel_case(input: &str) -> String {
	let mut found_underscore = false;
	return input
		.replace("minecraft:", "")
		.chars()
		.enumerate()
		.map(|i| {
			if i.1 == '_' {
				found_underscore = true;
				return i.1;
			}
			if i.0 == 0 || found_underscore {
				found_underscore = false;
				return i.1.to_ascii_uppercase();
			} else {
				return i.1;
			}
		})
		.filter(|i| *i != '_')
		.collect();
}

fn read_dir_recursively(path: PathBuf) -> std::io::Result<Vec<fs::DirEntry>> {
	let mut output: Vec<fs::DirEntry> = Vec::new();

	for entry in fs::read_dir(path).unwrap() {
		let entry = entry.unwrap();
		if entry.metadata().unwrap().is_file() {
			output.push(entry);
		} else {
			output.append(&mut read_dir_recursively(entry.path()).unwrap());
		}
	}

	return Ok(output);
}
