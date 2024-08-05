use crate::driver::{self, error::WindowHandlerError};

pub struct Window {
    handler: driver::WindowHandler,
}

#[derive(Debug)]
pub enum WindowError {
    WindowHandlerError(WindowHandlerError),
}

pub type WindowResult<T> = Result<T, WindowError>;

impl Window {
    pub fn new() -> WindowResult<Self> {
        let handler = match driver::WindowHandler::new() {
            Ok(handler) => handler,
            Err(err) => return Err(WindowError::WindowHandlerError(err)),
        };

        Ok(Self { handler })
    }

    pub fn set_title(&self, title: &str) {
        self.handler.set_title(title);
    }

    pub fn get_title(&self) {
        self.handler.get_title()
    }
}
