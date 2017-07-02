
use rustfft::num_complex::Complex;

use cgmath::{Vector2, BaseNum};
use cgmath::num_traits::Float;
use image::{GenericImage, ImageBuffer, Rgb};

use super::draw_shape;

pub fn render_fractal<ImageType, T>(target: &mut ImageType, fractal_points: &[Complex<T>])
	where
		ImageType: GenericImage<Pixel=Rgb<u8>>,
		T: Float + BaseNum
{
	let half = T::from(0.5f64).unwrap();

	//find the bounds of the fractal
	let mut minx = fractal_points[0].re;
	let mut miny = fractal_points[0].im;
	let mut maxx = minx;
	let mut maxy = miny;

	for element in fractal_points {
		minx = T::min(minx, element.re);
		miny = T::min(miny, element.im);
		maxx = T::max(maxx, element.re);
		maxy = T::max(maxy, element.im);
	}

	//use the bounds of the fractal and the bounds of the image to compute a scale factor
	//our goal is to to make the fractal fill up ~90% of the width or ~90% of the height, whichever is a better fit
	let fractal_size_x = maxx - minx;
	let fractal_size_y = maxy - miny;

	let fractal_center_x = (minx + maxx) * half;
	let fractal_center_y = (miny + maxy) * half;

	let image_size_x = T::from(target.width()).unwrap();
	let image_size_y = T::from(target.height()).unwrap();

	let scale_x = image_size_x / fractal_size_x;
	let scale_y = image_size_y / fractal_size_y;

	//compute the transformation we'll apply to every point
	let scale = T::min(scale_x, scale_y) * T::from(0.9f64).unwrap();

	let translate = Vector2::new(
		image_size_x * half - scale * fractal_center_x,
		image_size_y * half + scale * fractal_center_y,
		);

	println!("{:?}", Vector2::new(fractal_center_x, -fractal_center_y) + translate);

	for (i, window) in fractal_points.windows(2).enumerate() {
		if i % 1000000 == 0 {
			println!("Drawing line {} of {}", i, fractal_points.len() - 1);
		}

		let line_start = Vector2::new(window[0].re, -window[0].im) * scale + translate;
		let line_end =   Vector2::new(window[1].re, -window[1].im) * scale + translate;

		draw_shape::draw_line(target, line_start, line_end);
	}
}