use std::fs::File;
use std::io::BufReader;
use hound::WavReader;

fn main() {
    // Open the WAV file (you must change that part.)
    let filename = "../personal/mixkit-arcade-retro-game-over-213.wav";
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut wav_reader = WavReader::new(reader).expect("Failed to read WAV file");

    // Read the samples
    let mut samples = Vec::new();
    for sample in wav_reader.samples::<i16>() {
        let sample = match sample {
            Ok(sample) => sample,
            Err(err) => {
                eprintln!("Failed to read sample: {:?}", err);
                continue;
            }
        };
        samples.push(sample);
    }

    // Process the samples (perform necessary operations)

    // Write the samples to a new WAV file (you must change that part.)
    let output_filename = "../personal/new arcade.wav";
    let output_file = File::create(output_filename).expect("Failed to create output file");
    let spec = wav_reader.spec();
    let mut writer = hound::WavWriter::new(output_file, spec).expect("Failed to create WAV writer");
    for sample in samples {
        writer.write_sample(sample).expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV file");
}

/*

Cargo.toml:

[package]
name = "rust-tutorial-with-projects"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rodio = "0.17.1"
hound = "3.5.0"

*/