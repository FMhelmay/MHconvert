use iced::widget::{text, button};
use iced::widget::{Column, column};



#[derive(Debug, Clone)]
enum Message {
	CloseWindow
}


pub fn draw_main_window(window_name: &str) {
	println!("GUI,DRAW_MAIN_WINDOW: создано окно {}", window_name)
}

pub fn draw_error_window(window_name: &str, window_text: &str, window_button_text: &str) -> iced::Result {
	println!("GUI,DRAW_ERROR_WINDOW: создано окно {}, оно извещает следующее: {}. {}", window_name, window_text, window_button_text);
	iced::run(MHconvertError::update, MHconvertError::view(window_text, window_button_text))
}

#[derive(Default)]
struct MHconvertError {
}


impl MHconvertError {
	fn update(&mut self, message: Message) {
		match message{
			Message::CloseWindow => {
				std::process::exit(0)
			}
		}
	}
	fn view(window_text: &str, window_button_text: &str) -> Column<Message> {
		let window_text = text(window_text);
		let window_button = button(window_button_text).on_press(Message::CloseWindow);
		column![window_text, window_button]
	}
}
