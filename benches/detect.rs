#[macro_use]
extern crate criterion;
extern crate byteorder;
extern crate pitch;

use std::io::Cursor;

use criterion::Criterion;
use pitch::detect;

use self::byteorder::{LittleEndian, ReadBytesExt};

const RESOLUTION: usize = 2048; // size of buffer

/*
const A1: &[u8] = include_bytes!("../examples/a1.raw");
const A2: &[u8] = include_bytes!("../examples/a2.raw");
const A3: &[u8] = include_bytes!("../examples/a3.raw");
const A4: &[u8] = include_bytes!("../examples/a4.raw");
const SINE_A4: &[u8] = include_bytes!("../examples/sine.raw");
const SAW_A4: &[u8] = include_bytes!("../examples/sawtooth.raw");
const SQUARE_A4: &[u8] = include_bytes!("../examples/square.raw");
const BAD4: &[u8] = include_bytes!("../examples/bad4.raw");
*/

/*
fn hz_of_raw(data: &[u8]) -> (f32, f32) {
    // Read Sound Wave Data Into `samples`
    let mut reader = Cursor::new(data);
    let mut samples = [0.0f32; RESOLUTION];

    for i in 0..RESOLUTION {
        samples[i] = (reader.read_i16::<LittleEndian>().unwrap() as f32)
            / (::std::i16::MAX as f32);
    }

    pitch::detect(&samples)
}*/

fn detection() {
    /*
    let _ = hz_of_raw(A1);
    let _ = hz_of_raw(A2);
    let _ = hz_of_raw(A3);
    let _ = hz_of_raw(A4);
    let _ = hz_of_raw(SINE_A4);
    let _ = hz_of_raw(SAW_A4);
    let _ = hz_of_raw(SQUARE_A4);
    let _ = hz_of_raw(BAD4);
    */
}

fn detect_benchmark(c: &mut Criterion) {
    c.bench_function("detect 8 samples", |b| b.iter(|| detection()));
}

criterion_group!(benches, detect_benchmark);
criterion_main!(benches);
