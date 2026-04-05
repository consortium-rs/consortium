mod compile;

// Trait
pub use compile::Compiler;

// Backward-compatible builders (String-error API)
pub use compile::application::ApplicationBuilder;
pub use compile::firmware::FirmwareBuilder;
pub use compile::optee::{OpteeBuilder, OpteeEnvironment};

// Typed compiler API
pub use compile::application::{ApplicationCompileError, ApplicationCompiler, ApplicationConfiguration};
pub use compile::firmware::{FirmwareCompileError, FirmwareCompiler, FirmwareConfiguration};
pub use compile::optee::{OpteeCompileError, OpteeCompiler, OpteeConfiguration, OPTEE_TARGET};
