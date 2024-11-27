//! Style types for Cassowary layout
use crate::CoreStyle;

/// The set of styles required for a Cassowary layout container
pub trait CassowaryContainerStyle: CoreStyle {}

/// The set of styles required for a Cassowary layout item (child of a Cassowary container)
pub trait CassowaryItemStyle: CoreStyle {}
