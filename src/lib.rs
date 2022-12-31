// pub mod algorithms::seen_before;

// pub(super) mod algorithms;
pub mod algorithms;
pub use crate::algorithms::*;

pub mod bfs;
pub use crate::bfs::*;

// mods::mods! {
//     pub(super) algorithms; // Visible to the parent module
// }


#[cfg(test)]
mod tests {
    use super::*;
}
