use super::Component;

pub struct Name {
    pub value: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        Self {
            value: name.to_string(),
        }
    }
}

impl Component for Name {}
