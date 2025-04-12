# Kite
A server reimplementation of a particular parental control service.
Works on all major operating systems.

## Roadmap
- URL Classification (Done) (mostly)
- Screen time limits
- Frontend
- Account System

## Running
1. Install [protoc](https://github.com/protocolbuffers/protobuf/releases) and add it to your PATH.
2. Clone Kite `git clone https://github.com/rifting/Kite` `cd Kite`
3. Build with `cargo build --release`. Find your binary in `target/release/kite`.
4. Edit `config.toml` to your desired configuration.
5. Run `kite --config config.toml`
