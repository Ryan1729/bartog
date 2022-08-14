use platform_types::{State, StateParams};

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

    // This must happen after the `build` call, or the style gets overridden.
    #[cfg(target_arch = "wasm32")]
    wasm::style_canvas();

    let mut graphics_context = unsafe { GraphicsContext::new(window) }.unwrap();

    let mut sound_handler = init_sound_handler();

    let mut loop_helper;
    
    #[cfg(target_arch = "wasm32")] 
    {
        loop_helper = ();
    }
    #[cfg(not(target_arch = "wasm32"))] 
    {
        loop_helper = spin_sleep::LoopHelper::builder()
            .build_with_target_rate(60.0)
    }

    event_loop.run(move |event, _, control_flow| {
        let window = graphics_context.window();

        match event {
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
                let (frame_buffer, sounds) = state.frame();

                handle_sounds(&mut sound_handler, sounds);

                let (mut width, mut height) = {
                    let size = window.inner_size();
                    (size.width as u16, size.height as u16)
                };

                let screen_width = screen::WIDTH.into();
                let screen_height = screen::HEIGHT.into();

                let frame_cow =
                    if width < screen::WIDTH.into()
                    || height < screen::HEIGHT.into() {
                        width = screen_width;
                        height = screen_height;
                        Cow::Borrowed(frame_buffer)
                    } else {
                        add_bars_if_needed(
                            frame_buffer,
                            (screen_width, screen_height),
                            (width, height),
                        )
                    };

                graphics_context.set_buffer(
                    &frame_cow,
                    width,
                    height,
                );

                #[cfg(not(target_arch = "wasm32"))] 
                {
                    loop_helper.loop_sleep();
                    loop_helper.loop_start();
                }
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
    use platform_types::SFX;

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

    pub type SoundHandler = ();

    pub fn init_sound_handler() -> SoundHandler {
        ()
    }

    pub(super) fn handle_sounds(_: &mut SoundHandler, requests: &[SFX]) {
        fn inner(request: SFX) -> Option<()> {
            use js_sys::{Function, Reflect};
            use wasm_bindgen::JsValue;
    
            let window = web_sys::window()?;
    
            let handler = Reflect::get(
                &window,
                &JsValue::from_str("soundHandler")
            ).ok()?.dyn_into::<Function>().ok()?;
    
            let request_string = request.to_sound_key();
    
            handler.call1(&JsValue::undefined(), &request_string.into()).ok()?;
    
            Some(())
        }
    
        for &request in requests {
            // Sound is inessential, so ignore errors.
            let _ = inner(request);
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

    (
        new_seed(),
        Some(logger),
        Some(error_logger),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn new_seed() -> xs::Seed {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let time = time.as_secs_f64();

    unsafe {
        core::mem::transmute::<[f64; 2], [u8; 16]>([time, 1.0 / time])
    }
}

#[cfg(target_arch = "wasm32")]
use wasm::{init_sound_handler, handle_sounds};

#[cfg(not(target_arch = "wasm32"))]
use not_wasm::{init_sound_handler, handle_sounds};

#[cfg(all(
    not(target_arch = "wasm32"),
    feature = "non-web-sound"
))]
mod not_wasm {
    use platform_types::SFX;

    use rodio::{
        decoder::Decoder,
        OutputStream,
        Source,
    };
    use std::sync::mpsc::{channel, Sender};

    pub struct SoundHandler {
        sender: Sender<SFX>
    }

    pub fn init_sound_handler() -> SoundHandler {
        let (sender, receiver) = channel();

        std::thread::spawn(move || {
            let mut rng = xs::from_seed(super::new_seed());

            let output = match OutputStream::try_default() {
                Ok(output) => output,
                // No point in leaving this thread running if we can't play sounds.
                Err(_) => return,
            };

            while let Ok(request) = receiver.recv() {
                macro_rules! i_b {
                    ($name: literal) => {
                        include_bytes!(concat!(
                            "../../../static/sounds/",
                            $name,
                            ".ogg"
                        ))
                    }
                }
                let sounds: &[&[u8]] = match request {
                    SFX::CardPlace => &[
                        i_b!("cardPlace1"),
                        i_b!("cardPlace2"),
                        i_b!("cardPlace3"),
                    ],
                    SFX::CardSlide => &[
                        i_b!("cardSlide1"),
                        i_b!("cardSlide2"),
                        i_b!("cardSlide3"),
                    ],
                    SFX::ButtonPress => &[
                        i_b!("buttonPress1"),
                        i_b!("buttonPress2"),
                        i_b!("buttonPress3"),
                    ],
                };

                let data: &[u8] = sounds[
                    xs::range(&mut rng, 0..sounds.len() as u32) as usize
                ];

                // If one sound file is messed up, don't break all the sounds.    
                if let Ok(decoder) = Decoder::new_vorbis(
                    std::io::Cursor::new(data)
                ) {
                    let _ = output.1.play_raw(
                        decoder.convert_samples()
                    );
                }
            }
        });

        SoundHandler {
            sender,
        }
    }

    pub(super) fn handle_sounds(handler: &mut SoundHandler, requests: &[SFX]) {
        for &request in requests {
            // Sound is inessential, so ignore errors.
            let _ = handler.sender.send(request);
        }
    }
}

#[cfg(all(
    not(target_arch = "wasm32"),
    not(feature = "non-web-sound")
))]
mod not_wasm {
    use platform_types::SFX;

