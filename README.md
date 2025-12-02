# Karaoke AI

Real-time pitch detection karaoke application with ZYN rewards system.

## Setup

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and run
cargo run
```

## Features

- Real-time microphone input processing
- Pitch detection and scoring
- Points system for ZYN product purchases
- JSON-based song format

## How it Works

- `cpal` captures audio samples as `f32` values between -1.0 and 1.0.
- We convert them to `i16` before writing, since WAV commonly stores 16-bit PCM.
- `hound` handles the WAV file format and writes samples sequentially.
- After 5 seconds, the program finalizes the file and closes it.

## Usage

1. Run the application
2. Sing along to the loaded song
3. Receive score and points based on pitch accuracy
4. Use points to purchase ZYN products

## Song Format

Songs are stored as JSON with pitch sequences in Hz:

```json
{
  "name": "Song Name",
  "pitch_sequence": [261.63, 293.66, 329.63],
  "duration_ms": 3000
}
```
