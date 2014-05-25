use std::fmt::{Formatter, Result, Show};
/// Represents an operator
pub trait Operator {
	/// Get the precedence as an unsignes integer, where the lower it is, the more precedence/priority it has
	fn get_precedence(&self) -> uint;
}
#[deriving(Clone, Eq)]
/// A numeric operation between 2 values
pub enum NumOp {
	/// `a + b` - Addition
	OpAdd,
	/// `a - b` - Subtraction
	OpSub,
	/// `a / b` - Division
	OpDiv,
	/// `a * b` - Multiplication
	OpMul,
	/// `a % b` - Modulus
	OpMod
}
impl Show for NumOp {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", match *self {
			OpAdd => "+",
			OpSub => "-",
			OpDiv => "/",
			OpMul => "*",
			OpMod => "%"
		})
	}
}
#[deriving(Clone, Eq)]
/// A unary operation on a single value
pub enum UnaryOp {
	/// `a++` - increment the value
	UnaryIncrement(bool),
	/// `a--` - decrement the value
	UnaryDecrement(bool),
	/// `-a` - negate the value
	UnaryMinus,
	/// `+a` - convert to a number
	UnaryPlus,
	/// `!a` - get the opposite of the boolean value
	UnaryNot
}
impl Show for UnaryOp {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", match *self {
			UnaryIncrement(_) => "++",
			UnaryDecrement(_) => "--",
			UnaryPlus => "+",
			UnaryMinus => "-",
			UnaryNot => "!"
		})
	}
}
#[deriving(Clone, Eq)]
/// A bitwise operation between 2 values
pub enum BitOp {
	/// `a & b` - Bitwise and
	BitAnd,
	/// `a | b` - Bitwise or
	BitOr,
	/// `a ^ b` - Bitwise xor
	BitXor,
	/// `a << b` - Bit-shift leftwards
	BitShl,
	/// `a >> b` - Bit-shift rightrights
	BitShr
}
impl Show for BitOp {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", match *self {
			BitAnd => "&",
			BitOr => "|",
			BitXor => "^",
			BitShl => "<<",
			BitShr => ">>"
		})
	}
}
#[deriving(Clone, Eq)]
/// A comparitive operation between 2 values
pub enum CompOp {
	/// `a == b` - Equality
	CompEqual,
	/// `a != b` - Unequality
	CompNotEqual,
	/// `a === b` - Strict equality
	CompStrictEqual,
	/// `a !== b` - Strict unequality
	CompStrictNotEqual,
	/// `a > b` - If `a` is greater than `b`
	CompGreaterThan,
	/// `a >= b` - If `a` is greater than or equal to `b`
	CompGreaterThanOrEqual,
	/// `a < b` - If `a` is less than `b`
	CompLessThan,
	/// `a <= b` - If `a` is less than or equal to `b`
	CompLessThanOrEqual,
}
impl Show for CompOp {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", match *self {
			CompEqual => "==",
			CompNotEqual => "!=",
			CompStrictEqual => "===",
			CompStrictNotEqual => "!==",
			CompGreaterThan => ">",
			CompGreaterThanOrEqual => ">=",
			CompLessThan => "<",
			CompLessThanOrEqual => "<="
		})
	}
}
#[deriving(Clone, Eq)]
/// A logical operation between 2 boolean values
pub enum LogOp {
	/// `a && b` - Logical and
	LogAnd,
	/// `a || b` - Logical or
	LogOr
}
impl Show for LogOp {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", match *self {
			LogAnd => "&&",
			LogOr => "||"
		})
	}
}
#[deriving(Clone, Eq)]
/// A binary operation between 2 values
pub enum BinOp {
	/// Numeric operation
	BinNum(NumOp),
	/// Bitwise operation
	BinBit(BitOp),
	/// Comparitive operation
	BinComp(CompOp),
	/// Logical operation
	BinLog(LogOp)
}
impl Operator for BinOp {
	fn get_precedence(&self) -> uint {
		match *self {
			BinNum(OpMul) | BinNum(OpDiv) | BinNum(OpMod) => 5,
			BinNum(OpAdd) | BinNum(OpSub) => 6,
			BinBit(BitShl) | BinBit(BitShr) => 7,
			BinComp(CompLessThan) | BinComp(CompLessThanOrEqual) | BinComp(CompGreaterThan) | BinComp(CompGreaterThanOrEqual) => 8,
			BinComp(CompEqual) | BinComp(CompNotEqual) | BinComp(CompStrictEqual) | BinComp(CompStrictNotEqual) => 9,
			BinBit(BitAnd) => 10,
			BinBit(BitXor) => 11,
			BinBit(BitOr) => 12,
			BinLog(LogAnd) => 13,
			BinLog(LogOr) => 14,
			
		}
	}
}
impl Show for BinOp {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", match *self {
			BinNum(op) => op.to_str(),
			BinBit(op) => op.to_str(),
			BinComp(op) => op.to_str(),
			BinLog(op) => op.to_str()
		})
	}
}