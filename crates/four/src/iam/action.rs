#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Sts(StsAction),
}

trait ActionName {
    fn name(&self) -> String;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StsAction {
    AssumeRole,
}

impl ActionName for StsAction {
    fn name(&self) -> String {
        match self {
            StsAction::AssumeRole => String::from("sts::AssumeRole"),
        }
    }
}
