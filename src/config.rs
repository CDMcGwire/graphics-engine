use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};
use winit::{dpi::LogicalSize, window::Icon};

use crate::constants::ENGINE_CONFIG_PATH;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct EngineConfig {
	pub window: WindowConfig,
}

impl Default for EngineConfig {
	fn default() -> Self {
		EngineConfig {
			window: WindowConfig::default(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PixelSize {
	pub(crate) width: u16,
	pub(crate) height: u16,
}

impl PixelSize {
	pub(crate) fn new(width: u16, height: u16) -> PixelSize {
		Self { width, height }
	}

	pub(crate) fn to_logical_size(&self) -> LogicalSize<u16> {
		LogicalSize::new(self.width, self.height)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct WindowConfig {
	pub(crate) size: PixelSize,
	pub(crate) min_size: PixelSize,
	pub(crate) fullscreen: bool,
	pub(crate) borderless: bool,
	pub(crate) decorations: bool,
	pub(crate) resizable: bool,
	pub(crate) transparent: bool,
	pub(crate) title: String,
	pub(crate) icon_path: String,
}

impl WindowConfig {
	pub(crate) fn load_icon(&self) -> Option<Icon> {
		let path = Path::new(&self.icon_path);

		match image::open(path) {
			Ok(img) => {
				let rgba = img.into_rgba8();
				let (width, height) = rgba.dimensions();

				Icon::from_rgba(rgba.into_raw(), width, height).ok()
			}
			Err(_) => None,
		}
	}
}

impl Default for WindowConfig {
	fn default() -> Self {
		WindowConfig {
			size: PixelSize::new(800, 600),
			min_size: PixelSize::new(800, 600),
			fullscreen: false,
			borderless: false,
			decorations: true,
			resizable: true,
			transparent: false,
			title: "Application".to_owned(),
			icon_path: "".to_owned(),
		}
	}
}

pub(crate) fn load_engine_config() -> EngineConfig {
	let engine_config_file =
		File::open(ENGINE_CONFIG_PATH).expect("There should be an engine config file");

	ron::de::from_reader(BufReader::new(engine_config_file))
		.expect("The EngineConfig file should contain a valid EngineConfig structure")
}
