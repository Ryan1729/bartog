use platform_types::{State, StateParams, SFX};

use softbuffer::GraphicsContext;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

pub fn run<S: State + 'static>(mut state: S) {
    let event_loop = EventLoop::new();

    let builder = WindowBuilder::new()
        .with_title("bartog");

    #[cfg(target_arch = "wasm32")]
    let builder = wasm::set_canvas(builder);

    let window = builder
        .build(&event_loop)
        .unwrap();

    // This must happen after the `build` call, or the stle gets overridden.
    #[cfg(target_arch = "wasm32")]
    wasm::style_canvas();

    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        let window = graphics_context.window();

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.frame(handle_sound);

                let frame_buffer = state.get_frame_buffer();

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                graphics_context.set_buffer(
                    frame_buffer,
                    screen::WIDTH.into(),//width as u16,
                    screen::HEIGHT.into(),//height as u16
                );
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput{
                    input: winit::event::KeyboardInput {
                        state: element_state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                    ..
                },
                window_id,
            } if window_id == window.id() => {
                use winit::event::{ElementState, VirtualKeyCode as VK};
                use platform_types::Button;

                let button = match keycode {
                    VK::Return => Button::START,
                    VK::RShift => Button::SELECT,
                    VK::Up => Button::UP,
                    VK::Left => Button::LEFT,
                    VK::Right => Button::RIGHT,
                    VK::Down => Button::DOWN,

                    VK::Z => Button::A,
                    VK::X => Button::B,

                    // For those using the Dvorak layout.
                    VK::Semicolon => Button::A,
                    VK::Q => Button::B,

                    _ => return,
                };

                match element_state {
                    ElementState::Pressed => state.press(button),
                    ElementState::Released => state.release(button),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use winit::{
        event::Event,
        window::{Window, WindowBuilder},
        platform::web::WindowBuilderExtWebSys,
    };
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    pub fn set_canvas(builder: WindowBuilder) -> WindowBuilder {
        let canvas = get_canvas();

        // Use the size of the frambuffer, since the browser will stretch it for us.

        let size = winit::dpi::Size::Physical(
            winit::dpi::PhysicalSize::new(
                screen::WIDTH.into(),
                screen::HEIGHT.into(),
            ),
        );

        builder
            .with_canvas(Some(canvas))
            .with_inner_size(size)
    }

    pub fn style_canvas() {
        let style = get_canvas().style();

        // Remove the winit default applied CSS properties.
        style.remove_property("width").unwrap();
        style.remove_property("height").unwrap();
    }

    fn get_canvas() -> HtmlCanvasElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        document.get_element_by_id("viewport")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap()
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_state_params() -> StateParams {
    use js_sys::Date;
    use web_sys::console;

    fn logger(s: &str) {
        console::log_1(&s.into());
    }

    fn error_logger(s: &str) {
        console::error_1(&s.into());
    }

    let time = Date::new_0().get_time();

    let seed = unsafe {
        core::mem::transmute::<[f64; 2], [u8; 16]>([time, 1.0 / time])
    };

    (
        seed,
        Some(logger),
        Some(error_logger),
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_state_params() -> StateParams {
    fn logger(s: &str) {
        println!("{}", s);
    }

    fn error_logger(s: &str) {
        eprintln!("{}", s);
    }

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let time = time.as_secs_f64();

    let seed = unsafe {
        core::mem::transmute::<[f64; 2], [u8; 16]>([time, 1.0 / time])
    };

    (
        seed,
        Some(logger),
        Some(error_logger),
    )
}

#[cfg(target_arch = "wasm32")]
fn handle_sound(request: SFX) {
    fn inner(request: SFX) -> Option<()> {
        use js_sys::{Function, Reflect};
        use wasm_bindgen::{JsCast, JsValue};

        let window = web_sys::window()?;

        let handler = Reflect::get(
            &window,
            &JsValue::from_str("soundHandler")
        ).ok()?.dyn_into::<Function>().ok()?;

        let request_string = request.to_sound_key();

        handler.call1(&JsValue::undefined(), &request_string.into()).ok()?;

        Some(())
    }

    let _ignored = inner(request);
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_sound(request: SFX) {
    // TODO actually handle sound
}