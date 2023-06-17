/// Returns the value at the top of the stack.
pub const OP_RETURN: u8 = 0x00;

/// Pushes a constant onto the stack.
pub const OP_CONSTANT: u8 = 0x01;

/// Pops the top two values on the stack, adds them, and pushes the result.
pub const OP_ADD: u8 = 0x02;

/// Pops the top two values on the stack, subtracts the first from the second,
/// and pushes the result.
pub const OP_SUB: u8 = 0x03;

/// Pops the top two values on the stack, multiplies them, and pushes the
/// result.
pub const OP_MUL: u8 = 0x04;

/// Pops the top two values on the stack, divides the second by the first, and
/// pushes the result.
pub const OP_DIV: u8 = 0x05;
