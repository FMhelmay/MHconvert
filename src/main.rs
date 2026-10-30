use std::path::PathBuf;
use rfd::FileDialog;
use iced::widget::{button, pick_list, column, Column};
use iced::Element;


const Available_files_ext: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "raw"];


pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}


#[derive(Default)]
struct Counter {
	selected_ext: Option<&'static str>,
	selected_files: Vec<PathBuf>
}

#[derive(Debug, Clone)]
enum Message {
	SelectFiles,
	FilesSelected(Vec<PathBuf>),
	ExtSelected(&'static str)
	
}


impl Counter {
    fn update(&mut self, message: Message) {
        match message{
        	Message::SelectFiles => {
        		self.selected_files = FileDialog::new()
					.add_filter("Видеофайл", &["webm", "mp4", "mkv", "mov", "avi", "ogg", "raw"])
					.set_directory("/")
					.pick_files()
					.unwrap_or_default();
        	}
            Message::ExtSelected(ext) => {
                println!("{}",ext);
                self.selected_ext = Some(ext);
			}
			Message::FilesSelected(_) => {
				
			}
		}
	}

	fn view(&self) -> Column<'_, Message> {
    	column![
    		button("Выбрать файлы").on_press(Message::SelectFiles),
    		pick_list(
        		Available_files_ext,
        		self.selected_ext,
				Message::ExtSelected)
				.placeholder("Выберите расширение")
		]
	}

}
