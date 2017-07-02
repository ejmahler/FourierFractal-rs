
use rustfft::num_complex::Complex;
use rustfft::num_traits::Float;

use image::{GenericImage, Rgb};
use imageproc;

pub fn render_fractal<ImageType, T>(target: &mut ImageType, fractal_points: &[Complex<T>])
	where
		ImageType: GenericImage<Pixel=Rgb<u8>>,
		T: Float
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

	let translate = Complex{
		re: image_size_x * half - scale * fractal_center_x,
		im: image_size_y * half + scale * fractal_center_y,
		};

	for window in fractal_points.windows(2) {
		let line_start = window[0].conj() * scale + translate;
		let line_end =   window[1].conj() * scale + translate;

		imageproc::drawing::draw_line_segment_mut(target, 
			(line_start.re.to_f32().unwrap(), line_start.im.to_f32().unwrap()),
			(line_end.re.to_f32().unwrap(), line_end.im.to_f32().unwrap()),
			Rgb([255,255,255]),
		);
	}
}