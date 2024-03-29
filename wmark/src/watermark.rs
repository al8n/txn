#[cfg(feature = "std")]
mod std_;
#[cfg(feature = "std")]
pub use std_::*;

/// Watermark implementations for generic async runtime.
#[cfg(feature = "future")]
#[cfg_attr(docsrs, doc(cfg(feature = "future")))]
pub mod future;

/// Watermark implementations for the `tokio` runtime.
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod tokio;

/// Watermark implementations for no_std environments.
#[cfg(feature = "core")]
#[cfg_attr(docsrs, doc(cfg(feature = "core")))]
pub(crate) mod no_std;

/// Error type for watermark.
pub enum WaterMarkError {
  /// The watermark is uninitialized, please call init first before using any other functions
  Uninitialized,
  /// The watermark is canceled.
  Canceled,
}

impl core::fmt::Debug for WaterMarkError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Uninitialized => write!(
        f,
        "watermark: uninitialized, please call init first before using any other functions"
      ),
      Self::Canceled => write!(f, "watermark: canceled"),
    }
  }
}

impl core::fmt::Display for WaterMarkError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Uninitialized => write!(
        f,
        "watermark: uninitialized, please call init first before using any other functions"
      ),
      Self::Canceled => write!(f, "watermark: canceled"),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for WaterMarkError {}
