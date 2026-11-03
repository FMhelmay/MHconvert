pub fn draw_main_window(window_name: &str) {
	println!("GUI,DRAW_MAIN_WINDOW: создано окно {}", window_name)
}



pub fn draw_error_window(window_name: &str, window_text: &str, window_button_text: &str) {
	println!("GUI,DRAW_ERROR_WINDOW: создано окно {}, оно извещает следующее: {}. {}", window_name, window_text, window_button_text)
}
