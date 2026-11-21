use std::process::{Command, Stdio};
//use chrono::{Local};
use std::path::PathBuf;
use rfd::FileDialog;


pub const AVAILABLE_FILES_EXT: [&str; 9] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav", "gif", "avif"];

 
#[derive(Clone, Default)]
pub struct FileItem {
	pub file: PathBuf,
	pub is_active: bool,
	pub status: Status
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Status {
	Success,
	Error,

	InProcess,
	#[default]
	NotActive
}

pub fn cut_path(file_path: &str, finaly_count:usize) -> String {
	let flen = file_path.chars().count();
	if flen <= finaly_count {
		file_path.to_string()
	}
	else {
		let mut cutted_path: PathBuf = PathBuf::new();
		for i in PathBuf::from(file_path).iter().rev() {
			let now_len = cutted_path.to_string_lossy().as_ref().chars().count();
			if now_len - 5 < finaly_count {
				cutted_path.push(i);
			}
		}
		let mut new_path: PathBuf = PathBuf::new();
		for i in cutted_path.iter().rev().skip(2) { 
			new_path.push(i);
		}
		format!("/.../{}", new_path.to_string_lossy().as_ref())
	}
}
pub fn check_ffmpeg() -> bool {
	Command::new("ffmpeg").arg("-version").output().map(|output| output.status.success()).unwrap_or(false)
}
pub fn select_all(files:&[FileItem]) -> Vec<FileItem> {
	let mut new_files:Vec<FileItem> = Vec::new();
		for FileItem{file, is_active: _, status} in files.iter() {
			new_files.push(FileItem{file: file.clone(), is_active: true, status: *status});
		}
	new_files
}
pub fn clear_selected(files:&Vec<FileItem>) -> Vec<FileItem> {
	let mut new_vector:Vec<FileItem> = Vec::new();
	for file in files {
		if !file.is_active {
			new_vector.push(file.clone())
		}
	}
	new_vector
}

pub fn add_files(files: &mut Vec<FileItem>) -> Vec<FileItem> {
	let added_files = FileDialog::new()
		.add_filter("Видеофайл", &AVAILABLE_FILES_EXT)
		.set_directory("/")
		.pick_files()
		.unwrap_or_default();
	for file in added_files.iter() {
		if !files.iter().any(|item| &item.file == file) {
			files.push(FileItem{file: file.clone(), is_active: true, status: Status::NotActive});
		}
	}
	files.clone()
}
pub fn run_convert(selected_ext: &Option<&'static str>, files: &[FileItem]) -> Vec<Status>{
	let mut files2: Vec<Status> = Vec::new();
	for FileItem{file, is_active, status} in files.iter() {
        if *is_active && status == &Status::NotActive {
			let ext = selected_ext.unwrap_or("error");
			files2.push(Status::InProcess);
       		let result = Command::new("ffmpeg")
				.arg("-y")
    			.arg("-loglevel").arg("quiet")
				.arg("-threads").arg("0")
				.arg("-i").arg(file)
				.arg(file.with_extension(ext).to_string_lossy().as_ref())
				.stdout(Stdio::null())
				.stderr(Stdio::null())
				.status();
				files2.pop(); files2.push( 
					if result.is_ok_and(|file_status| file_status.success()) {Status::Success}
					else {Status::Error});}
		else{files2.push(Status::NotActive);}
		}
	files2
}
