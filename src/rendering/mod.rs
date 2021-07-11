pub use rgb::Rgb as Rgb;

pub use crate::scene::camera::Camera as Camera;

pub mod backends;
pub mod rgb;
pub mod framebuffer;

pub use framebuffer::{FrameBuffer, RenderTarget};
