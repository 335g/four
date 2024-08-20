use nutype::nutype;

#[nutype(
    validate(greater_or_equal = 60, less_or_equal = 21600),
    derive(Debug, Clone, Serialize)
)]
pub struct MaximumEventAgeInSeconds(usize);

#[nutype(validate(less_or_equal = 2), derive(Debug, Clone, Serialize))]
pub struct MaximumRetryAttempts(usize);
