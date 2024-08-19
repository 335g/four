use nutype::nutype;

#[nutype(
    validate(
        not_empty,
        len_char_max = 512,
        regex = r#"((/[A-Za-z0-9\.,\+@=_-]+)*)/"#
    ),
    derive(Debug, Clone, Serialize)
)]
pub struct Path(String);
