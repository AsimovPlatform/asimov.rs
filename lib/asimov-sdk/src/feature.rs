// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "derive")]
    "derive",
    #[cfg(feature = "tokio")]
    "tokio",
    #[cfg(feature = "tracing")]
    "tracing",
];
