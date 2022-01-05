use std::fs::File;
use std::io::Write;


#[derive(Debug)]
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

#[derive(Debug)]
pub struct BlogPost {
    title: String,
    intro: Intro,
    list_items: Vec<String>,
    final_thoughts: String,
}

impl BlogPost {
    pub fn new(title: String, list_items: Vec<String>, final_thoughts: String) -> Self {
        Self {
            title,
            list_items,
            final_thoughts,
            intro: Intro::default()
        }
    }

    pub(crate) fn save(&self, filename: String) -> Result<(), std::io::Error> {
        let mut file = File::create(filename)?;
        writeln!(file, "# {}", self.title)?;
        writeln!(file, "{}", self.intro.content)?;
        writeln!(file, "TOC")?;
        for list_item in &self.list_items {
            writeln!(file, "## {}", list_item)?;
        }
        writeln!(file, "\n")?;
        writeln!(file, "## Final Thoughts")?;
        writeln!(file, "{}", self.final_thoughts)?;
        Ok(())
    }
}

impl Default for BlogPost {
    fn default() -> Self {
        Self {
            title: "11 Proven Ways to do something awesome".into(),
            intro: Intro::default(),
            list_items: vec![],
            final_thoughts: String::default(),
        }
    }
}