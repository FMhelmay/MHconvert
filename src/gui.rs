use std::process::{Command, Stdio};
use std::path::PathBuf;
use rfd::FileDialog;

use iced::widget::{container, text, button, pick_list, checkbox};
use iced::widget::{Container, Column, Row, column, row};
use iced::Fill;
use crate::logic;


const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav"];


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

#[derive(Default)]
struct MHconvertMainState {
	selected_ext: Option<&'static str>,
	selected_files: Vec<PathBuf>,
	active_files: Vec<bool>
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

impl MHconvertMainState {
    fn update(&mut self, message: MainMessage) {
        match message{
        	MainMessage::SelectFiles => {
        		self.selected_files = FileDialog::new()
					.add_filter("Видеофайл", &AVAILABLE_FILES_EXT)
					.set_directory("/")
					.pick_files()
					.unwrap_or_default();
				self.active_files = vec![true; self.selected_files.len()];
        	}
        	MainMessage::RunConvert => {
        		for (file_index, file_path) in self.selected_files.iter().enumerate() {
        			if self.active_files[file_index] {
        				println!("{:?}", file_path);
						let ext = self.selected_ext.unwrap_or("error");
						let file2 = file_path.with_extension(ext);
        				
        				println!("{:?}", file2);
        				Command::new("ffmpeg")
							.arg("-y")
	    					.arg("-loglevel").arg("quiet")
							.arg("-threads").arg("0")
							.arg("-i").arg(file_path)
							.arg(file2)
							.stdout(Stdio::null())
							.stderr(Stdio::null())
							.status()
							.expect("Не удалось запустить процесс");
        			}
        		}
        	}
            MainMessage::ExtSelected(ext) => {self.selected_ext = Some(ext);}
			MainMessage::CheckboxFileToggled(index, is_selected) => {self.active_files[index] = is_selected;}
		}
	}

	fn view(&self) -> Container<'_, MainMessage> {
    	let is_allowed_convert: bool = self.selected_ext.is_some() && self.active_files.contains(&true);
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(MainMessage::SelectFiles),
    		pick_list(
        		AVAILABLE_FILES_EXT,
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
		container(row![btn_picklist, file_check]).center(Fill)
	}
}
