use vector_config::configurable_component;

mod basic;
pub use self::basic::BasicTransformConfig;

mod noop;
pub use self::noop::NoopTransformConfig;

/// Transform types.
#[configurable_component]
#[derive(Clone, Debug)]
pub enum TransformType {
    /// Transforms an individual event.
    Function,

    /// Transforms an individual event, but can send the transformed event(s) to different
    /// downstream outputs.
    Synchronous,

    /// Transforms events in an asynchronous iterator.
    Task,
}
