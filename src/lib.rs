// "pitch" - Licensed under the MIT LICENSE (see /LICENSE)

//! Quickly and accurately determine the pitch and volume of a sound sample.
//!
//! This crate uses a
//! [Bitstream Autocorrelation Function (BCF)](http://www.cycfi.com/2018/03/fast-and-efficient-pitch-detection-bitstream-autocorrelation/)
//! to determine the pitch of the sound sample.
//!
//! # Example
//! Example code can be found in `test.rs`.  The audio files I used were me
//! playing my trombone on notes A1, A2, A3, and A4, and generating sine, saw,
//! and square waves with Audacity on the note A4 (making sure to set 48000hz
//! sample rate).  Here is my example output (Note: I didn't tune my trombone
//! beforehand since this program is a tuner, and may give different results
//! than other tuners):
//!
//! ```none
//! A1: 55.422962 Hz, 0.7379681 Vl
//! A2: 107.865166 Hz, 0.43305764 Vl
//! A3: 215.78847 Hz, 0.69536424 Vl
//! A4: 436.30658 Hz, 0.117435224 Vl
//! SINE_A4: 439.99805 Hz, 0.8001343 Vl
//! SAW_A4: 439.99167 Hz, 0.8001343 Vl
//! SQUARE_A4: 434.3891 Hz, 0.80022585 Vl
//! ```

// BCF constants:
const SPS: u32 = 48_000; // Sample Hz
const MAX_FREQ: f32 = 10_000.0; // Stupidly high note
const MIN_PERIOD: f32 = (SPS as f32) / MAX_FREQ; // Minumum Period Samples

struct ZeroCross(bool);

impl ZeroCross {
	fn new() -> Self {
		ZeroCross(false)
	}

	fn get(&mut self, s: f32, t: f32) -> bool {
		if s < -t {
			self.0 = false;
		} else if s > t {
			self.0 = true;
		}

		self.0
	}
}

struct BitStream {
	bits: Vec<usize>,
	len: usize,
}

impl BitStream {
	fn new(size: usize) -> BitStream {
		let nbits = ::std::mem::size_of::<usize>() * 8;

		BitStream {
			bits: vec![0usize; size/nbits],
			len: size,
		}
	}

	fn set(&mut self, i: usize, value: bool) {
		let nbits = ::std::mem::size_of::<usize>() * 8;

		let index = i / nbits;
		let bitofs = i % nbits;

		self.bits[index] ^= (if value { ::std::usize::MAX } else { 0 }
			^ self.bits[index]) & (1usize << bitofs);
	}

	fn autocorrelate(&mut self, start_pos: usize, f: &mut FnMut(usize, u32)) {
		let nbits = ::std::mem::size_of::<usize>() * 8;

		let mid_array = (self.bits.len() / 2) - 1;
		let mid_pos = self.len / 2;
		let mut index = start_pos / nbits;
		let mut shift = start_pos % nbits;

		// get autocorrelation values for the first half of the sample.
		for pos in start_pos..mid_pos {
			let mut p1 = 0;
			let mut p2 = index;
			let mut count = 0;

			if shift == 0 {
				for _i in 0..mid_array {
					count += (self.bits[p1] ^ self.bits[p2])
						.count_ones();
					p1+=1; p2+=1;
				}
			} else {
				let shift2 = nbits - shift;
				for _i in 0..mid_array {
					let mut v = self.bits[p2] >> shift;
					p2+=1;
					v |= self.bits[p2] << shift2;
					count += (self.bits[p1] ^ v).count_ones();
					p1+=1;
				}
			}
			shift += 1;
			if shift == nbits {
				shift = 0;
				index += 1;
			}

			f(pos, count);
		}
	}
}

/// Do the BCF calculation on raw samples.  Returns `(hz, amplitude[0-1])`.
pub fn detect(samples: &[f32]) -> (f32, f32) {
	// Get The Amplitude (Volume).
	let mut volume = 0.0f32;
	for i in samples.iter() {
		volume = volume.max(i.abs());
	}

	// Convert Into a Bitstream of Zero-Crossings
	let mut bin = BitStream::new(samples.len());
	let mut zc = ZeroCross::new();
	let t = volume * 0.00001;

	for i in 0..samples.len() {
		let setv = zc.get(samples[i], t);
		bin.set(i, setv);
	}

	// Binary Autocorrelation
	let mut max_count = 0u32;
	let mut min_count = ::std::u32::MAX;
	let mut est_index = 0usize;
	let mut corr = vec![0u32; samples.len() / 2];

	bin.autocorrelate(MIN_PERIOD as usize, &mut |pos, count| {
		corr[pos] = count;
		max_count = max_count.max(count);
		if count < min_count {
			min_count = count;
			est_index = pos;
		}
	});

	// Estimate the pitch:
	// - Get the start edge
	let mut prev = 0.0f32;
	let mut start_edge = samples.iter().enumerate();
	let start_edge = loop {
		let (i, start_edge2) = start_edge.next().unwrap();
		if *start_edge2 > 0.0 {
			break (i as f32, start_edge2);
		}
		prev = *start_edge2;
	};

	let mut dy = start_edge.1 - prev;
	let dx1 = -prev / dy;

	// - Get the next edge
	let mut next_edge = samples.iter().enumerate().skip(est_index - 1);
	let next_edge = loop {
		let (i, next_edge2) = next_edge.next().unwrap();
		if *next_edge2 > 0.0 {
			break (i as f32, next_edge2);
		}
		prev = *next_edge2;
	};

	dy = next_edge.1 - prev;
	let dx2 = -prev / dy;

	let n_samples: f32 = (next_edge.0 - start_edge.0) + (dx2 - dx1);

	// The frequency
	((SPS as f32) / n_samples, volume)
}
