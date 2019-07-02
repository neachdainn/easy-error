/// Exits a function early with an `Error`.
#[macro_export]
macro_rules! bail
{
	($ctx:expr) => {
		return Err($crate::err_msg($ctx).into());
	};

	($fmt:expr, $($arg:tt)*) => {
		return Err($crate::err_msg(format!($fmt, $($arg)*)).into());
	};
}

/// Exits a function early with an `Error` if the condition is not satisfied.
#[macro_export]
macro_rules! ensure
{
	($cond:expr, $ctx:expr) => {
		if !($cond) {
			return Err($crate::err_msg($ctx).into());
		}
	};

	($cond:expr, $fmt:expr, $($arg:tt)*) => {
		if !($cond) {
			return Err($crate::err_msg(format!($fmt, $($arg)*)).into());
		}
	};
}

/// Creates an `Error` using the standard string interpolation syntax.
#[macro_export]
macro_rules! format_err
{
	($($arg:tt)*) => { $crate::err_msg(format!($($arg)*)) };
}
