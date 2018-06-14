## ["pitch"](http://plopgrizzly.com/audio#plopgrizzly) by [Plop Grizzly](http://plopgrizzly.com)
Quickly and accurately determine the pitch and volume of a sound sample.

This crate uses a
[Bitstream Autocorrelation Function (BCF)
](http://www.cycfi.com/2018/03/fast-and-efficient-pitch-detection-bitstream-autocorrelation/)
invented by Joel de Guzman to determine the pitch of the sound sample.

## Features
* Determine pitch of an audio wave using Bitstream Autocorrelation (BCF)

## [Contributing](http://plopgrizzly.com/contributing/en#contributing)

## Roadmap to 1.0 (Future Features)
* Try using BACF, the updated version of BCF, which uses peaks rather than
zero-crossings.  Evaluate Speed vs accuracy benifits / downfalls.  Method
differences described
[here](http://www.cycfi.com/2018/04/fast-and-efficient-pitch-detection-bliss/)

## Change Log
### 0.1
* Initial release.
