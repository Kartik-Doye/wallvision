//! WiFlow-STD â€” spatio-temporal-decoupled CSI pose estimation (ADR-152 Â§2.2).
//!
//! Native Rust port of the **WiFlow-STD** architecture by DY2434
//! (<https://github.com/DY2434/WiFlow-WiFi-Pose-Estimation-with-Spatio-Temporal-Decoupling>,
//! Apache-2.0), reimplemented idiomatically from the vendored read-only
//! reference in `benchmarks/wiflow-std/upstream/models/`.
//!
//! ## Evidence grade (ADR-152 Â§2.2 citation rule)
//!
//! Per `benchmarks/wiflow-std/RESULTS.md`, the upstream accuracy claims are
//! **MEASURED-EQUIVALENT**: our retraining of the reference implementation on
//! the released dataset reproduced **~96% PCK@20** (96.09% full test / 96.61%
//! corruption-free; published claim 97.25%). The *shipped* upstream checkpoint
//! was REFUTED (0.08% PCK@20 â€” keypoint-convention mismatch), and the released
//! dataset/code required repairs before training converged. Cite this port as
//! "~96% PCK@20 (our reproduction)" â€” **not comparable** to RuView's
//! 17-keypoint ESP32 numbers (different hardware, subjects, split, skeleton).
//!
//! ## Name collision
//!
//! WiFlow-STD (this module) is the *external* DY2434 architecture. It is
//! **distinct from RuView's internal WiFlow** camera-free pose pipeline; the
//! `_std` suffix (Spatio-Temporal Decoupling) disambiguates the two.
//!
//! ## Architecture
//!
//! ```text
//! CSI window [B, 540 sub, 20 t]
//!   â”‚  TCN stack: 4 Ã— grouped TemporalBlock (groups=20, k=3, dilation 1/2/4/8,
//!   â”‚             depthwise-grouped + pointwise convs, causal Chomp1d padding)
//!   â–¼  channels 540 â†’ 540 â†’ 440 â†’ 340 â†’ 240
//! [B, 240, 20] â”€â”€ transpose+unsqueeze â”€â”€â–º [B, 1, 20, 240]   (image-like)
//!   â”‚  ConvBlock1 (1â†’8, asymmetric 1Ã—3 kernels, no downsampling)
//!   â”‚  4 Ã— AsymmetricConvBlock (8â†’8â†’16â†’32â†’64, stride (1,2) on subcarrier axis)
//!   â–¼
//! [B, 64, 20, 15] â”€â”€ permute â”€â”€â–º [B, 64, 15, 20]
//!   â”‚  DualAxialAttention (64 ch, 8 groups, width- then height-axial
//!   â”‚  self-attention with BN-normalised qkv and BN-normalised similarity)
//!   â”‚  Decoder convs 64 â†’ 32 â†’ 2 (3Ã—3 then 1Ã—1, BN + SiLU)
//!   â–¼
//! [B, 2, 15, 20] â”€â”€ adaptive avg-pool (K, 1) â”€â”€â–º [B, K, 2] keypoints
//! ```
//!
//! 2,225,042 parameters / ~0.055 GFLOPs at the 15-keypoint default
//! (both verified against the reference â€” see `RESULTS.md`).
//!
//! Note: upstream `config.py` lists `TCN_CHANNELS = [480, 360, 240]`, but the
//! released checkpoint and `models/` code use `[540, 440, 340, 240]`. This
//! port follows the `models/` code, which we verified loads the released
//! weights after key remapping.
//!
//! ## Feature gating
//!
//! [`WiFlowStdConfig`] (validation, parameter-count formula, output-shape
//! inference) is pure Rust and always available. [`model::WiFlowStdModel`]
//! (the tch / LibTorch forward pass) requires the `tch-backend` feature,
//! matching [`crate::model`]'s gating.

pub mod config;

#[cfg(feature = "tch-backend")]
mod layers;
#[cfg(feature = "tch-backend")]
pub mod model;

pub use config::{TcnGroupsMode, WiFlowStdConfig};

#[cfg(feature = "tch-backend")]
pub use model::WiFlowStdModel;

