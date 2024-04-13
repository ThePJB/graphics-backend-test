mod input;
mod program;
mod vertex;
mod game_trait;

#[cfg(not(target_arch = "wasm32"))]
mod run_winit;
#[cfg(target_arch = "wasm32")]
mod run_wasm;

pub use input::*;
#[cfg(not(target_arch = "wasm32"))]
pub use run_winit::*;
#[cfg(target_arch = "wasm32")]
pub use run_wasm::*;
pub use vertex::*;
pub use program::*;
pub use game_trait::*;