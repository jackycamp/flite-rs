<div align="center">
    <h1>fliters</h1>
    <p style="font-size: 18px;">rust bindings for the flite c library</p>
</div>

## Usage
Check out `/examples` directory for more examples.
```rust
// standard example
use fliters;

fn main() {
  fliters::tts("hello, world!").play();
}
```

```rust
// Controlling the output stream. (i.e. playing and pausing)
use fliters;
use rodio::{OutputStream, Sink};

fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let fwav = fliters::tts("play this audio with some stream controls");
    let decoder = fwav.get_decoder();
    sink.append(decoder);
    sink.play();
    std::thread::sleep(std::time::Duration::from_secs(1));
    sink.pause();
    std::thread::sleep(std::time::Duration::from_secs(3));
    sink.play();
    sink.sleep_until_end();
}
```
> Only macos is supported at this time xD

## Roadmap
- [x] basic tts api
- [x] control output stream with `play`
- [ ] optimize build (include only necessary c files)
- [ ] customize voice (defaults to slt right now)
- [ ] linux support (rpi) (mess with build script)

## Installation

Add this to your cargo.toml:
`fliters = "0.1.0"`

## Building
```bash
# clone the repository
git clone https://github.com/jackycamp/flite-rs.git

# do the build
cd flite-rs && cargo build

# run an example
cargo run --example tts
```
