use std::process::Command;


pub const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav"];


pub fn check_ffmpeg() -> bool{
	Command::new("ffmpeg").arg("-version").output().map(|output| output.status.success()).unwrap_or(false)
} 
