use std::path::PathBuf;
use rfd::FileDialog;
use iced::widget::{button, pick_list, checkbox, text};
use iced::widget::{column, row, Column};
//use iced::Element;


const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "raw"];


pub fn main() -> iced::Result {
    iced::run(MHconvert::update, MHconvert::view)
}


#[derive(Default)]
struct MHconvert {
	selected_ext: Option<&'static str>,
	selected_files: Vec<PathBuf>,
	is_file_selected: bool
}

#[derive(Debug, Clone)]
enum Message {
	SelectFiles,
	FilesSelected(Vec<PathBuf>),
	ExtSelected(&'static str),
	CheckboxFile0Toggled(bool)
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
			Message::FilesSelected(_) => {}
			Message::CheckboxFile0Toggled(is_toggled) => {
				self.is_file_selected = is_toggled;
			}
		}
	}

	fn view(&self) -> Column<'_, Message> {
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(Message::SelectFiles),
    		pick_list(
        		AVAILABLE_FILES_EXT,
        		self.selected_ext,
				Message::ExtSelected)
				.placeholder("Выберите расширение")
		];
		let file_check = row![
			if self.selected_files.first().is_some() {
            	checkbox(self.is_file_selected)
            		.label(self.selected_files[0].to_string_lossy())
            	    .on_toggle(Message::CheckboxFile0Toggled)
			}
			else {
				checkbox(self.is_file_selected)
					.label("Файлы не выбраны")
			}
		];
		column![
			btn_picklist,
			file_check
		]
	}

}
