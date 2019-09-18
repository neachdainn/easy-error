//! Types that are useful in combination with the `Termination` trait.
use std::{error::Error as StdError, fmt};

/// An error that wraps all other error types for a nicer debug output.
///
/// Given the current implementation of the `Termination` trait, and the
/// implementation for every type that implements `Debug`, having a `main`
/// function that returns a `Result` requires either using a type that
/// implements the `Debug` trait poorly or dealing with an output that isn't
/// very user friendly.
///
/// The types here help alleviate those issues. To begin with, we have an
/// `Error` type that simply wraps any possible error and implements `Debug` in
/// such a way as to make the output look nice. Additionally, there is a
/// `Result` specialization in order to make the `main` function a little
/// cleaner.
pub struct Terminator
{
	inner: Box<dyn StdError + 'static>,
}

impl fmt::Debug for Terminator
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		writeln!(f, "{}", self.inner)?;
		for cause in super::iter_causes(self.inner.as_ref()) {
			writeln!(f, "Caused by: {}", cause)?;
		}

		Ok(())
	}
}

impl<E: StdError + 'static> From<E> for Terminator
{
	fn from(err: E) -> Terminator { Terminator { inner: Box::new(err) } }
}
