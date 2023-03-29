use std::path::Path;

use winit::{
	event::{Event as WinitEvent, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Icon, Window, WindowBuilder},
};

use gilrs::{Button, Event as GilEvent, Gilrs};

struct EngineContext {
	window: Window,
	window_size: winit::dpi::LogicalSize<f32>,
	input: Gilrs,
}

fn main() {
	let path = &Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/data/img/icon.png"));
	let event_loop = EventLoop::new();

	let window_size = winit::dpi::LogicalSize::new(1280.0, 720.0);

	let mut engine_context = EngineContext {
		window: WindowBuilder::new()
			.with_title("Engine")
			.with_inner_size(window_size)
			.with_min_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
			//.with_decorations(false)
			.with_window_icon(get_icon(path))
			.build(&event_loop)
			.expect("Engine needs a system Window to render to"),
		window_size,
		input: Gilrs::new().expect("Evironment needs to be supported by Game Input Library"),
	};

	for (_id, gamepad) in engine_context.input.gamepads() {
		println!("{} is {:?}", gamepad.name(), gamepad.power_info());
	}

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;

		match event {
			WinitEvent::WindowEvent {
				event: WindowEvent::CloseRequested,
				window_id,
			} if window_id == engine_context.window.id() => *control_flow = ControlFlow::Exit,
			_ => update(&mut engine_context),
		}
	});
}

fn update(context: &mut EngineContext) {
	while let Some(GilEvent { id, event, time }) = context.input.next_event() {
		println!("{:?} New event from {}: {:?}", time, id, event);
		if let gilrs::EventType::ButtonPressed(_button @ Button::South, _) = event {
			println!("South Button Hit!!!")
		};
	}

	for (_id, gamepad) in context.input.gamepads() {
		context.window_size.width += match gamepad.axis_data(gilrs::Axis::RightStickX) {
			Some(axis) => axis.value(),
			None => 0.0,
		};
		context.window_size.height += match gamepad.axis_data(gilrs::Axis::RightStickY) {
			Some(axis) => -axis.value(),
			None => 0.0,
		};
	}

	context.window.set_inner_size(context.window_size);
}

fn get_icon(path: &Path) -> Option<Icon> {
	match image::open(path) {
		Ok(img) => {
			let rgba = img.into_rgba8();
			let (width, height) = rgba.dimensions();

			winit::window::Icon::from_rgba(rgba.into_raw(), width, height).ok()
		}
		Err(_) => None,
	}
}
