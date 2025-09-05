pub mod traits;
pub mod base;
pub mod types;
pub mod specialized;
pub mod composite;
pub mod factory;
pub mod validation;
pub mod patterns;
pub mod philippine;

#[cfg(test)]
mod tests;

pub use traits::*;
pub use base::*;
pub use types::*;
pub use specialized::*;
pub use composite::*;
pub use factory::*;
pub use validation::*;
pub use patterns::*;
pub use philippine::*;