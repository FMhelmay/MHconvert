use std::path::PathBuf;
use rfd::FileDialog;

use iced::widget::{text, button, pick_list, checkbox};
use iced::widget::{Container, container, center};
use iced::widget::{Column, column, Row, row};
use iced::Fill;

use crate::logic;


#[derive(Clone)]
enum ErrorMessage {
	CloseWindow
}

#[derive(Debug, Clone)]
enum MainMessage {
	RunConvert,
	SelectFiles,
	ExtSelected(&'static str),
	CheckboxFileToggled(usize, bool)
}


pub fn draw_main_window() -> iced::Result {
	iced::run(MHconvertMainState::update, MHconvertMainState::view)
}

pub fn draw_error_window(/*window_name: &str, */window_text: &str, window_button_text: &str) -> iced::Result {
	let state = MHconvertErrorState {/*window_name: window_name.to_string(), */window_text: window_text.to_string(), window_button_text: window_button_text.to_string()};
	iced::application(
		move || state.clone(),
			MHconvertErrorState::update,
			MHconvertErrorState::view
	)
	.window_size((500.0, 200.0))
	.centered()
	.resizable(false)
	.run()
}


#[derive(Clone)]
struct MHconvertErrorState {
	//window_name: String,
	window_text: String,
	window_button_text: String
}

#[derive(Clone, Default)]
struct MHconvertMainState {
	selected_ext: Option<&'static str>,
	selected_files: Vec<PathBuf>,
	active_files: Vec<bool>
}

// == == == == ERROR WINDOW
impl MHconvertErrorState {
	fn update(&mut self, message: ErrorMessage) {
		match message{
			ErrorMessage::CloseWindow => {
				std::process::exit(0)
			}
		}
	}

	fn view(&self) -> Container<'_, ErrorMessage> {
		let element_window_text = text(&self.window_text);
		let element_window_button = button(text(&self.window_button_text)).on_press(ErrorMessage::CloseWindow);
		center(column![element_window_text, element_window_button.width(Fill)].spacing(15)).padding(20)
			.width(Fill)
			.height(Fill)
	}
}
// == == == == MAIN WINDOW
impl MHconvertMainState {
    fn update(&mut self, message: MainMessage) {
        match message{
        	MainMessage::SelectFiles => {logic::select_files();}
        	MainMessage::RunConvert => {
        		logic::run_convert(
        			&self.selected_ext,
        			&self.active_files.clone(),
        			&self.selected_files.clone()
        		);
        	}
            MainMessage::ExtSelected(ext) => {self.selected_ext = Some(ext);}
			MainMessage::CheckboxFileToggled(index, is_selected) => {self.active_files[index] = is_selected;}
		}
	}

	fn view(&self) -> Row<'_, MainMessage> {
    	let is_allowed_convert: bool = self.selected_ext.is_some() && self.active_files.contains(&true);
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(MainMessage::SelectFiles),
    		pick_list(
        		logic::AVAILABLE_FILES_EXT,
        		self.selected_ext,
				MainMessage::ExtSelected)
				.placeholder("Выберите расширение"),
			button("Конвертать").on_press_maybe(if is_allowed_convert {Some(MainMessage::RunConvert)} else {None})
		];
		let mut file_check = column![];
			for (file_index, file_path) in self.selected_files.iter().enumerate() {
            	file_check = file_check.push(
            	checkbox(
            	*self.active_files
            	.get(file_index)
            	.unwrap_or(&false))
						.label(file_path.to_string_lossy())
						.on_toggle(move | is_selected | MainMessage::CheckboxFileToggled(file_index, is_selected))
				).spacing(5);
			}
		row![btn_picklist, file_check]

	}
}
