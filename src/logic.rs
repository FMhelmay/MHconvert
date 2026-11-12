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


pub fn check_ffmpeg() -> bool {
	Command::new("ffmpeg").arg("-version").output().map(|output| output.status.success()).unwrap_or(false)
}

pub fn clear_selected(files:&Vec<FileItem>) -> Vec<FileItem> {
	let mut new_vector:Vec<FileItem> = Vec::new();
	for file in files {
		if file.is_active {
			new_vector.push(file.clone())
		}
	}
	new_vector
}

pub fn add_stamp(file_name: &str, format: &str) -> String {
	let now_time = Local::now().format(format).to_string();
	format!("{}___{}", file_name, now_time)

}
pub fn select_files() -> (Vec<PathBuf>, Vec<bool>){
	let selected_files = FileDialog::new()
		.add_filter("Видеофайл", &AVAILABLE_FILES_EXT)
		.set_directory("/")
		.pick_files()
		.unwrap_or_default();
	let selected_files_bool_len = vec![true; selected_files.len()];
	(selected_files, selected_files_bool_len)
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
