use std::path::PathBuf;
use rfd::FileDialog;
use iced::widget::{button, pick_list, checkbox};
use iced::widget::{column, row};
use iced::widget::Row;
//use iced::widget::Column;


const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "raw"];


pub fn main() -> iced::Result {
    iced::run(MHconvert::update, MHconvert::view)
}


#[derive(Default)]
struct MHconvert {
	selected_ext: Option<&'static str>,
	selected_files: Vec<PathBuf>,
	active_files: Vec<bool>
}

#[derive(Debug, Clone)]
enum Message {
	SelectFiles,
	ExtSelected(&'static str),
	CheckboxFileToggled(&'static PathBuf)
}


impl MHconvert {
    fn update(&mut self, message: Message) {
        match message{
        	Message::SelectFiles => {
        		self.selected_files = FileDialog::new()
					.add_filter("Видеофайл", &AVAILABLE_FILES_EXT)
					.set_directory("/")
					.pick_files()
					.unwrap_or_default();
				println!("{:?}", self.selected_files);
        	}
            Message::ExtSelected(ext) => {
                println!("{}",ext);
                self.selected_ext = Some(ext);
			}
			Message::CheckboxFileToggled(file) => {
				self.active_files[file] = true;
			}
		}
	}

	fn view(&self) -> Row<'_, Message> {
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(Message::SelectFiles),
    		pick_list(
        		AVAILABLE_FILES_EXT,
        		self.selected_ext,
				Message::ExtSelected)
				.placeholder("Выберите расширение")
		];
		let mut file_check = column![];
			for file in &self.selected_files {
            	file_check = file_check.push(checkbox(&self.active_files[file])
					.label(file.to_string_lossy())
					.on_toggle(Message::CheckboxFileToggled(file))
				);
			}
		row![
			btn_picklist,
			file_check
		]
	}

}
