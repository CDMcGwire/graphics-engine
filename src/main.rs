use winit::{
	dpi::LogicalSize,
	event::{Event as WinitEvent, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

use gilrs::{Button, Event as GilEvent};

use crate::input::Input;

mod config;
mod constants;
mod input;

struct EngineContext {
	window: Window,
	window_size: LogicalSize<f32>,
	input: Input,
}

fn main() {
	// TODO: Fleet debugger runs the debug build in a weird directory. Need to change that.
	println!("{}", std::env::current_dir().unwrap().display());

	let config = config::load_engine_config();
	let event_loop = EventLoop::new();

	let window = WindowBuilder::new()
		.with_title(&config.window.title)
		.with_inner_size(config.window.size.to_logical_size())
		.with_min_inner_size(config.window.min_size.to_logical_size())
		.with_decorations(config.window.decorations)
		.with_window_icon(config.window.load_icon())
		.build(&event_loop)
		.expect("Engine needs a system Window to render to");

	let input = Input::new().expect("Evironment needs to be supported by Game Input Library");

	let mut engine_context = EngineContext {
		window,
		window_size: LogicalSize {
			width: config.window.size.width as f32,
			height: config.window.size.height as f32,
		},
		input,
	};

	for (_id, gamepad) in engine_context.input.gamepads() {
		println!("{} is {:?}", gamepad.name(), gamepad.power_info());
	}

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;

		// TODO: add event that resets the controller driven window size when window is manually dragged to resize
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
	while let Some(GilEvent { id, event, time }) = context.input.next_gp_event() {
		println!("{:?} New event from {}: {:?}", time, id, event);
		if let gilrs::EventType::ButtonPressed(_button @ Button::South, _) = event {
			println!("South Button Hit!!!")
		};
	}

	for (_id, gamepad) in context.input.gamepads() {
		context.window_size.width += match gamepad.axis_data(gilrs::Axis::RightStickX) {
			Some(axis) => axis.value().round(),
			None => 0.0,
		};
		context.window_size.height += match gamepad.axis_data(gilrs::Axis::RightStickY) {
			Some(axis) => -axis.value().round(),
			None => 0.0,
		};
	}

	context.window.set_inner_size(context.window_size);
}
