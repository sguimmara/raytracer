use crate::rendering::backends::{Backend, WindowBackend};
use crate::rendering::{FrameBuffer, RenderOpts, Sampling};
use crate::scene::Scene;
use log::*;

/// The parameters for the run command.
#[derive(Debug)]
pub struct RunOpts {
    /// be verbose
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

#[doc(hidden)]
fn progress_func(progress: f32) {
    let percent = (progress * 100f32) as u32;
    if percent % 10 == 0 {
        info!("progress: {}%", percent);
    }
}

/// Runs the raytracer using the specified [RunOpts]
pub fn run(opts: RunOpts) {
    info!("running raytracer");
    info!("output file is {}", opts.output_file);

    info!("start rendering...");

    let scene = Scene::new();

    let ratio = scene.camera().aspect();
    let height = 512;
    let width = (ratio * height as f32) as u32;
    let mut fb = FrameBuffer::new(width, height);
    info!("created {}", fb);

    let opts = RenderOpts::new().with_samples(Sampling::Samples4);

    scene.render(&mut fb, &opts, &progress_func);

    // let backend = FileBackend::new(opts.output_file.as_str());
    let backend = WindowBackend::new();

    info!("saving render to {}", backend);
    backend.present(&fb);

    info!("finished.");
}
