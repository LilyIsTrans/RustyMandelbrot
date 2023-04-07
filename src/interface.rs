use num::complex::Complex;
use nalgebra::{Vector2, Similarity2};
use image::{Rgb32FImage, ColorType::Rgb32F};
type Point = Complex<f64>;


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



impl Viewport {
    pub fn default() -> Viewport {
        Viewport { center: (Point {re: 0f64, im: 0f64}), radius: (2.0f64) }
    }

    pub fn new(center: Point, radius: f64) -> Viewport {
        Viewport { center: (center), radius: (radius) }
    }
}