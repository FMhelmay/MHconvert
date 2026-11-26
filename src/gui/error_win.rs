use iced::widget::{text, button};
use iced::widget::{Container, center};
use iced::widget::{column, row};
use iced::{Task, Fill};
use iced::window;

#[derive(Clone)]
pub enum ErrorMessage {
	CloseWindow
}

#[derive(Clone)]
pub struct MHconvertErrorState {
	pub window_name: String,
	pub window_text: String,
	pub window_button_text: String
}

impl MHconvertErrorState {
	pub fn get_title(&self, window_id: window::Id) -> String {
		window_id.to_string()
	}
	pub fn boot(self) -> (Self, Task<ErrorMessage>) {
		let (article, art2) = window::open(window::Settings::default());
		println!("{}", article);
		println!("{:?}", art2);
		(self, art2.discard())
		
	}
	pub fn update(&mut self, message: ErrorMessage) {
		match message{
			ErrorMessage::CloseWindow => {
				std::process::exit(0)
			}
		}
	}

	pub fn view(&self, _window_id: window::Id) -> Container<'_, ErrorMessage> {
		let element_window_text = text(&self.window_text);
		let element_window_button = button(text(&self.window_button_text).width(Fill).center()).on_press(ErrorMessage::CloseWindow);
		center(column![
				element_window_text.width(Fill).height(Fill),
				row![
					text("").width(Fill),
					text("").width(Fill),
					text("").width(Fill),
					text("").width(Fill),
					element_window_button.width(Fill)
				]
			]
			.spacing(15))
			.padding(20)
			.width(Fill)
			.height(Fill)
	}
}
