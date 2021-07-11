pub mod file;
use crate::rendering::framebuffer::RenderTarget;
pub use file::FileBackend as FileBackend;

/// Provides a method to render a [FrameBuffer] into an output
pub trait Backend {
    /// renders the [FrameBuffer] into the backend
    fn write(&self, buf: &dyn RenderTarget);
}
