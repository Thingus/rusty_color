extern crate hound;
use hound::{WavReader, Sample};
//use sample::{signal, Signal};
use image::{Pixel, ImageBuffer};
use std::{
//    error::Error,
//    io::BufWriter,
		//io::FileWriter,
//    path::PathBuf,
    string::String,
    io::{Read,Seek},
    any::type_name,
		error::Error
};

//GStreamer for streaming from audio to 'camera'

fn main() -> Result<(), Box<dyn Error>>{
    //Ask for path to audio file
    let audio_path = String::from("/home/john/projects/rusty_color/sample.wav");
    //Ask for path to output
    let out_path = String::from("../out.avi");
    
    //Open audio
    let mut reader = WavReader::open(&audio_path).unwrap();
    let spec = reader.spec();
    
    let samples = reader.samples::<i32>();
       
    let n_samples = samples.len() as usize;
        
    print!("Sample rate: {}\n", spec.sample_rate);
    print!("Number of samples {}\n", n_samples);
    
		let samples = reader
		.into_samples::<i16>()
		.map(|result| result.map(|sample| [sample]))
		.collect::<Result<Vec<_>, _>>()?;

		print!("Window 1 is:");
		let sample_1 = get_nth_frame(&samples, 0, 20);
		let frame_1 = audio_to_video_frame(sample_1, 512, 512);
		save_image_as_png(frame_1);
//Break sample into frames
				
    //Process audio frames to video frames
    
		//Put video frames back together
    //Save video
		Ok(())
}


fn get_nth_frame<S>(samples: &Vec<S>, frame_index :usize, frame_length:usize) -> &[S]
{
		let frame_start = frame_index * frame_length;
		let frame_end = frame_start + frame_length;   // Add range check here
		&samples[frame_start..frame_end]
}


fn audio_to_video_frame<S>(audio_frame: &Vec<S>, x_res:u32, y_res:u32) -> ImageBuffer<P,Vec<P::Subpixel>>
where
	S: Sample,
	P: Pixel
{
	let mut out_image = ImageBuffer::from_fn(x_res, y_res, |x, y| {
		image::Luma(audio_frame[x%(x_res*y_res)])
	});
	return out_image
}


fn save_image_as_png(image: ImageBuffer<P, Vec<P::Subpixel>>)
where
  P: Pixel
{
	image.save("test_image.png").unwrap()
}


// fn audio_to_video<S>(

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

//fn samples_to_frames<R>(samples: WavSamples<R, i32>) -> FrameArray
//where
fn print_samples<S>(samples: Vec<S>)
where 
		S: Sample
{
    let mut sample_vaule: i32;
    for this_sample in samples{
        //hound::Sample::read(&mut this_sample.unwrap(), sample_value, 8, 0);
        //print!("{:?}", this_sample);
        //print!("{}", type_of(this_sample));
    }
}
