//! # Easy-error
//!
//! This crate is a lightweight error handling library meant to play well with
//! the standard `Error` trait. It is designed for quick prototyping or for
//! Command-line applications where any error will simply bubble up to the user.
//! There are four major components of this crate:
//!
//! 1. A basic, string-based error type that is meant for either quick
//!    prototyping or human-facing errors.
//! 2. A nice way to iterate over the causes of an error.
//! 3. Some macros that make returning errors slightly more ergonomic.
//! 4. A "termination" type that produces nicely formatted error messages when
//!    returned from the `main` function.
//!
//! ## Rust Version Requirements
//!
//! The current version requires **Rustc 1.32 or newer**.
//! In general, this crate will be compilable with the Rustc version available
//! on the oldest Ubuntu LTS release. Any change that requires a new Rustc
//! version will be considered a breaking change and will be handled
//! accordingly.
//!
//! ## Example
//!
//! ```no_run
//! use std::{fs::File, io::Read};
//! use easy_error::{bail, ensure, Error, ResultExt, Terminator};
//!
//! fn from_file() -> Result<i32, Error> {
//!     let file_name = "example.txt";
//!     let mut file = File::open(file_name).context("Could not open file")?;
//!
//!     let mut contents = String::new();
//!     file.read_to_string(&mut contents).context("Unable to read file")?;
//!
//!     contents.trim().parse().context("Could not parse file")
//! }
//!
//! fn validate(value: i32) -> Result<(), Error> {
//!     ensure!(value > 0, "Value must be greater than zero (found {})", value);
//!
//!     if value % 2 == 1 {
//!         bail!("Only even numbers can be used");
//!     }
//!
//!     Ok(())
//! }
//!
//! fn main() -> Result<(), Terminator> {
//!     let value = from_file().context("Unable to get value from file")?;
//!     validate(value).context("Value is not acceptable")?;
//!
//!     println!("Value = {}", value);
//!     Ok(())
//! }
//! ```

// Just bunches of Clippy lints.
#![deny(clippy::all)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::use_self)] // I rather like the name repetition
#![allow(clippy::missing_errors_doc)] // This is an error handling library, errors are implied.

use std::{
	error,
	fmt::{self, Display, Formatter},
	string::ToString,
};

mod macros;
mod terminator;
pub use terminator::Terminator;

pub type Result<T> = std::result::Result<T, Error>;

/// An error that is a human-targetted string plus an optional cause.
#[derive(Debug)]
pub struct Error
{
	/// The human-targetting error string.
	pub ctx: String,

	/// The optional cause of the error.
	pub cause: Option<Box<dyn error::Error + Send + 'static>>,
}

impl Error
{
	/// Create a new error with the given cause.
	#[allow(clippy::needless_pass_by_value)] // `T: ToString` implies `&T: ToString`
	pub fn new<S, E>(ctx: S, cause: E) -> Error
	where
		S: ToString,
		E: error::Error + Send + 'static,
	{
		let ctx = ctx.to_string();
		let cause: Option<Box<dyn error::Error + Send + 'static>> = Some(Box::new(cause));

		Error { ctx, cause }
	}

	/// Iterates over the causes of the error.
	pub fn iter_causes(&self) -> Causes { iter_causes(self) }
}

impl Display for Error
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "{}", self.ctx) }
}

impl error::Error for Error
{
	fn description(&self) -> &str { &self.ctx }

	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		self.cause.as_ref().map(|c| &**c as _)
	}
}

/// An iterator over the causes of an error.
// Add the `must_use` tag to please Clippy. I really doubt there will ever be a situation where
// someone creates a `Causes` iterator and doesn't consume it but we might as well warn them.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Causes<'a>
{
	/// The next cause to display.
	cause: Option<&'a (dyn error::Error + 'static)>,
}

impl<'a> Iterator for Causes<'a>
{
	type Item = &'a (dyn error::Error + 'static);

	fn next(&mut self) -> Option<Self::Item>
	{
		let cause = self.cause.take();
		self.cause = cause.and_then(error::Error::source);

		cause
	}
}

/// Extension methods to the `Result` type.
pub trait ResultExt<T>
{
	/// Adds some context to the error.
	fn context<S: ToString>(self, ctx: S) -> Result<T>;

	/// Adds context to the error, evaluating the context function only if there
	/// is an `Err`.
	fn with_context<S: ToString, F: FnOnce() -> S>(self, ctx_fn: F) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
	E: error::Error + Send + 'static,
{
	fn context<S: ToString>(self, ctx: S) -> Result<T>
	{
		self.map_err(|e| Error { ctx: ctx.to_string(), cause: Some(Box::new(e)) })
	}

	fn with_context<S: ToString, F: FnOnce() -> S>(self, ctx_fn: F) -> Result<T>
	{
		self.map_err(|e| Error { ctx: ctx_fn().to_string(), cause: Some(Box::new(e)) })
	}
}

/// Creates an error message from the provided string.
#[inline]
#[allow(clippy::needless_pass_by_value)] // `T: ToString` implies `&T: ToString`
pub fn err_msg<S: ToString>(ctx: S) -> Error { Error { ctx: ctx.to_string(), cause: None } }

/// Returns an iterator over the causes of an error.
#[inline]
pub fn iter_causes<E: error::Error + ?Sized>(e: &E) -> Causes { Causes { cause: e.source() } }
