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

                let frame_buffer: &[u32] = state.get_frame_buffer();

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width as u16, size.height as u16)
                };

                let frame_cow = add_bars_if_needed(
                    frame_buffer,
                    (screen::WIDTH.into(), screen::HEIGHT.into()),
                    (width, height),
                );

                graphics_context.set_buffer(
                    &frame_cow,
                    width,
                    height
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

use std::borrow::Cow;
fn add_bars_if_needed<'buffer>(
    frame_buffer: &'buffer [u32],
    (src_w, src_h): (u16, u16),
    (dst_w, dst_h): (u16, u16),
) -> Cow<'buffer, [u32]> {
    let src_w = src_w as usize;
    let src_h = src_h as usize;
    let dst_w = dst_w as usize;
    let dst_h = dst_h as usize;
    let expected_length = dst_w * dst_h;
    if frame_buffer.len() < expected_length {
        let mut frame_vec = Vec::with_capacity(expected_length);

        let width_multiple = dst_w / src_w;
        let height_multiple = dst_h / src_h;
        let multiple = core::cmp::min(width_multiple, height_multiple);

        let vertical_bar_width = (
            (dst_w - (multiple * src_w)) / 2
        ) as usize;

        let horizontal_bar_height = (
            (dst_h - (multiple * src_h)) / 2
        ) as usize;

        dbg!(multiple, vertical_bar_width, horizontal_bar_height);

        // Hopefully this compiles to something not inefficent
        for i in 0..expected_length {
            frame_vec.push(0);
        }

        for y in horizontal_bar_height..(dst_h - horizontal_bar_height) {
            for x in vertical_bar_width..(dst_w - vertical_bar_width) {
                let dst_i = y * dst_w + x;
                frame_vec[dst_i as usize] = 0xFFFFFFFF;
            }
        }

        Cow::Owned(frame_vec)
    } else {
        Cow::Borrowed(frame_buffer)
    }
}

#[cfg(test)]
mod add_bars_if_needed_returns_then_expected_result {
    use super::add_bars_if_needed;

    const R: u32 = 0xFFFF0000;
    const G: u32 = 0xFF00FF00;
    const B: u32 = 0xFF0000FF;
    const C: u32 = 0xFF00FFFF;

    macro_rules! a {
        ($actual: expr, $expected: expr) => {
            assert_eq!(<&[u32] as From<_>>::from(&$actual), &$expected)
        }
    }

    #[test]
    fn on_this_trival_example() {
        let actual = add_bars_if_needed(
            &[
                R, G,
                B, C
            ],
            (2, 2),
            (2, 2),
        );

        a!(
            actual, 
            [
                R, G,
                B, C
            ]
        )
    }

    #[test]
    fn on_this_small_non_trival_example() {
        let actual = add_bars_if_needed(
            &[R, G, B, C],
            (2, 2),
            (4, 2),
        );

        a!(
            actual, 
            [
                0, R, G, 0, 
                0, B, C, 0,
            ]
        )
    }
}
