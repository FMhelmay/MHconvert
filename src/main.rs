use std::process::Command;

fn main() {
    let files: [&str; 3] = ["/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Звезда__2026-09-30-18-33-08.webm",
       				  	    "/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Нравится__2026-09-30-18-32-37.webm",
    				        "/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Сердце__2026-09-30-18-32-30.webm"];
    let status = Command::new("sh").arg("-c").current_dir("/home/mplhelmay/Documents/r").arg("ls").status().expect("Не удалось запустить процесс");
	println!("{}", status);
	let task: &str = "mp4";

	for file in files {
		let file0cut: &str = file.split('/').next_back().unwrap().split('.').next().unwrap();
		println!("{}.{}", file0cut, task);
		//filename with mp4 type
		
	}
}
