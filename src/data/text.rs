#[derive(Debug, PartialEq)]
pub struct Text{
    pub content: String,
}

impl From<String> for Text{
    fn from(item: String) -> Self {
        Text{ content: item }
    }
}
