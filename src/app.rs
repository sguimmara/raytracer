use crate::rendering::backends::{Backend, FileBackend, WindowBackend};
use crate::rendering::FrameBuffer;
use crate::scene::Scene;
use log::*;

#[derive(Debug)]
pub struct RunOpts {
    verbose: bool,
    output_file: String,
}

impl RunOpts {
    pub fn new(output_file: &str, verbose: bool) -> Self {
        RunOpts {
            output_file: String::from(output_file),
            verbose,
        }
    }
}

/// runs the raytracer
pub fn run(opts: RunOpts) {
    info!("running raytracer");
    info!("output file is {}", opts.output_file);

    info!("start rendering...");

    let scene = Scene::new();

    let ratio = scene.camera().aspect();
    let height = 512u32;
    let width = (ratio * height as f32) as u32;
    let mut fb = FrameBuffer::new(width, height);
    info!("created {}", fb);

    scene.render(&mut fb);

    // let backend = FileBackend::new(opts.output_file.as_str());
    let backend = WindowBackend::new();

    info!("saving render to {}", backend);
    backend.present(&fb);

    info!("finished.");
}
