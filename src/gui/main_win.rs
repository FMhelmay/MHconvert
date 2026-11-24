use iced::widget::{text, button, pick_list, checkbox, tooltip};
use iced::widget::{/*Container, */container/*, center*/};
use iced::widget::{column, Row, row};
//use iced::Fill;
use std::time::Duration;

use crate::logic;


#[derive(Clone, Default)]
pub struct MHconvertMainState {
	selected_ext: Option<&'static str>,
	files: Vec<logic::FileItem>
}


#[derive(Debug, Clone)]
pub enum MainMessage {
	CheckboxFileToggled(usize, bool),
	ExtSelected(&'static str),
	ClearSelected,
	AddFiles,
	RunConvert,
	SelectAll
}

impl MHconvertMainState {
    pub fn update(&mut self, message: MainMessage) {
        match message{
        	MainMessage::AddFiles => {
        		logic::add_files(&mut self.files);
        	}
        	MainMessage::SelectAll => {
        		self.files = logic::select_all(&self.files);
        	}
        	MainMessage::RunConvert => {
        		let new_statuses = logic::run_convert(
					&self.selected_ext,
        		    &self.files
				);
				if !new_statuses.is_empty() {
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

	pub fn view(&self) -> Row<'_, MainMessage> {
    	let is_selected_ext: bool = self.selected_ext.is_some();
    	let is_selected_files: bool = self.files.iter().any(|item| item.is_active);
    	let is_allowed_convert: bool = is_selected_ext && is_selected_files;
    	let btn_picklist = column![
    		button("Выбрать файлы").on_press(MainMessage::AddFiles),
    		pick_list(
        		logic::AVAILABLE_FILES_EXT,
        		self.selected_ext,
				MainMessage::ExtSelected)
				.placeholder("Выберите расширение"),
			button("Конвертать").on_press_maybe(if is_allowed_convert {Some(MainMessage::RunConvert)} else {None}),
			button("Убрать выбранные").on_press_maybe(if is_selected_files {Some(MainMessage::ClearSelected)} else {None}),
			button("Выбрать все").on_press_maybe(if !self.files.is_empty() {Some(MainMessage::SelectAll)} else {None})

		];
		let mut tooliped_checkboxes = column![];
		for (file_index, logic::FileItem{file, is_active, status}) in self.files.iter().enumerate() {
				let file_path = file.to_string_lossy();
       			let checkbox = (checkbox(*is_active)
					.label(logic::cut_path(&file_path, 50))
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
			tooliped_checkboxes = tooliped_checkboxes.push(
				tooltip(
					checkbox,
					container(
						text(file_path))
						.style(container::rounded_box),
					tooltip::Position::Right
				).delay(Duration::from_millis(500))
			)
		}
		row![btn_picklist, tooliped_checkboxes].spacing(20)

	}
}
