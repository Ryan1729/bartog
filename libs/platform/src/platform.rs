use platform_types::{
    State,
    StateParams,
};

use softbuffer::GraphicsContext;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

use render::{clip, FrameBuffer, NeedsRedraw};

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

    let mut output_frame_buffer = {
        let size = window.inner_size();

        FrameBuffer::from_size((size.width as clip::W, size.height as clip::H))
    };

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

    let mut just_gained_focus = true;

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
            Event::WindowEvent {
                event: WindowEvent::Focused(true),
                window_id,
            } if window_id == window.id() => {
                just_gained_focus = true;
            }
            Event::MainEventsCleared => {
                let (commands, sounds) = state.frame();

                handle_sounds(&mut sound_handler, sounds);

                {
                    let size = window.inner_size();
                    output_frame_buffer.width = size.width as u16;
                    output_frame_buffer.height = size.height as u16;
                }

                let needs_redraw = render::render(
                    &mut output_frame_buffer,
                    &commands,
                );

                if NeedsRedraw::Yes == needs_redraw
                || just_gained_focus {
                    graphics_context.set_buffer(
                        &output_frame_buffer.buffer,
                        output_frame_buffer.width,
                        output_frame_buffer.height,
                    );
                }

                just_gained_focus = false;

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

        // Use the size of the screen the commands pretends there is, since the
        // browser will stretch it for us.
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

    pub(super) fn handle_sounds(_: &mut SoundHandler, _: &[SFX]) {
        // Sound is disabled
    }
}
