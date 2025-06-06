## Pitch

[![tests](https://github.com/NibbleRealm/pitch/actions/workflows/ci.yml/badge.svg)](https://github.com/AldaronLau/p-chan/actions/workflows/ci.yml)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/y/AldaronLau/p-chan)](https://github.com/NibbleRealm/pitch)
[![GitHub contributors](https://img.shields.io/github/contributors/AldaronLau/p-chan)](https://github.com/NibbleRealm/pitch/graphs/contributors)  
[![Crates.io](https://img.shields.io/crates/v/p-chan)](https://crates.io/crates/pitch)
[![Crates.io](https://img.shields.io/crates/d/p-chan)](https://crates.io/crates/pitch)
[![Crates.io (recent)](https://img.shields.io/crates/dr/p-chan)](https://crates.io/crates/pitch)  
[![Crates.io](https://img.shields.io/crates/l/p-chan)](https://github.com/search?q=repo%3ANibbleRealm%2Fpitch+path%3A**%2FLICENSE*&type=code)
[![Docs.rs](https://docs.rs/pitch/badge.svg)](https://docs.rs/pitch/)

Quickly and accurately determine the pitch and volume of a sound sample.

This crate uses a [Bitstream Autocorrelation Function (BCF)] invented by Joel de
Guzman to determine the pitch of the sound sample.

## Features

 - Determine pitch of an audio wave using Bitstream Autocorrelation (BCF)

## Roadmap to 1.0 (Future Features)

 - Try using BACF, the updated version of BCF, which uses peaks rather than
zero-crossings.  Evaluate Speed vs accuracy benifits / downfalls.  Method
differences described [here].

### Supported Platforms

Pitch targets all platforms that can run Rust, and contains no platform-specific
code.

## MSRV

The current MSRV is Rust 1.85.

Any future MSRV updates will follow the [Ardaku MSRV guidelines].

## License

Copyright © 2018-2025 The Pitch Contributors.

Licensed under any of
 - Apache License, Version 2.0, ([LICENSE\_APACHE] or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 - Boost Software License, Version 1.0, ([LICENSE\_BOOST] or
   <https://www.boost.org/LICENSE_1_0.txt>)
 - MIT License, ([LICENSE\_MIT] or <https://mit-license.org/>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## Help

If you want help using or contributing to this library, feel free to send me an
email at <aldaronlau@gmail.com>.

[LICENSE\_APACHE]: https://github.com/NibbleRealm/pitch/blob/v0/LICENSE_APACHE
[LICENSE\_MIT]: https://github.com/NibbleRealm/pitch/blob/v0/LICENSE_MIT
[LICENSE\_BOOST]: https://github.com/NibbleRealm/pitch/blob/v0/LICENSE_BOOST
[Ardaku MSRV guidelines]: https://github.com/ardaku/.github/blob/v1/profile/MSRV.md
[here]: http://www.cycfi.com/2018/04/fast-and-efficient-pitch-detection-bliss/
[Bitstream Autocorrelation Function (BCF)]: http://www.cycfi.com/2018/03/fast-and-efficient-pitch-detection-bitstream-autocorrelation/
