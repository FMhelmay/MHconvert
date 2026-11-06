use iced::widget::{text, button};
use iced::widget::{Column, column};

use crate::logic;

#[derive(Clone)]
enum ErrorMessage {
	CloseWindow
}


pub fn draw_main_window(window_name: &str)  -> iced::Result {
	println!("GUI,DRAW_MAIN_WINDOW: создано окно {}", window_name);
	let state = MHconvertErrorState {window_name: window_name.to_string(), window_text: "Не указано".to_string(), window_button_text: "Не указано".to_string()};
	iced::application(
		move || state.clone(),
			MHconvertErrorState::update,
			MHconvertErrorState::view
	).run()
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
	fn update(&mut self, message: ErrorMessage) {
		match message{
			ErrorMessage::CloseWindow => {
				std::process::exit(0)
			}
		}
	}
	fn view(&self) -> Column<'_, ErrorMessage> {
		let element_window_text = text(&self.window_text);
		let element_window_button = button(text(&self.window_button_text)).on_press(ErrorMessage::CloseWindow);
		column![element_window_text, element_window_button]
	}
}
