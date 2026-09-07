//! # wifi-densepose-calibration â€” ADR-151 per-room calibration & specialist training
//!
//! "Teach the room before you teach the model." A local-first pipeline that turns
//! a few minutes of clean human anchors â€” layered on the ADR-135 empty-room
//! baseline â€” into a versioned bank of small, specialised models for breathing,
//! heartbeat, restlessness, posture, presence, and anomaly.
//!
//! Stages (ADR-151 Â§1.3):
//! 1. **baseline** â€” empty-room environmental fingerprint (ADR-135; consumed here).
//! 2. **enroll** â€” guided anchors with an adaptive quality gate ([`anchor`],
//!    [`enrollment`]) plus an optional transceiver-geometry record ([`geometry`],
//!    ADR-152 Â§2.1.1) and its fixed-length conditioning featurization
//!    ([`geometry_embedding`], ADR-152 Â§2.1.2).
//! 3. **extract** â€” labelled feature records from anchor captures ([`extract`]).
//! 4. **train** â€” a bank of small specialist models ([`specialist`], [`bank`]) and a
//!    confidence-gated mixture runtime ([`runtime`]).
//!
//! Invariants: specialisation over scale; local-first; honest `STALE` degradation
//! when the baseline drifts.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod anchor;
pub mod bank;
pub mod certificate;
pub mod enrollment;
pub mod error;
pub mod extract;
pub mod geometry;
pub mod geometry_embedding;
pub mod multistatic;
pub mod runtime;
pub mod specialist;

pub use anchor::{Anchor, AnchorLabel, AnchorQuality, EnrollmentEvent, EnrollmentSession, Posture};
pub use bank::SpecialistBank;
pub use certificate::{
    CalibrationCertificate, CalibrationTier, CertificateSignature, CertificateSigner,
    CertificateStatus, CertificateVerifier, CharacterizationSource, CompatibilityEnvelope,
    EvidenceLevel, FingerprintDistance, KeyedHashSigner, MintParams, RoomFingerprint,
};
pub use enrollment::{AnchorQualityGate, AnchorRecorder};
pub use error::{CalibrationError, Result};
pub use extract::AnchorFeature;
pub use geometry::{AntennaOrientation, NodeGeometry, PositionEstimate};
pub use geometry_embedding::GeometryEmbedding;
pub use multistatic::MultiNodeMixture;
pub use runtime::{MixtureOfSpecialists, RoomState};
pub use specialist::{Specialist, SpecialistKind, SpecialistReading};

