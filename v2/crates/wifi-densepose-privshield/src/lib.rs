//! # VEIL â€” a compliant-waveform privacy shield against WiFi sensing
//!
//! VEIL (Verifiable Emission-shaping for Identity-Leakage prevention) is the
//! countermeasure counterpart to BFLD (ADR-118/121, `wifi-densepose-bfld`).
//! Where BFLD *detects* when beamforming feedback becomes identifying, VEIL
//! *acts*: it shapes a node's own outgoing beamforming feedback so that an
//! unauthorized passive sniffer cannot re-identify people or infer activity,
//! while a legitimate receiver â€” which shares the per-session key â€” sees an
//! essentially unchanged link.
//!
//! This crate is a **deterministic, dependency-free, WASM-ready reference and
//! experiment**, not a radio driver. It models the physics faithfully enough to
//! measure the core claim, and it never emits RF. Per ADR-288 and CLAUDE.md,
//! every number it produces is `SYNTHETIC`, reproduced by
//! `cargo test -p wifi-densepose-privshield`.
//!
//! ## The idea in one paragraph
//!
//! Identity leaks through the *fine* cross-subcarrier phase structure of a
//! compressed beamforming report; data throughput rides the *dominant* beam
//! direction. These live in (mostly) separable subspaces. VEIL composes extra
//! keyed [`linalg::apply_givens`] rotations â€” the exact primitive the report is
//! already built from â€” over the **fine** subspace only. The rotation is:
//! orthogonal (energy-preserving â‡’ no added transmit power â‡’ **not jamming**,
//! [`compliance`]); keyed per session (the legitimate AP inverts it â‡’
//! throughput preserved, [`throughput`]); and fresh each session (a sniffer
//! sees a different rotation every time and cannot average back the signature
//! â‡’ re-identification collapses to chance, [`attacker`]/[`experiment`]).
//!
//! ## Threat model and scope (stated plainly)
//!
//! VEIL defends against a **third-party passive sniffer** capturing
//! plaintext beamforming feedback. It does **not** hide identity from the AP a
//! node is associated with (that party holds the key). It is **compliant by
//! construction**: it only shapes the node's own standards-conformant frames;
//! it never transmits to interfere with another station (47 U.S.C. Â§333) and
//! never operates an unauthorized emitter (Â§302a). It is not jamming, not RF
//! denial, and not a claim of camera-grade anything.
//!
//! ## Modules
//!
//! - [`prng`] â€” deterministic, WASM-safe PRNG and key derivation.
//! - [`linalg`] â€” the small Givens-rotation vector algebra.
//! - [`identity`] â€” the SYNTHETIC two-subspace beamforming-feedback model.
//! - [`protector`] â€” the compliant waveform controls (the shield).
//! - [`attacker`] â€” the passive re-identification adversary.
//! - [`throughput`] â€” the link-throughput model.
//! - [`compliance`] â€” the machine-checkable "not jamming" audit.
//! - [`experiment`] â€” the attacker-vs-protector head-to-head.
//! - [`proof`] â€” the byte-stable deterministic witness.
//!
//! ## Quick start
//!
//! ```
//! use wifi_densepose_privshield::experiment::{run, ExperimentConfig};
//!
//! let report = run(&ExperimentConfig::default());
//! assert!(report.attack_is_effective_without_shield()); // threat is real
//! assert!(report.drives_to_chance());                   // shield collapses re-ID
//! assert!(report.preserves_throughput());               // throughput â‰¥ 95%
//! assert!(report.compliance.is_compliant());            // energy-preserving
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod attacker;
pub mod compliance;
pub mod experiment;
pub mod identity;
pub mod linalg;
pub mod optimize;
pub mod prng;
pub mod proof;
pub mod protector;
pub mod throughput;

pub use attacker::{
    AdaptivePoolingAttacker, AttackerKind, Metric, NearestCentroidAttacker, ReconstructionAttacker,
};
pub use compliance::ComplianceReport;
pub use experiment::{run, ExperimentConfig, ExperimentReport};
pub use identity::{BfiSample, Channel, SceneConfig};
pub use optimize::{adaptive_shield, hyper_optimize, HyperOptimized};
pub use proof::Proof;
pub use protector::{ObfMode, Protector, SensingDetector, ShieldConfig};
pub use throughput::LinkModel;

