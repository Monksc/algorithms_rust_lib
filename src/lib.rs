// pub mod algorithms::seen_before;

// pub(super) mod algorithms;
pub mod algorithms;
pub use crate::algorithms::*;

pub mod graph;
pub use crate::graph::*;

pub mod other;
pub use crate::other::*;

// mods::mods! {
//     pub(super) algorithms; // Visible to the parent module
// }


#[cfg(test)]
mod tests {
    use super::*;
}
