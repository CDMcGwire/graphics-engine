use std::error::Error;

use gilrs::{ConnectedGamepadsIterator, Gilrs};

#[derive(Debug)]
pub(crate) enum InputInitError {
	GamepadLibraryNotImpl(gilrs::Gilrs),
	GamepadLibraryInit(Box<dyn Error + Send + Sync + 'static>),
	GamepadLibraryOther(gilrs::Error),
}

impl From<gilrs::Error> for InputInitError {
	fn from(e: gilrs::Error) -> Self {
		match e {
			gilrs::Error::NotImplemented(dummy) => InputInitError::GamepadLibraryNotImpl(dummy),
			gilrs::Error::InvalidAxisToBtn => InputInitError::GamepadLibraryOther(e),
			gilrs::Error::Other(inner) => Self::GamepadLibraryInit(inner),
		}
	}
}

#[derive(Debug)]
pub(crate) struct Input {
	gamepad_system: Gilrs,
}

impl Input {
	pub(crate) fn new() -> Result<Input, InputInitError> {
		let gilrs = Gilrs::new()?;
		Ok(Input {
			gamepad_system: gilrs,
		})
	}

	pub(crate) fn gamepads<'a>(&'a self) -> ConnectedGamepadsIterator<'a> {
		self.gamepad_system.gamepads()
	}

	pub(crate) fn next_gp_event(&mut self) -> Option<gilrs::Event> {
		self.gamepad_system.next_event()
	}
}
