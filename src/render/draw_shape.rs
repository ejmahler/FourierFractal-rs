
use cgmath::{Vector2, BaseNum};
use cgmath::num_traits::Float;
use image::{GenericImage, Rgb};

fn pixels_in_line<T, F>(start: Vector2<T>, end: Vector2<T>, mut callback: F)
	where
		T: Float + BaseNum,
		F: FnMut(u32, u32)
{
	let dv = end - start;

	//we're going to lerp from start to end using just enough inbetween point to make sure we put a pixel in every row and column
	let num_segments = T::max(dv.x.abs(), dv.y.abs()).ceil();

	for i in 0..(num_segments.to_u32().unwrap() + 1) {
		let percent = T::from(i).unwrap() / num_segments;

		let interpolated = start * (T::one() - percent) + end * percent;

		callback(interpolated.x.to_u32().unwrap(), interpolated.y.to_u32().unwrap());
	}
}

pub fn draw_line<ImageType, T>(target: &mut ImageType, start: Vector2<T>, end: Vector2<T>)
	where
		ImageType: GenericImage<Pixel=Rgb<u8>>,
		T: Float + BaseNum
{
	pixels_in_line(start, end, |x, y| {
		if x < target.width() && y < target.height() {
			target.put_pixel(x, y, Rgb([255,255,255]));
		}
	});
}