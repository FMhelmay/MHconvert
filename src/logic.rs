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

#[derive(Clone, Default, PartialEq)]
pub enum Status {
	Success,
	Error,

	InProcess,
	#[default]
	NotActive
}


pub fn add_stamp(file_name: &str, format: &str) -> String {
	let now_time = Local::now().format(format).to_string();
	format!("{}___{}", file_name, now_time)
}

pub fn check_ffmpeg() -> bool {
	Command::new("ffmpeg").arg("-version").output().map(|output| output.status.success()).unwrap_or(false)
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
pub fn run_convert(selected_ext: &Option<&'static str>, files: Vec<FileItem>) {
	let mut files2: Vec<Status> = Vec::new();
	for (_file_index, FileItem{file, is_active, status}) in files.iter().enumerate() {
        if *is_active && status == &Status::NotActive {
			let ext = selected_ext.unwrap_or("error");
			let file2 = add_stamp(
				file
				.with_extension("")
				.to_string_lossy()
				.as_ref(),
				"%d.%m.%Y_%H.%M");
			files2.push(Status::InProcess);
       		Command::new("ffmpeg")
				.arg("-y")
    			.arg("-loglevel").arg("quiet")
				.arg("-threads").arg("0")
				.arg("-i").arg(file)
				.arg(format!("{}.{}", file2, ext))
				.stdout(Stdio::null())
				.stderr(Stdio::null())
				.status()
				.expect("Не удалось запустить процесс");
		}
	}
}

