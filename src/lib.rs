//! `vigilo-core` — camera frames in, proctoring signals and violations out.
//!
//! High-performance multimodal exam proctoring and behavioral stream fusion
//! engine in pure Rust. Zero UI, zero browser, zero Tauri.
//!
//! Originally developed as `deepscreen-detect`, battle-tested inside the
//! `vigilo` desktop app (`deepscreen-viewer`), and now extracted as the pure
//! engine foundation powering:
//! - Native CLI harnesses (`detect-cli`)
//! - Python PyPI package (`rustream` via PyO3 / Maturin)
//! - Node.js npm package (`flapguard` via napi-rs)
//! - Desktop applications (Vigilo / Tauri)
//!
//! # Architecture
//!
//! - [`capture`]: High-throughput frame sources (DirectShow camera, video files, MJPEG/directory replay)
//! - [`pipeline`]: Lock-free `ArcSwap` triple-buffered frame bus, multithreaded detection workers
//! - [`models`]: ONNX inference wrappers (YuNet face, GazeNet gaze, HeadPoseNet pose, YoloxNano objects, ArcFace identity)
//! - [`fusion`]: Pure deterministic temporal decision engine ([`FusionEngine`]) with hysteresis, hold timers, and decaying scores
//! - [`direction`]: Robust head yaw/pitch angular coordinate bucketing
//! - [`config`]: User and developer threshold configurations with dynamic hot-reloading
//! - [`config_store`]: Zero-dependency TOML settings persistence
//! - [`types`]: Strictly-typed signals, events, bounding boxes, and violations

pub mod capture;
pub mod config;
pub mod config_store;
pub mod direction;
pub mod error;
pub mod fusion;
pub mod models;
pub mod pipeline;
pub mod report;
pub mod types;

pub use capture::{FrameSource, SourceSpec};
pub use config::{Config, DevThresholds, SettingsPayload, UserThresholds};
pub use direction::{Axes, DebugDirections, DirectionTracker, FrameOfReference, Horizontal, Vertical};
pub use error::{DetectError, Result};
pub use fusion::FusionEngine;
pub use pipeline::{Detected, Detector, DetectorBuilder};
pub use report::{FrameStats, Latencies, LatencySummary, SessionReport, SignalStatus};
pub use types::{
    BBox, Contribution, DegradeReason, DetectorState, Event, EyeAspect, FaceDetection,
    FaceKeypoints, Frame, GateReason, Gaze, HeadPose, ObjectDetection, Severity, SignalCoverage,
    SignalSource, Signals, SlotState, Violation, ViolationKind,
};

/// Version of the `Signals` JSONL format. Bump when a change would make an
/// old recording replay to different violations — recordings are the
/// regression corpus, and silently reinterpreting them would be worse than
/// refusing to read them.
///
/// **2**: `SignalCoverage` went from five booleans to five [`SlotState`]s.
pub const SIGNALS_FORMAT_VERSION: u32 = 2;
