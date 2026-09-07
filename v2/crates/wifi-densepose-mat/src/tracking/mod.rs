//! Survivor track lifecycle management for the MAT crate.
//!
//! Implements three collaborating components:
//!
//! - **[`KalmanState`]** â€” constant-velocity 3-D position filter
//! - **[`CsiFingerprint`]** â€” biometric re-identification across signal gaps
//! - **[`TrackLifecycle`]** â€” state machine (Tentativeâ†’Activeâ†’Lostâ†’Terminated)
//! - **[`SurvivorTracker`]** â€” aggregate root orchestrating all three
//!
//! # Example
//!
//! ```rust,no_run
//! use wifi_densepose_mat::tracking::{SurvivorTracker, TrackerConfig, DetectionObservation};
//!
//! let mut tracker = SurvivorTracker::with_defaults();
//! let observations = vec![]; // DetectionObservation instances from sensing pipeline
//! let result = tracker.update(observations, 0.5); // dt = 0.5s (2 Hz)
//! println!("Active survivors: {}", tracker.active_count());
//! ```

pub mod fingerprint;
pub mod kalman;
pub mod lifecycle;
pub mod tracker;

pub use fingerprint::CsiFingerprint;
pub use kalman::KalmanState;
pub use lifecycle::{TrackLifecycle, TrackState, TrackerConfig};
pub use tracker::{
    AssociationResult, DetectionObservation, SurvivorTracker, TrackId, TrackedSurvivor,
};

