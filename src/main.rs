use std::process::Command;
use std::path::PathBuf;
use rfd::FileDialog;


fn main() {
    let files: Vec<PathBuf> = select_files().unwrap();
    let status = Command::new("sh").arg("ls").status().expect("Не удалось запустить процесс");
	println!("{}", status);
	let task: &str = "mp4";

	for file in files {
		//let file_sf = Path::new(file);
		//let file0path: &str = file_sf.parent().unwrap().to_str().unwrap();
		//println!("file path: {}", file0path);

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
