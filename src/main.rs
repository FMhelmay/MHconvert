use std::process::{Command, Stdio};
use std::path::PathBuf;
use rfd::FileDialog;
use iced::widget::{button, pick_list, checkbox};
use iced::widget::{column, row};
use iced::widget::Row;
//use iced::widget::Column;


const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav"];


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
	RunConvert,
	SelectFiles,
	ExtSelected(&'static str),
	CheckboxFileToggled(usize, bool)
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
				self.active_files = vec![true; self.selected_files.len()];
        	}
        	Message::RunConvert => {
        		for (file_index, file_path) in self.selected_files.iter().enumerate() {
        			if self.active_files[file_index] {
        				println!("{:?}", file_path);
        				let file2: &str = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("error1");
        				println!("{}.{:?}", file2, self.selected_ext.unwrap_or("error2"));
        				Command::new("ffmpeg")
							.arg("-y")
	    					.arg("-loglevel").arg("quiet")
							.arg("-threads").arg("0")
							.arg("-i").arg(file_path)
							.arg(format!("{}.{}", file2, self.selected_ext.unwrap_or("error2")))
							//.stdout(Stdio::null())
							//.stderr(Stdio::null())
							.status()
							.expect("Не удалось запустить процесс");
        			}
        		}
        	}
            Message::ExtSelected(ext) => {self.selected_ext = Some(ext);}
			Message::CheckboxFileToggled(index, is_selected) => {self.active_files[index] = is_selected;}
		}
	}

	fn view(&self) -> Row<'_, Message> {
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(Message::SelectFiles),
    		pick_list(
        		AVAILABLE_FILES_EXT,
        		self.selected_ext,
				Message::ExtSelected)
				.placeholder("Выберите расширение"),
			button("Конвертать").on_press(Message::RunConvert)
		];
		let mut file_check = column![];
			for (file_index, file_path) in self.selected_files.iter().enumerate() {
            	file_check = file_check.push(
            	checkbox(
            	*self.active_files
            	.get(file_index)
            	.unwrap_or(&false))
						.label(file_path.to_string_lossy())
						.on_toggle(move | is_selected | Message::CheckboxFileToggled(file_index, is_selected))
				).spacing(5);
			}
		row![btn_picklist, file_check]
	}
}
