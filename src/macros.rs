/// Exits a function early with an `Error`.
#[macro_export]
macro_rules! bail
{
	($($arg:tt)*) => {
		return Err($crate::format_err!($($arg)*)).into();
	};
}

/// Exits a function early with an `Error` if the condition is not satisfied.
#[macro_export]
macro_rules! ensure
{
	($cond:expr, $($arg:tt)*) => {
		if !($cond) {
			return Err($crate::format_err!($($arg)*).into());
		}
	};
}

/// Creates an `Error` using the standard string interpolation syntax.
#[macro_export]
macro_rules! format_err
{
	($($arg:tt)*) => { $crate::err_msg(format_args!($($arg)*)) };
}
