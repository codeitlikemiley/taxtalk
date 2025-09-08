pub mod traits;
pub mod types;
pub mod manifest;
pub mod router;
pub mod loader;
pub mod system;
pub mod wit_loader;
pub mod host_bindings;

// Re-export specific items to avoid ambiguous re-exports
pub use traits::{
    Plugin, PluginLifecycle, CanHandleResult, CommandRegistration, 
    CommandRequirements, TokenRequirement, TokenDefault, CommandValidator,
    ExecutionResult, JournalEntry, JournalLine, Artifact
};

pub use types::{
    LoadedPlugin, PluginInstance, PluginState, TermsToken, TermType,
    CommandRequest, CommandResponse, PluginError, PluginCapabilities
    // Note: types also has PluginDependency but we'll use manifest's version
};

pub use manifest::{
    PluginManifest, PluginType, CommandManifest, RequirementsManifest,
    TokenRequirementManifest, TokenDefaultManifest, TokenTypeManifest,
    PluginDependency, PluginMetadata
};

pub use router::CommandRouter;
pub use loader::PluginLoader;
pub use system::PluginSystem;