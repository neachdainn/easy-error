//! # Easy-error
//!
//! This crate is a lightweight error handling library meant to play well with the standard `Error` trait.
//! There are three major components of this crate:
//!
//! 1. A basic, string-based error type that is meant for either quick prototyping or human-facing errors.
//! 2. A nice way to iterate over the causes of an error.
//! 3. Some macros that make returning errors slightly more ergonomic.
//!
//! ## Rust Version Requirements
//!
//! The current version requires **Rustc 1.32 or newer**.
//! In general, this crate will be compilable with the Rustc version available on the oldest Ubuntu LTS release.
//! Any change that requires a new Rustc version will be considered a breaking change and will be handled accordingly.
//!
//! ## Example
//!
//! ```rust
//! use std::{fs::File, io::Read};
//! use easy_error::{Error, ResultExt};
//!
//! fn run(file: &str) -> Result<i32, Error> {
//!     let mut file = File::open(file).context("Could not open file")?;
//!
//!     let mut contents = String::new();
//!     file.read_to_string(&mut contents).context("Unable to read file")?;
//!
//!     let value = contents.trim().parse().context("Could not parse file")?;
//!     ensure!(value != 0, "Value cannot be zero");
//!
//!     Ok(value)
//! }
//!
//! fn main() {
//!     let file = "example.txt";
//!
//!     if let Err(e) = run(file) {
//!         eprintln!("Error: {}", e);
//!         e.iter_causes().for_each(|c| eprintln!("Caused by: {}", c));
//!     }
//! }
//! ```

// Just bunches of Clippy lints.
#![deny(clippy::all)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::use_self)]

use std::{error::Error as StdError, fmt, result::Result as StdResult};

mod macros;

pub type Result<T> = StdResult<T, Error>;

/// An error that is a human-targetted string plus an optional cause.
#[derive(Debug)]
pub struct Error
{
	/// The human-targetting error string.
	pub ctx: String,

	/// The optional cause of the error.
	pub cause: Option<Box<dyn StdError + 'static>>,
}

impl Error
{
	/// Create a new error with the given cause.
	pub fn new<S, E>(ctx: S, cause: E) -> Error
	where
		S: Into<String>,
		E: StdError + 'static
	{
		let ctx = ctx.into();
		let cause: Option<Box<dyn StdError + 'static>> = Some(Box::new(cause));

		Error { ctx, cause }
	}

	/// Iterates over the causes of the error.
	pub fn iter_causes(&self) -> Causes
	{
		Causes { cause: self.cause.as_ref().map(Box::as_ref) }
	}
}

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.ctx)
	}
}

impl StdError for Error
{
	fn description(&self) -> &str
	{
		&self.ctx
	}

	fn source(&self) -> Option<&(dyn StdError + 'static)>
	{
		self.cause.as_ref().map(Box::as_ref)
	}
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

impl<T, E: StdError + 'static> ResultExt<T> for StdResult<T, E>
{
	fn context<S: Into<String>>(self, ctx: S) -> Result<T>
	{
		self.map_err(|e| Error { ctx: ctx.into(), cause: Some(Box::new(e)) })
	}
}

/// Creates an error message from the provided string.
#[inline]
pub fn err_msg<S: Into<String>>(ctx: S) -> Error
{
	Error { ctx: ctx.into(), cause: None }
}

/// Returns an iterator over the causes of an error.
pub fn iter_causes<E: StdError>(e: &E) -> Causes
{
	Causes { cause: e.source() }
}
