use std::collections::HashSet;
use std::path::Path;
use std::io::Error as IOError;
use crate::utils::{map_atom_to_entries, map_entries_to_atom};
use crate::lock::Lock;
use crate::atom::AtomMetadata;
use crate::group::Group;

fn log_map_atom_to_entries(lock: &Lock, path: &str) {
	let entries = map_atom_to_entries(lock, path, true, &["cache"]);

	if !entries.is_empty(){
		println!("Missing:");
	}
	for entry in entries {
		if !Path::new(&entry).exists() {
			println!("\t\"{}\"", entry);
		}
	}
}

fn log_map_to_lock(path: &str, lock: &Lock, ignore: &Lock)
-> Result<(), IOError> {
	let entries = map_entries_to_atom(path, lock, ignore, false)?;

	if !entries.is_empty() {
		println!("Extra:");
	}
	for entry in entries {
		println!("\t\"{}\"", entry);
	}

	return Ok(());
}

pub fn validate_atoms(lock: &Lock, ignore: &Lock, root_dir: &str) 
-> Result<(), IOError> {
	log_map_atom_to_entries(lock, root_dir);
	log_map_to_lock(root_dir, lock, ignore)?;

	return Ok(());
}

fn map_to_atoms(groups: &Vec<Group>, atoms: &Vec<AtomMetadata>) {
	let file_atoms: Vec<String> = atoms.iter()
		.map(|a| a.name.to_string())
		.collect();

	for group in groups {
		for atom in &group.atoms {
			if !file_atoms.contains(&atom.to_string()) {
				println!(
					"Group \"{}\" references non-existent package \"{}\"",
					group.name,
					atom
				);
			}
		}
	}
}

fn map_to_groups(atoms: &Vec<AtomMetadata>, groups: &Vec<Group>) {
	let grouped_atoms: HashSet<String> = groups
		.iter()
		.map(|g| g.atoms.clone())
		.flatten()
		.collect();

	if !atoms
		.iter()
		.filter(|a|
			!grouped_atoms
				.contains(&a.name)
		).collect::<Vec<&AtomMetadata>>()
		.is_empty() {
		println!("These packages are not in any group:")
	}
	for atom in atoms {
		if !grouped_atoms.contains(&atom.name) {
			println!("\t\"{}\"", &atom.name);
		}
	}
}

pub fn validate_groups(atoms: &Vec<AtomMetadata>, groups: &Vec<Group>) {
	map_to_atoms(&groups, &atoms);
	map_to_groups(&atoms, &groups);
}
