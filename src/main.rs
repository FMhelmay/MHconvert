use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::io;
use rfd::FileDialog;


fn main() {

	iced::application("Video Converter", Converter::update, Converter::view)
	    .theme(|_| Theme::Dark)   // или свой
	    .run();




	let output_user = select_files().unwrap();
    let files: Vec<PathBuf> = output_user;
	println!("{:?}", files);
	let mut u_input_task = String::new();
	println!("Select file type (webm/mp4/mkv/mov/avi/ogg/raw): ");
	io::stdin()
		.read_line(&mut u_input_task)
		.expect("Ничего не введено");
	let task: &str = u_input_task.trim();
	for file in files {
		let file0name: &str = file.to_str().unwrap().split('.').next().unwrap();
		let file0name2: String = format!("{}.{}", file0name, task);
		Command::new("ffmpeg")
					.arg("-y")
					.arg("-loglevel").arg("quiet")
					.arg("-threads").arg("0")
					.arg("-i").arg(&file)
					.arg(file0name2)
					.stdout(Stdio::null())
					.stderr(Stdio::null())
					.status()
					.expect("Не удалось запустить процесс");
	}
}

fn select_files() -> Result<Vec<PathBuf>, i32> {
	let files = FileDialog::new()
        .add_filter("Видеофайл", &["webm", "mp4", "mkv", "mov", "avi", "ogg", "raw"])
        .set_directory("/")
        .pick_files();

    if let Some(path) = files {
        Ok(path)
    } else {
        Err(0)
    }
}
