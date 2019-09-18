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
//! use easy_error::{bail, ensure, Result, ResultExt, termination};
//!
//! fn from_file() -> Result<i32> {
//!     let file_name = "example.txt";
//!     let mut file = File::open(file_name).context("Could not open file")?;
//!
//!     let mut contents = String::new();
//!     file.read_to_string(&mut contents).context("Unable to read file")?;
//!
//!     contents.trim().parse().context("Could not parse file")
//! }
//!
//! fn validate(value: i32) -> Result<()> {
//!     ensure!(value > 0, "Value must be greater than zero (found {})", value);
//!
//!     if value % 2 == 1 {
//!         bail!("Only even numbers can be used");
//!     }
//!
//!     Ok(())
//! }
//!
//! fn main() -> termination::Result {
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
#![allow(clippy::use_self)]

use std::{error::Error as StdError, fmt, result::Result as StdResult};

mod macros;
mod terminator;
pub use terminator::Terminator;

pub type Result<T> = StdResult<T, Error>;

/// An error that is a human-targetted string plus an optional cause.
#[derive(Debug)]
pub struct Error
{
	/// The human-targetting error string.
	pub ctx: String,

	/// The optional cause of the error.
	pub cause: Option<Box<dyn StdError + Send + 'static>>,
}

impl Error
{
	/// Create a new error with the given cause.
	pub fn new<S, E>(ctx: S, cause: E) -> Error
	where
		S: Into<String>,
		E: StdError + Send + 'static,
	{
		let ctx = ctx.into();
		let cause: Option<Box<dyn StdError + Send + 'static>> = Some(Box::new(cause));

		Error { ctx, cause }
	}

	/// Iterates over the causes of the error.
	pub fn iter_causes(&self) -> Causes { iter_causes(self) }
}

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.ctx) }
}

impl StdError for Error
{
	fn description(&self) -> &str { &self.ctx }

	fn source(&self) -> Option<&(dyn StdError + 'static)> { self.cause.as_ref().map(|c| &**c as _) }
}

/// An iterator over the causes of an error.
pub struct Causes<'a>
{
	/// The next cause to display.
	cause: Option<&'a (dyn StdError + 'static)>,
}

impl<'a> Iterator for Causes<'a>
{
	type Item = &'a (dyn StdError + 'static);

	fn next(&mut self) -> Option<Self::Item>
	{
		let cause = self.cause.take();
		self.cause = cause.and_then(StdError::source);

		cause
	}
}

/// Extention methods to the `Result` type.
pub trait ResultExt<T>
{
	/// Adds some context to the error.
	fn context<S: Into<String>>(self, ctx: S) -> Result<T>;
}

impl<T, E> ResultExt<T> for StdResult<T, E>
where
	E: StdError + Send + 'static,
{
	fn context<S: Into<String>>(self, ctx: S) -> Result<T>
	{
		self.map_err(|e| Error { ctx: ctx.into(), cause: Some(Box::new(e)) })
	}
}

/// Creates an error message from the provided string.
#[inline]
pub fn err_msg<S: Into<String>>(ctx: S) -> Error { Error { ctx: ctx.into(), cause: None } }

/// Returns an iterator over the causes of an error.
#[inline]
pub fn iter_causes<E: StdError + ?Sized>(e: &E) -> Causes { Causes { cause: e.source() } }
