use std::process::{Command, Stdio};
use std::path::PathBuf;
use rfd::FileDialog;
use iced::widget::{button, pick_list, checkbox};
use iced::widget::{column, row};
use iced::widget::Row;

use iced::widget::{container, text};
use iced::{Center, Element, Fill};

const AVAILABLE_FILES_EXT: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "wav"];


struct FileItem {
	path: PathBuf,
	is_active: bool,
	status: FileStatus
}

enum FileStatus {
	Waiting, Done, Error, None
}
fn check_ffmpeg() -> bool {
	Command::new("ffmpeg").arg("-version").output().map(|output| output.status.success()).unwrap_or(false)
}
pub fn main() -> iced::Result {
    if !check_ffmpeg() {
        return show_notice(
            "MHconvert: ffmpeg не найден",
            "В вашей системе не обнаружен ffmpeg. Вы можете проверить PATH или скачать его на ffmpeg.org",
            "Закрыть",
        );
    }

    iced::application(MHconvert::default, MHconvert::update, MHconvert::view)
        .title("MHconvert")
        .window_size((900.0, 600.0))
        .centered()
        .run()
}

struct NoticeWindow {
    title: String,
    text: String,
    button_text: String,
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
	CheckboxFileToggled(usize, bool),
    CloseWindow
}


#[allow(non_snake_case)]
impl NoticeWindow {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn updateNo_ffmpeg(&mut self, message: Message) {
        match message {
            Message::CloseWindow => {
                std::process::exit(0);
            }
        }
    }

    fn viewNo_ffmpeg(&self) -> Element<'_, Message> {
        let content = column![
            text(&self.text).align_x(Center),
            button(text(&self.button_text))
                .padding([8, 24])
                .on_press(Message::CloseWindow),
        ]
        .spacing(20)
        .align_x(Center);

        container(content).center(Fill).padding(20).into()
    }
}

/// Показывает маленькое окно с текстом и одной кнопкой закрытия.
fn show_notice(title: &str, message: &str, button_text: &str) -> iced::Result {
    let (title, message, button_text) =
        (title.to_string(), message.to_string(), button_text.to_string());

    iced::application(
        move || NoticeWindow {
            title: title.clone(),
            text: message.clone(),
            button_text: button_text.clone(),
        },
        NoticeWindow::updateNo_ffmpeg,
        NoticeWindow::viewNo_ffmpeg,
    )
    .title(NoticeWindow::title)
    .window_size((650.0, 350.0))
    .centered()
    .resizable(false)
    .run()
}

impl MHconvert {
	fn update(&mut self, message: Message) {
		match message {
			Message::CloseWindow => {
				std::process::exit(0);
			}
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
            Message::ExtSelected(ext) => {self.selected_ext = Some(ext);}
			Message::CheckboxFileToggled(index, is_selected) => {self.active_files[index] = is_selected;}
		}
	}
	fn view(&self) -> Row<'_, Message> {
    	let is_allowed_convert: bool = self.selected_ext.is_some() && self.active_files.contains(&true);
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(Message::SelectFiles),
    		pick_list(
        		AVAILABLE_FILES_EXT,
        		self.selected_ext,
				Message::ExtSelected)
				.placeholder("Выберите расширение"),
			button("Конвертать").on_press_maybe(if is_allowed_convert {Some(Message::RunConvert)} else {None})
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
