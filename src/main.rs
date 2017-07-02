extern crate rustfft;
extern crate image;
extern crate imageproc;

use rustfft::num_complex::Complex;
use image::{ImageBuffer, Rgb};

mod generate_points;
mod render;



fn main() {
	let num_iterations = 14;
	let interpolation_amount = 16;
	let pad_size = 2;
	let truncate_size = interpolation_amount * pad_size * 8;

	let initial_points = [
		Complex{ re:  1f64, im: 0f64 },
		Complex{ re:  0f64, im: 0f64 },
		Complex{ re: -1f64, im: 0f64 }
		];

	let fractal_points = generate_points::generate(&initial_points, num_iterations, pad_size);

	println!("Interpolating");
	let fractal_points = generate_points::interpolate(fractal_points, interpolation_amount);

	//there's usually some garbage at the beginning and end that looks way better if we take it off
	let rendered_points = &fractal_points[truncate_size..(fractal_points.len() - truncate_size)];

	println!("Drawing");
	let mut img = ImageBuffer::from_pixel(4096, 4096, Rgb([0,0,0]));
	render::render_fractal(&mut img, rendered_points);

	img.save("test.png").unwrap();
}
