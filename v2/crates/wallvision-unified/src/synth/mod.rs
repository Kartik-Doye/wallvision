//! Physics-guided synthetic RF world generator (ADR-276).
//!
//! Generates labeled CSI windows from first-principles multipath physics â€”
//! shoebox rooms via the Allenâ€“Berkley image method (reflection order â‰¤ 2),
//! Fresnel wall materials with complex permittivity, moving people as
//! bistatic point scatterers (Doppler emerges from path-length change, it is
//! never injected), plus domain randomization of the *physics* parameters
//! (materials, geometry, chipset gain/phase/noise, CFO, packet loss,
//! interference) rather than cosmetic noise.
//!
//! Everything is seeded ChaCha20-deterministic, and every tensor produced
//! here is stamped [`crate::tensor::RfModality::Synthetic`] â€” the honest
//! label ADR-276 requires until results are validated on measured data.
//!
//! Physics anchors proven in tests:
//! - direct path amplitude â‰¡ Friis (`raytrace::tests::direct_path_is_exact_friis`)
//! - reciprocity `H(aâ†’b) = H(bâ†’a)`
//! - first-order reflection delay â‰¡ mirror-image geometry
//! - a walking person produces the analytically expected Doppler phase rate

pub mod generator;
pub mod raytrace;
pub mod room;

pub use generator::{LabeledWindow, SynthConfig, SynthGenerator};
pub use raytrace::{enumerate_paths, synthesize_csi, PathContribution};
pub use room::{Material, PersonSpec, RoomSpec};

