use log::*;
use crate::rendering::backends::{FileBackend, Backend};
use crate::rendering::{FrameBuffer};
use crate::scene::Scene;

#[derive(Debug)]
pub struct RunOpts {
    verbose: bool,
    output_file: String,
}

impl RunOpts {
    pub fn new(output_file: &str, verbose: bool) -> Self {
        RunOpts { output_file: String::from(output_file), verbose }
    }
}

/// runs the raytracer
pub fn run(opts: RunOpts) {
    info!("running raytracer");
    info!("output file is {}", opts.output_file);

    let mut fb = FrameBuffer::new(256, 256);
    info!("created {}", fb);

    info!("start rendering...");

    let scene = Scene::new();

    scene.render(&mut fb);

    let backend = FileBackend::new(opts.output_file.as_str());

    info!("saving render to {}", backend);
    backend.write(&fb);

    info!("finished.");
}
