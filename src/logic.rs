use std::process::{Command, Stdio};
use chrono::{Local};
use std::path::PathBuf;
use rfd::FileDialog;


pub const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav"];

 
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
	let flen = file_path.len();
	println!("flen: {}", flen);
	println!("finaly_count: {}", finaly_count);
	println!("file_name: {}", file_path);
	if flen <= finaly_count {
		file_path.to_string()
	}
	else {
		format!("...{}", &file_path[flen-finaly_count+3..flen])
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

pub fn add_stamp(file_name: &str, format: &str) -> String {
	let now_time = Local::now().format(format).to_string();
	format!("{}___{}", file_name, now_time)

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
			let file2 = add_stamp(
				file
				.with_extension("")
				.to_string_lossy()
				.as_ref(),
				"%d.%m.%Y_%H.%M");
			files2.push(Status::InProcess);
       		let result = Command::new("ffmpeg")
				.arg("-y")
    			.arg("-loglevel").arg("quiet")
				.arg("-threads").arg("0")
				.arg("-i").arg(file)
				.arg(format!("{}.{}", file2, ext))
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
