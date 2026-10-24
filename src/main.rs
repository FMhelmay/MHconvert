use std::process::Command;

fn main() {
    println!("Hello, world!");
    let status = Command::new("sh").arg("-c").current_dir("/home/mplhelmay/Documents/r").arg("ls").status().expect("Не удалось запустить процесс");
    println!("{}", status);
}
