use std::process::Command;

fn main() {
    let mut files = vec!["/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Звезда__2026-09-30-18-33-08.webm",
       				  	 "/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Нравится__2026-09-30-18-32-37.webm",
    				     "/home/mplhelmay/Documents/test/tz3_testtttt/Коденька_Уведомление _Сердце__2026-09-30-18-32-30.webm"];
	let files_count: i16 = files.len();
    println!("{}", files_count);
    let status = Command::new("sh").arg("-c").current_dir("/home/mplhelmay/Documents/r").arg("ls").status().expect("Не удалось запустить процесс");
	println!("{}", status[0]);
}
