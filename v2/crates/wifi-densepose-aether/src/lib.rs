//! AETHER pure-compute stack (ADR-024 / ADR-185 Â§3.2).
//!
//! This crate is the dependency-free leaf hoisted out of
//! `wifi-densepose-sensing-server` so that the Python `wifi_densepose[aether]`
//! wheel can bind the contrastive-embedding surface without linking the server's
//! Axum / tokio / worldgraph / ruvector tree (which blew the ADR-117 Â§5.4 â‰¤5 MB
//! wheel budget).
//!
//! Modules:
//! - [`embedding`] â€” AETHER contrastive CSI embedding: `EmbeddingConfig`,
//!   `EmbeddingExtractor`, `ProjectionHead`, `CsiAugmenter`, `info_nce_loss`,
//!   fingerprint indices.
//! - [`graph_transformer`] â€” CSI-to-pose transformer primitives
//!   (`CsiToPoseTransformer`, `TransformerConfig`, `Linear`).
//! - [`sona`] â€” self-organizing drift detection + LoRA adaptation + EWC.
//! - [`sparse_inference`] â€” quantization helpers used by the embedding path.
//!
//! `wifi-densepose-sensing-server` re-exports these modules so its own code and
//! public API are unchanged.

// `embedding` carries a couple of not-yet-read fields (e.g. `PoseEncoder.d_proj`);
// this mirrors the `#[allow(dead_code)]` the module had at its previous home in
// `wifi-densepose-sensing-server`.
#[allow(dead_code)]
pub mod embedding;
pub mod graph_transformer;
pub mod sona;
pub mod sparse_inference;

