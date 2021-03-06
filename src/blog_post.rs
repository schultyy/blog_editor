use std::fs::File;
use std::io::Write;


#[derive(Debug, Clone)]
struct Intro {
    content: String,
}

impl Default for Intro {
    fn default() -> Self {
        Self {
            content: r#"

**Feeling overwhelmed by the infinite options for driving traffic to your website? You're not alone**
Change this.

Intro Tips:

- Keep it short
- Try to establish trust in as few words as possible
- Add a table of content

            "#.into()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ListItem {
    title: String,
    content: Option<String>
}

impl ListItem {
    pub fn new(title: &str, content: Option<String>) -> Self {
        Self {
            title: title.to_string(),
            content
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlogPost {
    title: String,
    intro: Intro,
    list_items: Vec<ListItem>,
    final_thoughts: String,
    should_number_list_items: bool
}

impl BlogPost {
    pub fn new(title: String, list_items: Vec<ListItem>, final_thoughts: String, should_number_list_items: bool) -> Self {
        Self {
            title,
            list_items,
            final_thoughts,
            should_number_list_items,
            intro: Intro::default()
        }
    }

    pub(crate) fn save(&self, filename: String) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;
        writeln!(file, "# {}", self.title)?;
        writeln!(file, "{}", self.intro.content)?;
        writeln!(file, "**TOC PLACEHOLDER**\n")?;
        for (index, list_item) in self.list_items.clone().into_iter().enumerate() {
            writeln!(file, "{}", self.create_headline(index, list_item.title))?;
            if let Some(content) = list_item.content {
                writeln!(file, "{}\n", content)?;
            }
        }
        writeln!(file, "\n")?;
        writeln!(file, "## Final Thoughts")?;
        writeln!(file, "{}", self.final_thoughts)?;
        Ok(())
    }

    fn create_headline(&self, index: usize, headline: String) -> String {
        if self.should_number_list_items {
            format!("## {}. {}\n", index + 1, headline)
        }
        else {
            format!("## {}\n", headline)
        }
    }
}

impl Default for BlogPost {
    fn default() -> Self {
        Self {
            title: "11 Proven Ways to do something awesome".into(),
            intro: Intro::default(),
            list_items: vec![],
            should_number_list_items: false,
            final_thoughts: String::default(),
        }
    }
}