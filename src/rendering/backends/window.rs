use minifb::{Key, Window, WindowOptions};

use crate::rendering::backends::Backend;
use crate::rendering::RenderTarget;
use std::fmt::{Display, Formatter};

/// A [Backend] that renders into a window.
pub struct WindowBackend {}

impl Backend for WindowBackend {
    fn present(&self, buf: &dyn RenderTarget) {
        let pixel_count = (buf.width() * buf.height()) as usize;

        let mut window = Window::new(
            "raytracer - Press 'Esc' to exit",
            buf.width() as usize,
            buf.height() as usize,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let mut output_buf = vec![0; pixel_count * 4];

        const R_OFFSET : usize = 0;
        const G_OFFSET : usize = 1;
        const B_OFFSET : usize = 2;

        let input = buf.as_bytes();

        for i in 0..pixel_count {
            let offset = (i * 3) as usize;
            let r = input[offset + R_OFFSET] as u32;
            let g = input[offset + G_OFFSET] as u32;
            let b = input[offset + B_OFFSET] as u32;

            // weird pattern 0RGB
            output_buf[i as usize] = r << 16 | g << 8 | b << 0;
        }

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        window
            .update_with_buffer(
                &output_buf,
                buf.width() as usize,
                buf.height() as usize,
            )
            .unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update();
        }
    }
}

impl WindowBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for WindowBackend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowBackend")
    }
}
