use num::complex::Complex;
use image::{Rgb32FImage, Rgb};
use ndarray::{Array2, Shape, ShapeBuilder};
pub type Point = Complex<f64>;
pub type HDRColour = Rgb<f32>;
pub type DisplayColour = u32;

use rayon::prelude::*;

pub fn hdr_to_display_colour(colour: &HDRColour) -> DisplayColour
{
    let red = (colour.0[0] * 255.0f32) as u32;
    let green = (colour.0[1] * 255.0f32) as u32;
    let blue = (colour.0[2] * 255.0f32) as u32;

    blue | (green << 8) | (red << 16)
   
}

pub struct Viewport {
    /// The point in the center of the viewport
    center: Point,
    /// The radius of the largest circle centered at center which is entirely contained in the viewport
    radius: f64
}

pub struct Frame {
    pub viewport: Viewport,
    pub buffer: Rgb32FImage,
    iter_buf: Array2<usize>
}

impl Viewport {
    pub fn default() -> Viewport {
        Viewport { center: (Point {re: 0f64, im: 0f64}), radius: (5.0f64) }
    }

    pub fn new(center: Point, radius: f64) -> Viewport {
        Viewport { center: (center), radius: (radius) }
    }

    pub fn zoom(&mut self, toward: Point, factor: f64)
    {
        let new_radius = self.radius * factor;
        let new_center = self.center + ((1f64 - factor) * (toward - self.center));
        self.radius = if new_radius < 6.0f64 {new_radius} else {6.0f64};
        self.center = if new_center.norm_sqr() < 8.0f64 {new_center} else {self.center};
    }
}

enum EscapeTime {
    Iterations(usize),
    Never
}


fn compute_escape_time(point: Point) -> EscapeTime {
    const MAX_ITERATIONS: usize = 200;
    let c = point;
    let mut z = Point::new(0f64, 0f64);
    for i in 0..MAX_ITERATIONS {
        z = z * z + c;
        if z.norm_sqr() > 4.0f64 {
            return EscapeTime::Iterations(i);
        };
    };
    EscapeTime::Never
}



impl Frame {
    pub fn default() -> Frame {
        Frame { viewport: (Viewport::default()), buffer: (Rgb32FImage::new(1600, 900)), iter_buf: (Array2::default([1600, 900]))}
    }

    pub fn new(width: u32, height: u32) -> Frame {
        Frame { viewport: (Viewport::default()), buffer: (Rgb32FImage::new(width, height)), iter_buf: (Array2::) }
    }

    pub fn change_window_size(&mut self, width: u32, height: u32) {
        self.buffer = Rgb32FImage::new(width, height);
    }

    fn point_from_pixel_index(x: f64, y: f64, bufferheight: u32, bufferwidth: u32, viewport: &Viewport) -> Point {
        let distance_per_pixel = 
        if bufferheight < bufferwidth {
             viewport.radius / bufferheight as f64
        }
        else {
            viewport.radius / bufferwidth as f64
        };
        Point { re: (viewport.center.re - (distance_per_pixel * ((bufferwidth as f64  / 2f64) - x))), 
                im: (viewport.center.im + (distance_per_pixel * ((bufferheight as f64 / 2f64) - y)))}
        // image crate puts 0, 0 at top left, so from the center, that's negative real and positive imaginary, hence the different signs
        
        
    }

    pub fn point_from_pixel(&self, x: f64, y: f64) -> Point {
        Frame::point_from_pixel_index(x, y, self.buffer.height(), self.buffer.width(), &self.viewport)
    }

    pub fn render(&mut self)
    {
        const WHITE: Rgb<f32> = Rgb([1.0f32, 1.0f32, 1.0f32]);
        const BLACK: Rgb<f32> = Rgb([0.0f32, 0.0f32, 0.0f32]);
        let bufferheight = self.buffer.height();
        let bufferwidth = self.buffer.width();
        self.buffer.enumerate_pixels_mut().par_bridge().for_each(|(x, y, color)|
        {
            
            *color = match compute_escape_time(Frame::point_from_pixel_index(x as f64, y as f64, bufferheight, bufferwidth, &self.viewport)) {
                EscapeTime::Iterations(_) => WHITE,
                EscapeTime::Never => BLACK
            };
        });
    }

    
}





