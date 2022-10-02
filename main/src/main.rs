use crate::game::Game;

mod game;
mod wgpu;

fn main() {
    println!("[*] Hello World");

    #[cfg(not(target_arch = "wasm32"))]
    native_main().unwrap();

    #[cfg(target_arch = "wasm32")]
    web_main().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn native_main() -> Result<(), Box<dyn std::error::Error>> {
    use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder};

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .filter_module("wgpu", log::LevelFilter::Info)
        .filter_module("winit", log::LevelFilter::Info)
        .init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1280, 720))
        .with_fullscreen(None)
        .with_resizable(true)
        .with_title("Hello :)")
        .build(&event_loop)?;

    let game = pollster::block_on(Game::new(event_loop, window))?;
    game.run();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn web_main() -> Result<(), Box<dyn std::error::Error>> {
    use winit::{
        dpi::PhysicalSize, event_loop::EventLoop, platform::web::WindowExtWebSys,
        window::WindowBuilder,
    };

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(PhysicalSize::new(1280, 720))
        .build(&event_loop)?;

    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            body.append_child(&web_sys::Element::from(window.canvas()))
                .ok()
        })
        .expect("couldn't append canvas to document body");

    wasm_bindgen_futures::spawn_local(async {
        match Game::new(event_loop, window).await {
            Ok(v) => v.run(),
            Err(e) => log::error!("{e}"),
        }
    });

    Ok(())
}
