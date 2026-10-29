use iced::widget::{/*button, column, text, */pick_list/*, Column*/};
use iced::Element;

const Availible_files_ext: [&str; 7] = ["webm", "mp4", "mkv", "mov", "avi", "ogg", "raw"];

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    selected_ext: Option<&'static str>
}

#[derive(Debug, Clone)]
enum Message {
    ExtSelected(&'static str)
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message{
            Message::ExtSelected(ext) => {
                println!("{}",ext);
                self.selected_ext = Some(ext); 
			}
        }
    }

    fn view(&self) -> Element<Message> {
        pick_list(
        	Availible_files_ext,
        	self.selected_ext,
        	Message::ExtSelected)
        	.placeholder("Выберите расширение...")
        	.into()
    }
}
