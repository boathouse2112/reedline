use log::{log, Level, LevelFilter};
use reedline::{
    default_vi_insert_keybindings, default_vi_normal_keybindings, DefaultPrompt, Reedline, Signal,
    ValidationResult, Validator, Vi,
};
use std::io;

pub struct MultilineValidator;

impl Validator for MultilineValidator {
    fn validate(&self, line: &str) -> ValidationResult {
        if line.ends_with('\\') {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Complete
        }
    }
}

fn main() -> io::Result<()> {
    let mut line_editor = Reedline::create()
        .with_validator(Box::new(MultilineValidator))
        .with_edit_mode(Box::new(Vi::new(
            default_vi_insert_keybindings(),
            default_vi_normal_keybindings(),
        )));
    let prompt = DefaultPrompt::default();

    simple_logging::log_to_file("reedline.log", LevelFilter::Debug).unwrap();

    loop {
        let sig = line_editor.read_line(&prompt)?;
        log!(Level::Info, "Signal: {:?}", sig);
        match sig {
            Signal::CtrlC | Signal::CtrlD => {
                break Ok(());
            }
            _ => {}
        }
    }
}
