# vigilo-core

[![Rust](https://img.shields.io/badge/Rust-1.80+-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![ONNX Runtime](https://img.shields.io/badge/ONNX%20Runtime-1.24-005CED?logo=onnx&logoColor=white)](https://onnxruntime.ai/)
[![ort](https://img.shields.io/badge/ort-2.0.0--rc.12-purple)](https://github.com/pykeio/ort)
[![Licence](https://img.shields.io/badge/licence-AGPL--3.0-blue)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20x64-0078D6?logo=windows&logoColor=white)](#requirements)

**Camera frames in, proctoring signals and decisions out.** A high-performance, multimodal stream fusion and proctoring engine written in pure Rust.

Zero UI, zero browser, zero webview, and **zero Tauri dependencies**.

---

## Lineage & Ecosystem

`vigilo-core` (formerly `deepscreen-detect`) represents the extracted, production-proven engine of [Vigilo](https://github.com/Abdullah-Masood-05/Vigilo).

It acts as the core engine powering three downstream targets:

1. **`vigilo-core` (This crate)** — The foundational Rust library and `detect-cli` developer harness.
2. **`rustream` (PyPI)** — Python bindings via PyO3 & Maturin providing zero-copy multimodal streaming pipelines and frame bus access.
3. **`flapguard` (npm)** — Node.js bindings via napi-rs providing the temporal decision engine (`FusionEngine`, hold timers, hysteresis, decaying score).
4. **`Vigilo` (`deepscreen-viewer`)** — The desktop proctoring application.

---

## What It Does

| Pipeline Slot | Model / Technique | Rate | Output |
|---|---|---|---|
| **Face** | YuNet 2023mar | 15 Hz | 2D BBox, 5 facial landmarks |
| **Head Pose** | MobileNetV3-Small | 15 Hz | Yaw, pitch, roll angles (degrees) |
| **Gaze** | MobileGaze (MobileOne-S0) | 15 Hz | Gaze angles & **eye-in-head** vector |
| **Objects** | YOLOX-Nano | 1 Hz | Phone, book, laptop bounding boxes |
| **Identity** | ArcFace `w600k_mbf` | 0.2 Hz | 512-d cosine face embedding verification |
| **Fusion** | Deterministic temporal engine | Every tick | Debounced proctoring events & violations |

### Key Capabilities

- **Eye-In-Head Gaze Tracking**: Isolates `gaze − head_pose`. A candidate looking down at a phone on their desk without turning their head is detected instantly.
- **Pure Deterministic Fusion**: `FusionEngine::step` is a pure function of inputs and discrete time (`t_ms`). Replaying the same recording yields bit-identical violations every time.
- **Lock-Free Triple-Buffered Frame Bus**: Uses `ArcSwap` and `crossbeam-channel` so slow inference workers drop stale frames rather than backing up the camera or blocking render loops.
- **Dynamic Config Hot-Reload**: Thresholds and cadences update without restarting pipeline workers or recompiling.

---

## Architecture

```
vigilo-core/
├── src/
│   ├── lib.rs              # Public library API & re-exports
│   ├── capture/            # Camera (DirectShow/ffmpeg), video file & MJPEG replay sources
│   ├── pipeline/           # Triple-buffered ArcSwap frame bus, background worker threads
│   ├── models/             # ONNX Runtime model wrappers (YuNet, Gaze, Pose, Objects, ArcFace)
│   ├── fusion/             # Temporal decision engine: HoldTimer, Hysteresis, DecayingScore, Ema
│   ├── config.rs           # Threshold configuration & validation
│   ├── config_store.rs     # Zero-dependency TOML settings persistence
│   ├── direction.rs        # Angular head pose & gaze bucketing
│   ├── types.rs            # Signals, Frame, Event, BBox, Violation types
│   ├── report.rs           # Latency metrics & session summary generators
│   ├── error.rs            # Strictly typed error handling (thiserror)
│   └── bin/
│       └── detect-cli.rs   # Standalone CLI test & benchmark harness
├── tests/                  # Deterministic replay and integration test suite
└── models/                 # Pre-trained ONNX model files
```

---

## Developer Harness (`detect-cli`)

`detect-cli` runs headless benchmarks, recording, and replay without needing a camera or GUI:

```bash
# Benchmark all models (p50 / p95 latencies)
cargo run --release --bin detect-cli -- bench --all --iters 100

# Live camera evaluation with overlay console
cargo run --bin detect-cli -- live --source camera:0

# Record signals to JSONL
cargo run --bin detect-cli -- record --source file:clip.mp4 --out signals.jsonl

# Replay recorded session through fusion engine with zero model execution
cargo run --bin detect-cli -- replay signals.jsonl --expect phone@2.1s
```

---

## Testing

Tests run using `cargo-nextest` and the `rust-lld` linker:

```bash
cargo nextest run
```

---

## License

GNU Affero General Public License v3.0 (`AGPL-3.0-only`). See [LICENSE](LICENSE) for details.
