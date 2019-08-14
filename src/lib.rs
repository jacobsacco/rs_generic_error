
use std::error;
use std::fmt;
use std::convert;


pub struct GenericError {
	pub msg: String,
}


pub type Result<T> = std::result::Result<T, GenericError>;


// GenErr!("this went wrong: {}", "thing")  ->  Err(GenericError("this went wrong: thing"))
#[macro_export]
macro_rules! GenErr {
	($arg:expr) => {
		Err(
			GenericError {
				msg: String::from($arg),
			}
		)
	};
	($($args:expr),+) => {
		Err(
			GenericError {
				msg: format!($($args),+),
			}
		)
	};
}


impl fmt::Display for GenericError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self.msg)
	}
}


impl fmt::Debug for GenericError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self.msg)
	}
}


impl<T> convert::From<T> for GenericError where T: error::Error {
	fn from(error: T) -> Self {
		let desc = format!("{:?}", error);
		GenericError {
			msg: desc,
		}
	}
}

