//! Multi-AP Triage (MAT) disaster-detection module â€” RuVector integrations.
//!
//! This module provides three ADR-017 integration points for the MAT pipeline:
//!
//! - [`triangulation`]: TDoA-based survivor localisation via
//!   ruvector-solver (`NeumannSolver`).
//! - [`breathing`]: Tiered compressed streaming breathing buffer via
//!   ruvector-temporal-tensor (`TemporalTensorCompressor`).
//! - [`heartbeat`]: Per-frequency-bin tiered compressed heartbeat spectrogram
//!   via ruvector-temporal-tensor.
//!
//! # Memory reduction
//!
//! For 56 subcarriers Ã— 60 s Ã— 100 Hz:
//! - Raw: 56 Ã— 6 000 Ã— 4 bytes = **13.4 MB**
//! - Hot tier (8-bit): **3.4 MB**
//! - Mixed hot/warm/cold: **3.4â€“6.7 MB** depending on recency distribution.

pub mod breathing;
pub mod heartbeat;
pub mod triangulation;

pub use breathing::CompressedBreathingBuffer;
pub use heartbeat::CompressedHeartbeatSpectrogram;
pub use triangulation::solve_triangulation;

