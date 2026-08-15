use super::Component;

pub struct Tags {
    pub values: Vec<String>,
}

impl Tags {
    pub fn new<const N: usize>(names: [&str; N]) -> Self {
        Self {
            values: names.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn add(&mut self, tag: &str) -> bool {
        if self.contains(tag) {
            return false;
        }

        self.values.push(tag.to_string());
        true
    }

    pub fn remove(&mut self, tag: &str) -> bool {
        let Some(index) = self.values.iter().position(|value| value == tag) else {
            return false;
        };

        self.values.remove(index);
        true
    }

    pub fn contains(&self, tag: &str) -> bool {
        self.values.iter().any(|value| value == tag)
    }
}

impl Component for Tags {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_inserts_tag_once() {
        let mut tags = Tags::new(["Object"]);

        assert!(tags.add("Primitive"));
        assert!(!tags.add("Primitive"));
        assert_eq!(tags.values, vec!["Object", "Primitive"]);
    }

    #[test]
    fn remove_deletes_existing_tag() {
        let mut tags = Tags::new(["Object", "Primitive"]);

        assert!(tags.remove("Object"));
        assert!(!tags.remove("Object"));
        assert_eq!(tags.values, vec!["Primitive"]);
    }
}
