// "pitch" - Licensed under the MIT LICENSE (see /LICENSE)

extern crate pitch;
extern crate byteorder;

use std::io::Cursor;
use self::byteorder::{LittleEndian, ReadBytesExt};

const RESOLUTION: usize = 2048; // size of buffer

const A1: &[u8] = include_bytes!("a1.raw");
const A2: &[u8] = include_bytes!("a2.raw");
const A3: &[u8] = include_bytes!("a3.raw");
const A4: &[u8] = include_bytes!("a4.raw");
const SINE_A4: &[u8] = include_bytes!("sine.raw");
const SAW_A4: &[u8] = include_bytes!("sawtooth.raw");
const SQUARE_A4: &[u8] = include_bytes!("square.raw");
const BAD: &[u8] = include_bytes!("bad.raw");

fn hz_of_raw(data: &[u8]) -> (f32, f32) {
	// Read Sound Wave Data Into `samples`
	let mut reader = Cursor::new(data);
	let mut samples = [0.0f32; RESOLUTION];

	for i in 0..RESOLUTION {
		samples[i] = (reader.read_i16::<LittleEndian>().unwrap()
			as f32) / (::std::i16::MAX as f32);
	}

	pitch::detect(&samples)
}

fn main() {
	let a1 = hz_of_raw(A1);
	let a2 = hz_of_raw(A2);
	let a3 = hz_of_raw(A3);
	let a4 = hz_of_raw(A4);
	let sine_a4 = hz_of_raw(SINE_A4);
	let saw_a4 = hz_of_raw(SAW_A4);
	let square_a4 = hz_of_raw(SQUARE_A4);
	let bad = hz_of_raw(BAD);

	// Print out the pitch of different audio waveforms.
	println!("A1: {} Hz, {} Vl", a1.0, a1.1);
	println!("A2: {} Hz, {} Vl", a2.0, a2.1);
	println!("A3: {} Hz, {} Vl", a3.0, a3.1);
	println!("A4: {} Hz, {} Vl", a4.0, a4.1);
	println!("SINE_A4: {} Hz, {} Vl", sine_a4.0, sine_a4.1);
	println!("SAW_A4: {} Hz, {} Vl", saw_a4.0, saw_a4.1);
	println!("SQUARE_A4: {} Hz, {} Vl", square_a4.0, square_a4.1);
	println!("BAD: {} Hz {}, Vl", bad.0, bad.1);
}
