use crate::rendering::backends::{Backend, FileBackend};
use crate::rendering::{Camera, FrameBuffer, Rgb};

mod math;
mod rendering;
use log;
use simplelog::*;

fn main() {
    configure();

    let mut fb = FrameBuffer::new(108, 25);
    log::info!("created {}", fb);

    let camera = Camera::new(Rgb::red());
    log::info!("created {}", camera);

    log::info!("start rendering...");

    camera.render(&mut fb);

    let backend = FileBackend::new("/home/sguimmara/Downloads/foo.png");

    log::info!("saving render to {}", backend);
    backend.write(&fb);

    log::info!("finished.");
}

fn configure() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
