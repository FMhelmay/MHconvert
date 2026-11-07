mod logic;
mod gui;


fn main() -> iced::Result {
	if logic::check_ffmpeg() {		
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
