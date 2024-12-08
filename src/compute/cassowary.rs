//! Computes size using styles and measure functions

use crate::geometry::{Point, Size};
use crate::style::{AvailableSpace, Overflow, Position};
use crate::tree::{CollapsibleMarginSet, RunMode};
use crate::tree::{LayoutInput, LayoutOutput, SizingMode};
use crate::util::debug::debug_log;
use crate::util::sys::f32_max;
use crate::util::MaybeMath;
use crate::util::{MaybeResolve, ResolveOrZero};
use crate::{BoxSizing, CoreStyle, LayoutBlockContainer, LayoutCassowaryContainer, NodeId};
use cassowary::{
    strength::{REQUIRED, STRONG, WEAK},
    Constraint, Expression, RelationalOperator, Solver, Variable,
    WeightedRelation::*,
};
use core::unreachable;

/// Compute the size of a leaf node (node with no children)
pub fn compute_cassowary_layout(
    tree: &mut impl LayoutCassowaryContainer,
    node_id: NodeId,
    inputs: LayoutInput,
    // constraints: Vec<Constraint>,
) -> LayoutOutput {
    let LayoutInput { known_dimensions, parent_size, available_space, sizing_mode, run_mode, .. } = inputs;

    todo!()
}
