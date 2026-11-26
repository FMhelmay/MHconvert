mod error_win;
mod main_win;


pub fn draw_main_window() -> iced::Result {
	iced::run(main_win::MHconvertMainState::update, main_win::MHconvertMainState::view)
}

pub fn draw_error_window(window_name: &str, window_text: &str, window_button_text: &str) -> iced::Result {
	let state = error_win::MHconvertErrorState {
		window_name: window_name.to_string(),
		window_text: window_text.to_string(),
		window_button_text: window_button_text.to_string()
		
	};
	iced::daemon(
		move || state.clone().boot(),
		error_win::MHconvertErrorState::update,
		error_win::MHconvertErrorState::view
	)
	.title(error_win::MHconvertErrorState::get_title)
	//.window_size((500.0, 150.0))
	//.centered()
	//.resizable(false)
	.run()
}
