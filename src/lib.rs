use std::fmt::{format, Display};

#[derive(Debug)]
struct Parameter {
    parameter: String,
    value: String,
}

impl Parameter {
    pub fn new(parameter: String, value: String) -> Self {
        Parameter {
            parameter,
            value: urlencoding::encode(&value).to_string(),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.parameter, self.value)
    }
}

pub struct ObsidianUri {
    action: String,
    params: Vec<Parameter>,
}

impl ObsidianUri {
    pub fn action(action: &str) -> Self {
        ObsidianUri {
            action: action.to_string(),
            params: Vec::new(),
        }
    }

    pub fn add_parameter(mut self, param: String, value: String) -> Self {
        let p = Parameter::new(param, value);
        self.params.push(p);

        self
    }

    pub fn build(self) -> String {
        let mut uri = format!("obsidian://{}?", &self.action);

        let mut is_first = true;
        for param in self.params {
            if !is_first {
                uri.push_str("&");
            }
            uri.push_str(&format!("{}", param));
            is_first = false;
        }

        uri
    }

    pub fn open(self) {
        let uri = self.build();
        println!("{}", uri);
        open::that(uri).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obsidian_buffer() {
        let uri = ObsidianUri::action("open")
            .add_parameter("file".to_string(), "test".to_string())
            .build();

        assert_eq!(uri, "obsidian://open?file=test".to_string());
    }

    #[test]
    fn obsidian_multiple_parameters() {
        let uri = ObsidianUri::action("open")
            .add_parameter("file".to_string(), "test".to_string())
            .add_parameter("line".to_string(), "10".to_string())
            .build();

        assert_eq!(uri, "obsidian://open?file=test&line=10".to_string());
    }

    #[test]
    fn obsidian_open() {
        let uri = ObsidianUri::action("open")
            .add_parameter("file".to_string(), "test".to_string())
            .add_parameter("line".to_string(), "10".to_string())
            .build();

        assert_eq!(uri, "obsidian://open?file=test&line=10".to_string());
    }

    #[test]
    fn obsidian_new() {
        let uri = ObsidianUri::action("new")
            .add_parameter("file".to_string(), "test".to_string())
            .build();

        assert_eq!(uri, "obsidian://new?file=test".to_string());
    }

    #[test]
    fn obsidian_search() {
        let uri = ObsidianUri::action("search")
            .add_parameter("query".to_string(), "test".to_string())
            .build();

        assert_eq!(uri, "obsidian://search?query=test".to_string());
    }

    #[test]
    fn obsidian_spaced() {
        let uri = ObsidianUri::action("search")
            .add_parameter("query".to_string(), "test test".to_string())
            .build();

        assert_eq!(uri, "obsidian://search?query=test%20test".to_string());
    }
}
