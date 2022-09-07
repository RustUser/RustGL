
use imgui::{ClipboardBackend, ImStr, ImString};

pub struct ClipboardSupport(pub ClipboardContext);

pub fn init() -> Option<ClipboardSupport> {
    ClipboardContext::new().ok().map(ClipboardSupport)
}

impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<ImString> {
        match self.0.get_contents() {
            Ok(value) => {
                Some(ImString::new(value))
            }
            Err(_) => {
                None
            }
        }
    }

    fn set(&mut self, value: &ImStr) {
        let _ = self.0.set_contents(value.to_string());
    }
}