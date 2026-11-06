use std::process::Command;

mod logic;
mod gui;


fn main() -> iced::Result {
	if !Command::new("ffmpeg").arg("-version").output().map(|output| output.status.success()).unwrap_or(false) {		
		gui::draw_main_window()
	}
	else {
		gui::draw_error_window(
			"MHconvert",
			"В вашей системе не обнаружен ffmpeg. Вы можете проверить PATH или скачать его на ffmpeg.org",
			"Закрыть"
		)
	}

}
