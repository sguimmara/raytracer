pub mod file;
pub mod window;
use crate::rendering::framebuffer::RenderTarget;
pub use file::FileBackend;
pub use window::WindowBackend;

/// Provides a method to render a [FrameBuffer] into an output
pub trait Backend {
    /// renders the [FrameBuffer] into the backend
    fn present(&self, buf: &dyn RenderTarget);
}
