use std::process::Command;
use std::path::PathBuf;
use std::io;
use rfd::FileDialog;


fn main() {
    let files: Vec<PathBuf> = select_files().unwrap();
    let status = Command::new("sh").arg("ls").status().expect("Не удалось запустить процесс");
	println!("{}", status);
	
	//let task: &str = "mp4";
	let mut task = String::new();
	println!("Select file type number (1-7): ");
	io::stdin()
		.read_line(&mut task)
		.expect("Ничего не введено");
	
	for file in files {
		let file0name: &str = file.to_str().unwrap().split('.').next().unwrap();
		let file0name2: String = format!("{}.{}", file0name, task);
		Command::new("ffmpeg").arg("-i").arg(file).arg(file0name2).status().expect("Не удалось запустить процесс");
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
