#[cfg(not(target_family = "wasm"))]
pub mod uniffi_ffi {
    use crate::app::App;
    use crux_core::Core;
    
    pub type CoreFFI = Core<App>;
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub mod wasm_ffi {
    use crate::app::App;
    use crux_core::Core;
    
    pub type CoreFFI = Core<App>;
}

#[cfg(all(target_os = "wasi", target_env = "p2"))]
pub mod wasip2 {
    use crate::app::App;
    use crux_core::Core;
    
    pub type CoreFFI = Core<App>;
}
