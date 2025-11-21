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
