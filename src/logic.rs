use std::process::{Command, Stdio};
use chrono::{Local};
use std::path::PathBuf;
use rfd::FileDialog;


pub const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav"];

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
pub fn run_convert(selected_ext: &Option<&'static str>, active_files: &[bool], selected_files: &[PathBuf]) {
	for (file_index, file_path) in selected_files.iter().enumerate() {
        if active_files[file_index] {
			let ext = selected_ext.unwrap_or("error");
			let file2 = format!("{}.{}", add_stamp(file_path.to_string_lossy().as_ref(), "%d.%m.%Y_%H.%M"), ext);
       			Command::new("ffmpeg")
					.arg("-y")
    				.arg("-loglevel").arg("quiet")
					.arg("-threads").arg("0")
					.arg("-i").arg(file_path)
					.arg(file2)
					.stdout(Stdio::null())
					.stderr(Stdio::null())
					.status()
					.expect("Не удалось запустить процесс");
		}
	}
}
