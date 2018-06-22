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
const BAD2: &[u8] = include_bytes!("bad2.raw");
const BAD3: &[u8] = include_bytes!("bad3.raw");
const BAD4: &[u8] = include_bytes!("bad4.raw");
const BAD5: &[u8] = include_bytes!("bad5.raw");
const BAD6: &[u8] = include_bytes!("bad6.raw");
const BAD7: &[u8] = include_bytes!("bad7.raw");
const BAD8: &[u8] = include_bytes!("bad8.raw");

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
	let bad2 = hz_of_raw(BAD2);
	let bad3 = hz_of_raw(BAD3);
	let bad4 = hz_of_raw(BAD4);
	let bad5 = hz_of_raw(BAD5);
	let bad6 = hz_of_raw(BAD6);
	let bad7 = hz_of_raw(BAD7);
	let bad8 = hz_of_raw(BAD8);

	// Print out the pitch of different audio waveforms.
	println!("A1(55hz): {} Hz, {} Vl", a1.0, a1.1);
	println!("A2(110hz): {} Hz, {} Vl", a2.0, a2.1);
	println!("A3(220hz): {} Hz, {} Vl", a3.0, a3.1);
	println!("A4(440hz): {} Hz, {} Vl", a4.0, a4.1);
	println!("SINE_A4(440hz): {} Hz, {} Vl", sine_a4.0, sine_a4.1);
	println!("SAW_A4(440hz): {} Hz, {} Vl", saw_a4.0, saw_a4.1);
	println!("SQUARE_A4(440hz): {} Hz, {} Vl", square_a4.0, square_a4.1);
	println!("BAD(114.286hz): {} Hz {}, Vl", bad.0, bad.1);
	println!("BAD2(114.286hz): {} Hz {}, Vl", bad2.0, bad2.1);
	println!("BAD3(114.286hz): {} Hz {}, Vl", bad3.0, bad3.1);
	println!("BAD4(114.286hz): {} Hz {}, Vl", bad4.0, bad4.1);
	println!("BAD5(114.286hz): {} Hz {}, Vl", bad5.0, bad5.1);
	println!("BAD6(114.286hz): {} Hz {}, Vl", bad6.0, bad6.1);
	println!("BAD7(114.286hz): {} Hz {}, Vl", bad7.0, bad7.1);
	println!("BAD8(114.286hz): {} Hz {}, Vl", bad8.0, bad8.1);
}
