use num::complex::Complex;
use image::{Rgb32FImage, ColorType::{Rgb32F, self}};
pub type Point = Complex<f64>;


pub struct Viewport {
    /// The point in the center of the viewport
    center: Point,
    /// The radius of the largest circle centered at center which is entirely contained in the viewport
    radius: f64
}

pub struct Frame {
    viewport: Viewport,
    buffer: Rgb32FImage
}


pub trait Renderer {
    fn get_frame(&self) -> &Frame;
    fn set_size(&self, width: usize, height: usize);
    fn set_viewport(&self, viewport: Viewport);
}


impl Viewport {
    pub fn default() -> Viewport {
        Viewport { center: (Point {re: 0f64, im: 0f64}), radius: (2.0f64) }
    }

    pub fn new(center: Point, radius: f64) -> Viewport {
        Viewport { center: (center), radius: (radius) }
    }
}


impl Frame {
    pub fn default() -> Frame {
        Frame { viewport: (Viewport::default()), buffer: (Rgb32FImage::new(1600, 900)) }
    }

    pub fn point_from_pixel_index(&self, x: u32, y: u32) -> Point {
        let distance_per_pixel = 
        if self.buffer.height() < self.buffer.width() {
             self.viewport.radius / self.buffer.height() as f64
        }
        else {
            self.viewport.radius / self.buffer.width() as f64
        };
        Point { re: (self.viewport.center.re - (distance_per_pixel * ((self.buffer.width()  / 2) - x) as f64)), 
                im: (self.viewport.center.im + (distance_per_pixel * ((self.buffer.height() / 2) - y) as f64))} 
        // image crate puts 0, 0 at top left, so from the center, that's negative real and positive imaginary, hence the different signs
    }    
}


