use unidecode::unidecode;
use std::fmt;
use std::path::PathBuf;
use config::Config;
use fail::Fail;

type Title = String;
type Content = String;

/// Represents a single page or post.
#[derive(Getters)]
pub struct Document {
    /// The title of the page or post, as given by the user.
    #[get = "pub"]
    title: Title,

    /// The filename-converted title of the page or post.
    #[get = "pub"]
    name: Name,

    /// The source location of the file.
    #[get = "pub"]
    src: PathBuf,

    /// The destination location of the file.
    #[get = "pub"]
    dest: PathBuf,

    /// The contents of the file.
    #[get = "pub"]
    content: Content,
}

impl Document {
    /// Creates a new page.
    pub fn page(config: Config, title: &str) -> Self {
        let title = title.to_string();

        let name = Name::from_title(&title);

        let mut src = PathBuf::new();
        src.push(config.pages_dir());
        src.push(name.to_string());
        src.set_extension("md");

        let mut dest = PathBuf::new();
        dest.push(config.build_dir());
        dest.push(name.to_string());
        dest.push("index");
        dest.set_extension("html");

        let mut content = String::new();
        content.push_str("# ");
        content.push_str(&title);
        content.push_str("\n");

        Document {
            title,
            name,
            src,
            dest,
            content,
        }
    }

    /// Creates a new post.
    pub fn post(_config: Config, _title: &str) -> Self {
        unimplemented!();
    }

    pub fn create(&self) -> Result<(), Fail> {
        unimplemented!();
    }
}

/// A representation of the name of the post or page.
#[derive(Debug)]
pub struct Name {
    s: String,
}

impl Name {
    /// Create a new name from a given title by cleaning it up.
    pub fn from_title(s: &str) -> Self {
        Name {
            s: cleanup(s.to_string())
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.s)
    }
}

/// Convert a post or page name into a string which will be used to create
/// the file.
fn cleanup(s: String) -> String {
    unidecode(&s)
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .to_lowercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_cleanup_name() {
        let expected_result = "hello-world".to_string();

        let n = Name::from_title("Hello, World!");
        let actual_result = n.to_string();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn more_complex_cleanup_name() {
        let expected_result = "setting-expectations-for-rusts-difficulty".to_string();

        let n = Name::from_title("Setting Expectations for Rust's Difficulty");
        let actual_result = n.to_string();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn unicode_cleanup_name() {
        let expected_result = "the-aeneid".to_string();

        let n = Name::from_title("The Æneid");
        let actual_result = n.to_string();

        assert_eq!(expected_result, actual_result);
    }
}
