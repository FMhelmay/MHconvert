use std::process::Command;
//use std::path::Path;


fn main() {
    let files: [&str; 3] = ["/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Звезда__2026-09-30-18-33-08.webm",
       				  	    "/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Нравится__2026-09-30-18-32-37.webm",
    				        "/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Сердце__2026-09-30-18-32-30.webm"];
    let status = Command::new("sh").arg("ls").status().expect("Не удалось запустить процесс");
	println!("{}", status);
	let task: &str = "mp4";

	for file in files {
		//let file_sf = Path::new(file);
		//let file0path: &str = file_sf.parent().unwrap().to_str().unwrap();
		//println!("file path: {}", file0path);

		let file0name: &str = file.split('.').next().unwrap();
		let command: String = format!("ffmpeg -i \"{}\" \"{}.{}\"", file, file0name, task);
		Command::new("sh").arg(command).status().expect("Не удалось запустить процесс");
	}
}
