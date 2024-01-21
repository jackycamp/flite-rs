<div align="center">
    <h1>fliters</h1>
    <p style="font-size: 18px;">rust bindings for the flite c library</p>
</div>

## Usage

```rust
use fliters;

fn main() {
  fliters::tts("hello, world!").play_without_control();
}
```
> Only macos is supported at this time xD

## Roadmap
- [x] basic tts api
- [ ] control output stream with `play`
- [ ] customize voice
- [ ] linux support (rpi) (mess with build script)

## Installation

todo


## Building

todo
