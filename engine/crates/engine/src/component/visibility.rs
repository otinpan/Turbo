use super::Component;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Visibility {
    pub is_visible: bool,
}

impl Default for Visibility {
    fn default() -> Self {
        Self { is_visible: true }
    }
}

impl Component for Visibility {}
