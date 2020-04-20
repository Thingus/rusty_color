extern crate hound;
use hound::{WavReader, WavSpec, WavIntoSamples, WavSamples};
use sample::{signal, Signal}
use std::{
    error::Error,
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
    string::String,
    io::Read,
    any::type_name
};

fn main(){
    //Ask for path to audio file
    let audio_path = String::from("/home/john/projects/rusty_color/sample.wav");
    //Ask for path to output
    let out_path = String::from("../out.avi");
    
    //Open audio
    let mut reader = WavReader::open(&audio_path).unwrap();
    let spec = reader.spec();
    
    
    //Read whole audio into memory (for now)
    //This from https://github.com/Michael-F-Bryan/noise-gate/blob/master/examples/wav-splitter.rs
    
    let samples = reader
       .samples::<i32>();
       
    let n_samples = samples.len() as usize;
        
    print!("Sample rate: {}\n", spec.sample_rate);
    print!("Number of samples {}\n", n_samples);
        
    print!("Samples are:\n");
    print_samples(samples)
    
    //Break sample into frames
				

    //Process audio frames to video frames
    //Put video frames back together
    //Save video
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn samples_to_frames<R>(samples: WavSamples<R, i32>) -> FrameArray
where

fn print_samples<R>(samples: WavSamples<R,i32>)
where 
    R: std::io::Read
{
    let mut sample_vaule: i32;
    for this_sample in samples{
        //hound::Sample::read(&mut this_sample.unwrap(), sample_value, 8, 0);
        print!("{}", this_sample.unwrap());
        //print!("{}", type_of(this_sample));
    }
}
