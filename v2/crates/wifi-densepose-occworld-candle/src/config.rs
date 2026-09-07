//! OccWorld model configuration.
//!
//! All constants match the Python reference implementation in
//! `OccWorld/model/occworld.py`.  Changing a value here must be
//! reflected in a matching weight checkpoint, because the tensor
//! shapes are baked into the SafeTensors file.

/// Complete configuration for the OccWorld TransVQVAE model.
///
/// The defaults reproduce the published 72.4 M-parameter config used during
/// training on nuScenes.  Pass a custom `OccWorldConfig` to `OccWorldCandle`
/// when loading a fine-tuned checkpoint with different hyper-parameters.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OccWorldConfig {
    // â”€â”€ Voxel grid â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    /// Grid width (X-axis). Python: `occ_size[0]` = 200.
    pub grid_h: usize,
    /// Grid depth (Y-axis). Python: `occ_size[1]` = 200.
    pub grid_w: usize,
    /// Grid height (Z-axis). Python: `occ_size[2]` = 16.
    pub grid_d: usize,

    // â”€â”€ Semantic labels â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    /// Total number of semantic classes (0-17). nuScenes: 18.
    pub num_classes: usize,
    /// Class index reserved for "free space / unknown". nuScenes: 17.
    pub free_class: u8,

    // â”€â”€ VQVAE dimensions â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    /// Base channel count for the encoder/decoder ResNet blocks.
    /// Embedding dimension per voxel position: 18 classes â†’ 64-dim vectors.
    pub base_channels: usize,
    /// Latent channels produced by the encoder (z). Python: 128.
    pub z_channels: usize,

    // â”€â”€ Vector-quantisation codebook â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    /// Number of discrete codes in the codebook. Python: 512.
    pub codebook_size: usize,
    /// Dimension of each codebook entry. Python: 512.
    pub embed_dim: usize,

    // â”€â”€ Temporal / spatial layout â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    /// Number of past occupancy frames used as context. Python: 15.
    pub num_frames: usize,
    /// Token grid height after VQVAE encoder (H/4). Python: 50.
    pub token_h: usize,
    /// Token grid width after VQVAE encoder (W/4). Python: 50.
    pub token_w: usize,

    // â”€â”€ Transformer â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    /// Number of attention heads in the transformer.
    pub num_heads: usize,
    /// Number of encoder layers in the UNet-style transformer.
    pub num_layers: usize,
    /// Feed-forward hidden size inside each transformer layer.
    pub ffn_hidden: usize,
}

impl Default for OccWorldConfig {
    fn default() -> Self {
        Self {
            grid_h: 200,
            grid_w: 200,
            grid_d: 16,
            num_classes: 18,
            free_class: 17,
            base_channels: 64,
            z_channels: 128,
            codebook_size: 512,
            embed_dim: 512,
            num_frames: 15,
            token_h: 50,
            token_w: 50,
            num_heads: 8,
            num_layers: 2,
            ffn_hidden: 2048,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let cfg = OccWorldConfig::default();
        assert_eq!(cfg.grid_h, 200);
        assert_eq!(cfg.grid_w, 200);
        assert_eq!(cfg.grid_d, 16);
        assert_eq!(cfg.num_classes, 18);
        assert_eq!(cfg.free_class, 17);
        assert_eq!(cfg.base_channels, 64);
        assert_eq!(cfg.z_channels, 128);
        assert_eq!(cfg.codebook_size, 512);
        assert_eq!(cfg.embed_dim, 512);
        assert_eq!(cfg.num_frames, 15);
        assert_eq!(cfg.token_h, 50);
        assert_eq!(cfg.token_w, 50);
    }
}

