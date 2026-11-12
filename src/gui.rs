use iced::widget::{text, button, pick_list, checkbox};
use iced::widget::{Container, center};
use iced::widget::{column, Row, row};
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
	ClearSelected,
	ExtSelected(&'static str),
	CheckboxFileToggled(usize, bool)
}

pub fn draw_main_window() -> iced::Result {
	iced::run(MHconvertMainState::update, MHconvertMainState::view)
}

pub fn draw_error_window(window_name: &str, window_text: &str, window_button_text: &str) -> iced::Result {
	let state = MHconvertErrorState {window_name: window_name.to_string(), window_text: window_text.to_string(), window_button_text: window_button_text.to_string()};
	iced::application(
		move || state.clone(),
			MHconvertErrorState::update,
			MHconvertErrorState::view
	)
	.title(MHconvertErrorState::get_title)
	.window_size((500.0, 150.0))
	.centered()
	.resizable(false)
	.run()
}


#[derive(Clone)]
struct MHconvertErrorState {
	window_name: String,
	window_text: String,
	window_button_text: String
}

#[derive(Clone, Default)]
struct MHconvertMainState {
	selected_ext: Option<&'static str>,
	files: Vec<logic::FileItem>
}


// == == == == ERROR WINDOW
impl MHconvertErrorState {
	fn get_title(&self) -> String {
		self.window_name.clone()
	}
	fn update(&mut self, message: ErrorMessage) {
		match message{
			ErrorMessage::CloseWindow => {
				std::process::exit(0)
			}
		}
	}

	fn view(&self) -> Container<'_, ErrorMessage> {
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
// == == == == MAIN WINDOW
impl MHconvertMainState {
    fn update(&mut self, message: MainMessage) {
        match message{
        	MainMessage::SelectFiles => {
				let (file_path, is_active) = logic::select_files();
				self.files = file_path
					.into_iter()
					.zip(is_active)
					.map(|(file_path, active)| logic::FileItem{
						file: file_path,
						is_active: active,
						status: logic::Status::NotActive
					})
					.collect();
        	}
        	MainMessage::RunConvert => {
        		let new_statuses = logic::run_convert(
					&self.selected_ext,
        		    &self.files
				);
				if new_statuses.is_empty() {
					println!("new_statuses пустой")
				}
				else {
					let mut new_files:Vec<logic::FileItem> = Vec::new();
	        		for (file_index, logic::FileItem{file, is_active, status: _}) in self.files.iter().enumerate() {
	        			new_files.push(logic::FileItem{file: file.clone(), is_active: *is_active, status:new_statuses[file_index]});
	        		}
	        		self.files = new_files;
	        	}
        	}
            MainMessage::ClearSelected => {self.files = logic::clear_selected(&self.files);}
            MainMessage::ExtSelected(ext) => {self.selected_ext = Some(ext);}
			MainMessage::CheckboxFileToggled(index, is_selected) => {self.files[index].is_active = is_selected;}
		}
	}

	fn view(&self) -> Row<'_, MainMessage> {
    	let is_selected_ext: bool = self.selected_ext.is_some();
    	let is_selected_files: bool = self.files.iter().any(|item| item.is_active);
    	let is_allowed_convert: bool = is_selected_ext && is_selected_files;
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(MainMessage::SelectFiles),
    		pick_list(
        		logic::AVAILABLE_FILES_EXT,
        		self.selected_ext,
				MainMessage::ExtSelected)
				.placeholder("Выберите расширение"),
			button("Конвертать").on_press_maybe(if is_allowed_convert {Some(MainMessage::RunConvert)} else {None}),
			button("Убрать выбранные").on_press_maybe(if is_selected_files {Some(MainMessage::ClearSelected)} else {None})
		];
		let mut file_check = column![];
		for (file_index, logic::FileItem{file, is_active, status}) in self.files.iter().enumerate() {
       			file_check = file_check.push(checkbox(*is_active)
					.label(file.to_string_lossy())
					.on_toggle(move | is_selected | MainMessage::CheckboxFileToggled(file_index, is_selected))
					.style(|theme, cb_status| {
						let base = checkbox::primary(theme, cb_status);
						checkbox::Style{text_color:
							match status {
								logic::Status::Success => Some(theme.extended_palette().success.base.color),
								logic::Status::Error => Some(theme.extended_palette().danger.base.color),
								logic::Status::InProcess => Some(theme.extended_palette().warning.base.color),
								_ => match is_active {
									true => {None},
									_ => Some(theme.extended_palette().secondary.base.color)
								}
							},..base}}).style(
								match status {
									logic::Status::Success => checkbox::success,
									logic::Status::Error => checkbox::danger,
									logic::Status::InProcess => checkbox::secondary,
									_ => checkbox::primary
									}))
										
					.spacing(10);
		}
		row![btn_picklist, file_check].spacing(20)

	}
}
