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

    #[cfg(target_arch = "wasm32")]
    let log_list = wasm::create_log_list(&window);

    #[cfg(target_arch = "wasm32")]
    wasm::style_canvas();

    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        #[cfg(target_arch = "wasm32")]
        wasm::log_event(&log_list, &event);

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
                    128,//width as u16,
                    128,//height as u16
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

        // Use the width and height specifed in the HTML as the single source of
        // truth.

        let size = winit::dpi::Size::Physical(
            winit::dpi::PhysicalSize::new(
                canvas.width(),
                canvas.height(),
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

    pub fn create_log_list(window: &Window) -> web_sys::Element {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let log_header = document.create_element("h2").unwrap();
        log_header.set_text_content(Some("Event Log"));
        body.append_child(&log_header).unwrap();

        let log_list = document.create_element("ul").unwrap();
        body.append_child(&log_list).unwrap();
        log_list
    }

    pub fn log_event(log_list: &web_sys::Element, event: &Event<()>) {
        //log::debug!("{:?}", event);

        // Getting access to browser logs requires a lot of setup on mobile devices.
        // So we implement this basic logging system into the page to give developers an easy alternative.
        // As a bonus its also kind of handy on desktop.
        if let Event::WindowEvent { event, .. } = &event {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let log = document.create_element("li").unwrap();
            log.set_text_content(Some(&format!("{:?}", event)));
            log_list
                .insert_before(&log, log_list.first_child().as_ref())
                .unwrap();
        }
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