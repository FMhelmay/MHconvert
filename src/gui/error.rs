#[derive(Clone)]
pub enum ErrorMessage {
	CloseWindow
}

#[derive(Clone)]
pub struct MHconvertErrorState {
	window_name: String,
	window_text: String,
	window_button_text: String
}

pub impl MHconvertErrorState {
	pub fn get_title(&self) -> String {
		self.window_name.clone()
	}
	pub fn update(&mut self, message: ErrorMessage) {
		match message{
			ErrorMessage::CloseWindow => {
				std::process::exit(0)
			}
		}
	}

	pub fn view(&self) -> Container<'_, ErrorMessage> {
		let element_window_text = text(&self.window_text);
		let element_window_button = button(text(&self.window_button_text).width(Fill).center()).on_press(ErrorMessage::CloseWindow);
		center(column![
				element_window_text.width(Fill).height(Fill),
				row![text("").width(Fill), text("").width(Fill), text("").width(Fill), text("").width(Fill),
				element_window_button.width(Fill)]]
			.spacing(15))
			.padding(20)
			.width(Fill)
			.height(Fill)
	}
}
