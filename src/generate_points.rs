
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::{FFTplanner, FFT, FFTnum};

//compute the inverse of the FFT we've been provided
//if you pass in an inverse FFT, this will compute a forward FFT, and vice versa
fn ifft<T: FFTnum>(fft: &FFT<T>, input: &mut [Complex<T>], output: &mut [Complex<T>]) {
	//conjugate inputs
	for element in input.iter_mut() {
		*element = element.conj();
	}

	//compute fft
	fft.process(input, output);

	//conjugate outputs
	for element in output {
		*element = element.conj();
	}
}

fn scale<T: FFTnum>(buffer: &mut [Complex<T>], scale: T) {
	for element in buffer.iter_mut() {
		*element = *element * scale;
	}
}

/// Uses the initial points and the number of generations to compute the points for our fractal. 
///
/// Returns a Vec containing the data points
pub fn generate<T: FFTnum>(inital_points: &[Complex<T>], num_iterations: usize, pad_size: usize) -> Vec<Complex<T>> {
    let mut points = inital_points.to_vec();

    let mut planner = FFTplanner::new(false);

    for i in 0..num_iterations {
    	let len = points.len();

    	//allocate a destination buffer for the FFT. we're going to concatenate a bunch of zeroes afterwards
    	//so skip a step and just create the zeroes here
    	let mut fft_output = vec![Complex::zero(); len * pad_size];
    	let short_fft = planner.plan_fft(len);

    	//FFT the data points
    	short_fft.process(&mut points, &mut fft_output[0..len]);

    	//inverse FFT the data points including the zeroes
    	points = vec![Complex::zero(); len * pad_size];

    	let long_fft = planner.plan_fft(len * pad_size);
    	ifft(&(*long_fft), &mut fft_output, &mut points);

    	//drop the final point and scale the remaining points by 1/n to keep the data points within a reasonable range
        let final_len = points.len() - (pad_size - 1);
    	points.truncate(final_len);
    	scale(&mut points, T::one() / T::from_usize(len).unwrap());

    	println!("Generating points: iteration={}, len={}", i + 1, points.len());
    }

    points
}


pub fn interpolate<T: FFTnum>(mut fractal_points: Vec<Complex<T>>, amount: usize) -> Vec<Complex<T>> {
    let mut planner = FFTplanner::new(false);

    let len = fractal_points.len();

    //allocate a destination buffer for the FFT. we're going to concatenate a bunch of zeroes afterwards
    //so skip a step and just create the zeroes here
    let mut fft_output = vec![Complex::zero(); len * amount];
    let short_fft = planner.plan_fft(len);

    //FFT the data points
    short_fft.process(&mut fractal_points, &mut fft_output[0..len]);

    //inverse FFT the data points including the zeroes
    fractal_points = vec![Complex::zero(); len * amount];

    let long_fft = planner.plan_fft(len * amount);
    ifft(&(*long_fft), &mut fft_output, &mut fractal_points);

    //drop the final point and scale the remaining points by 1/n to keep the data points within a reasonable range
    let final_len = fractal_points.len() - (amount - 1);
    fractal_points.truncate(final_len);
    scale(&mut fractal_points, T::one() / T::from_usize(len).unwrap());

    fractal_points
}