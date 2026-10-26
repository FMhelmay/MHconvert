use std::process::Command;

fn main() {
    let mut files = vec!["/home/mplhelmay/Documents/test/tz3_testtttt/file144.webm",
       				  	 "/home/mplhelmay/Documents/test/tz3_testtttt/file212.webm",
    				     "/home/mplhelmay/Documents/test/tz3_testtttt/file323.webm"];
	let files_count: i16 = files.len() as i16;
    println!("{}", files_count);
    let status = Command::new("sh").arg("-c").current_dir("/home/mplhelmay/Documents/r").arg("ls").status().expect("Не удалось запустить процесс");
	println!("{}", status);
}
