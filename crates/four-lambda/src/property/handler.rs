use nutype::nutype;

#[nutype(
    validate(len_char_max = 100, regex = r#"^[^\s]+$"#,),
    derive(Debug, Clone, Serialize)
)]
pub struct Handler(String);