    pub struct SoundHandler;

    pub fn init_sound_handler() -> SoundHandler {
        SoundHandler
    }

    pub(super) fn handle_sounds(handler: &mut SoundHandler, requests: &[SFX]) {
        // Sound is disabled
    }
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
        let width_multiple = dst_w / src_w;
        let height_multiple = dst_h / src_h;
        let multiple = core::cmp::min(width_multiple, height_multiple);
        if multiple == 0 {
            return Cow::Borrowed(frame_buffer);
        }

        let vertical_bars_width = dst_w - (multiple * src_w);

        let left_bar_width = (
            (vertical_bars_width + 1) / 2
        ) as usize;

        let right_bar_width = (
            vertical_bars_width / 2
        ) as usize;

        let horizontal_bars_height = dst_h - (multiple * src_h);

        let top_bar_height = (
            (horizontal_bars_height + 1) / 2
        ) as usize;

        let bottom_bar_height = (
            horizontal_bars_height / 2
        ) as usize;

        let mut frame_vec = Vec::with_capacity(expected_length);

        // Hopefully this compiles to something not inefficent
        for i in 0..expected_length {
            frame_vec.push(0);
        }

        let mut src_i = 0;
        let mut y_remaining = multiple;
        for y in top_bar_height..(dst_h - bottom_bar_height) {
            let mut x_remaining = multiple;
            for x in left_bar_width..(dst_w - right_bar_width) {
                let dst_i = y * dst_w + x;
                frame_vec[dst_i as usize] = frame_buffer[src_i];

                x_remaining -= 1;
                if x_remaining == 0 {
                    src_i += 1;
                    x_remaining = multiple;
                }
            }

            y_remaining -= 1;
            if y_remaining == 0 {
                y_remaining = multiple;
            } else {
                // Go back to the beginning of the row.
                src_i -= src_w;
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

    #[test]
    fn on_this_small_size_doubling_example() {
        let actual = add_bars_if_needed(
            &[R, G, B, C],
            (2, 2),
            (6, 4),
        );

        a!(
            actual,
            [
                0, R, R, G, G, 0,
                0, R, R, G, G, 0,
                0, B, B, C, C, 0,
                0, B, B, C, C, 0,
            ]
        )
    }

    #[test]
    fn on_this_small_odd_height_example() {
        let actual = add_bars_if_needed(
            &[R, G, B, C],
            (2, 2),
            (6, 3),
        );

        a!(
            actual,
            [
                0, 0, 0, 0, 0, 0,
                0, 0, R, G, 0, 0,
                0, 0, B, C, 0, 0,
            ]
        )
    }
}
