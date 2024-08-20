use nutype::nutype;

#[nutype(validate(len_char_min = 1), derive(Debug, Clone, Serialize))]
pub struct FunctionName(String);

#[nutype(
    validate(greater_or_equal = 128, less_or_equal = 10240),
    derive(Debug, Clone, Serialize)
)]
pub struct MemorySize(usize);

#[nutype(
    validate(greater = 0, less_or_equal = 900),
    derive(Debug, Clone, Serialize)
)]
pub struct Timeout(usize);
