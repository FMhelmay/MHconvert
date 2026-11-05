use iced::widget::{text, button};
use iced::widget::{Column, column};


enum Message {
	CloseWindow
}


pub fn draw_main_window(window_name: &str) {
	println!("GUI,DRAW_MAIN_WINDOW: создано окно {}", window_name)
}

pub fn draw_error_window(window_name: &str, window_text: &str, window_button_text: &str) -> iced::Result {
	println!("GUI,DRAW_ERROR_WINDOW: создано окно {}, оно извещает следующее: {}. {}", window_name, window_text, window_button_text);
	iced::run(MHconvertErrorState::update, MHconvertErrorState::view)
}

//#[derive(Default, Clone)]
struct MHconvertErrorState {
	window_name: String,
	window_text: String,
	window_button_text: String
}


impl MHconvertErrorState {
	fn update(&mut self, message: Message) {
		match message{
			Message::CloseWindow => {
				std::process::exit(0)
			}
		}
	}
	fn view(&self) -> Column<'_, Message> {
		let window_text = text(&self.window_text);
		let window_button = button(&self.window_button_text).on_press(Message::CloseWindow);
		column![window_text, window_button]
	}
}
