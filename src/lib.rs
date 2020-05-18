//! This library provides a custom [`eyre::EyreContext`] type for usage with [`eyre`] that provides
//! a minimal error report with no additional context. Essentially the minimal implementation of an
//! error reporter.
//!
//! # Example
//!
//! ```rust,should_panic
//! use eyre::{eyre, WrapErr};
//! use simple_eyre::Report;
//!
//! fn main() -> Result<(), Report> {
//!     let e: Report = eyre!("oh no this program is just bad!");
//!
//!     Err(e).wrap_err("usage example successfully experienced a failure")
//! }
//! ```
//!
//! [`eyre::EyreContext`]: https://docs.rs/eyre/0.3.8/eyre/trait.EyreContext.html
//! [`eyre`]: https://docs.rs/eyre
//! [`backtrace::Backtrace`]: https://docs.rs/backtrace/0.3.46/backtrace/struct.Backtrace.html
use eyre::Chain;
use eyre::EyreContext;
use indenter::indented;
use std::error::Error;

/// A custom context type for minimal error reporting via `eyre`
#[derive(Debug)]
pub struct Context;

impl EyreContext for Context {
    #[allow(unused_variables)]
    fn default(error: &(dyn Error + 'static)) -> Self {
        Self
    }

    fn debug(
        &self,
        error: &(dyn Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        use core::fmt::Write as _;

        if f.alternate() {
            return core::fmt::Debug::fmt(error, f);
        }

        write!(f, "{}", error)?;

        if let Some(cause) = error.source() {
            write!(f, "\n\nCaused by:")?;
            let multiple = cause.source().is_some();
            for (n, error) in Chain::new(cause).enumerate() {
                writeln!(f)?;
                if multiple {
                    write!(indented(f).ind(n), "{}", error)?;
                } else {
                    write!(indented(f), "{}", error)?;
                }
            }
        }

        Ok(())
    }
}

/// A type alias for `eyre::Report<stable_eyre::Context>`
///
/// # Example
///
/// ```rust
/// use stable_eyre::Report;
///
/// # struct Config;
/// fn try_thing(path: &str) -> Result<Config, Report> {
///     // ...
/// # Ok(Config)
/// }
/// ```
pub type Report = eyre::Report<Context>;

/// A type alias for `Result<T, stable_eyre::Report>`
///
/// # Example
///
///```
/// fn main() -> stable_eyre::Result<()> {
///
///     // ...
///
///     Ok(())
/// }
/// ```
pub type Result<T, E = Report> = core::result::Result<T, E>;
