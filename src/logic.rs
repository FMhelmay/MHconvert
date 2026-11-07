use std::process::{Command, Stdio};
use std::path::PathBuf;


pub const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav"];


pub fn check_ffmpeg() -> bool{
	Command::new("ffmpeg").arg("-version").output().map(|output| output.status.success()).unwrap_or(false)
}

pub fn run_convert(selected_ext: Option<&'static str>, active_files: Vec<bool>, selected_files: Vec<PathBuf>) {
	for (file_index, file_path) in selected_files.iter().enumerate() {
        if active_files[file_index] {
			let ext = selected_ext.unwrap_or("error");
				let file2 = file_path.with_extension(ext);

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
