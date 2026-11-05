use iced::widget::{text, button};
use iced::widget::{Column, column};

#[derive(Clone)]
enum Message {
	CloseWindow
}


pub fn draw_main_window(window_name: &str) {
	println!("GUI,DRAW_MAIN_WINDOW: создано окно {}", window_name)
}

pub fn draw_error_window(window_name: &str, window_text: &str, window_button_text: &str) -> iced::Result {
	println!("GUI,DRAW_ERROR_WINDOW: создано окно {}, оно извещает следующее: {}. {}", window_name, window_text, window_button_text);
	let state = MHconvertErrorState {window_name: window_name.to_string(), window_text: window_text.to_string(), window_button_text: window_button_text.to_string()};
	iced::application(
		move || state.clone(),
			MHconvertErrorState::update,
			MHconvertErrorState::view
	).run()
}

#[derive(Clone)]
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
		let element_window_text = text(&self.window_text);
		let element_window_button = button(text(&self.window_button_text)).on_press(Message::CloseWindow);
		column![element_window_text, element_window_button]
	}
}
