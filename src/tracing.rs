//! # Tracing
//!
//! AgIsoStack-rs provides tracing under the non-default `tracing` Cargo feature. Tracing is
//! intended for use by Crate maintainers for introspection. We use the [tracing] crate, but rather
//! than invoke `tracing`s macros directly, we provide our own shim layer, so that we don't need to
//! sprinkle `#[cfg(feature = "tracing")]` a bazillion places throughout the crate.

#[cfg(feature = "tracing")]
pub use tracing::*;

/// A conditional compilation shim around [tracing::debug!]
// NOTE: I tried to make this mess less gross by implementing the macro like
//
// macro_rules! debug {
//     ( $($all_tokens:tt)* ) => {
//         #[cfg(feature = "tracing")]
//         ::tracing::debug!($($all_tokens)*)
//     };
// }
//
// with various combinations of semi-colons and curly braces. No matter what, I couldn't find a way
// to get this macro to correctly expand when using the log statements as expressions as in
//
// match foo() {
//     Ok(x) => tracing::debug!("Ok: {x:?}"),
//     Err(e) => tracing::debug("Err: {e:?}"),
// }
//
// So here, I hoist the #[cfg] attribute up a level to the actual macro definition. It's gross, way
// more typing than I wanted, but it works.
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __debug {
    ( $($all_tokens:tt)* ) => { ::tracing::debug!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __debug {
    ( $($all_tokens:tt)* ) => {{}};
}
// HACK: This is a gross and dirty hack that shouldn't need to exist. When you use #[macro_export],
// that exports the macro under the top-level crate module for technical reasons (I think because
// when macros are expanded the module tree doesn't exist yet?). This re-export trick came from the
// rust-analyzer source code (thanks matklad!) and makes these macros available under the
// crate::tracing module.
pub use __debug as debug;

/// A conditional compilation shim around [tracing::debug_span!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __debug_span {
    ( $($all_tokens:tt)* ) => { ::tracing::debug_span!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __debug_span {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __debug_span as debug_span;

/// A conditional compilation shim around [tracing::enabled!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __enabled {
    ( $($all_tokens:tt)* ) => { ::tracing::enabled!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __enabled {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __enabled as enabled;

/// A conditional compilation shim around [tracing::error!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __error {
    ( $($all_tokens:tt)* ) => { ::tracing::error!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __error {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __error as error;

/// A conditional compilation shim around [tracing::error_span!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __error_span {
    ( $($all_tokens:tt)* ) => { ::tracing::error_span!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __error_span {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __error_span as error_span;

/// A conditional compilation shim around [tracing::event!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __event {
    ( $($all_tokens:tt)* ) => { ::tracing::event!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __event {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __event as event;

/// A conditional compilation shim around [tracing::event_enabled!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __event_enabled {
    ( $($all_tokens:tt)* ) => { ::tracing::event_enabled!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __event_enabled {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __event_enabled as event_enabled;

/// A conditional compilation shim around [tracing::info!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __info {
    ( $($all_tokens:tt)* ) => { ::tracing::info!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __info {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __info as info;

/// A conditional compilation shim around [tracing::info_span!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __info_span {
    ( $($all_tokens:tt)* ) => { ::tracing::info_span!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __info_span {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __info_span as info_span;

/// A conditional compilation shim around [tracing::span!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __span {
    ( $($all_tokens:tt)* ) => { ::tracing::span!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __span {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __span as span;

/// A conditional compilation shim around [tracing::span_enabled!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __span_enabled {
    ( $($all_tokens:tt)* ) => { ::tracing::span_enabled!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __span_enabled {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __span_enabled as span_enabled;

/// A conditional compilation shim around [tracing::trace!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __trace {
    ( $($all_tokens:tt)* ) => { ::tracing::trace!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __trace {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __trace as trace;

/// A conditional compilation shim around [tracing::trace_span!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __trace_span {
    ( $($all_tokens:tt)* ) => { ::tracing::trace_span!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __trace_span {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __trace_span as trace_span;

/// A conditional compilation shim around [tracing::warn!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __warn {
    ( $($all_tokens:tt)* ) => { ::tracing::warn!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __warn {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __warn as warn;

/// A conditional compilation shim around [tracing::warn_span!]
#[macro_export]
#[cfg(feature = "tracing")]
macro_rules! __warn_span {
( $($all_tokens:tt)* ) => { ::tracing::warn_span!($($all_tokens)*) };
}
#[macro_export]
#[cfg(not(feature = "tracing"))]
macro_rules! __warn_span {
    ( $($all_tokens:tt)* ) => {{}};
}
pub use __warn_span as warn_span;
