use std::{error::Error, net::IpAddr};

use clap::Parser;
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input, Select};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Write a List Post
    #[clap(short, long)]
    list_post: bool,
}

#[derive(Debug)]
struct Intro {
    content: String,
}

impl Default for Intro {
    fn default() -> Self {
        Self {
            content: r#"
            **Feeling overwhelmed by the infinite options for driving traffic to your website? You're not alone**

            -Keep it short
            - Try to establish trust in as few words as possible
            - Add a table of content
            "#.into()
        }
    }
}

#[derive(Debug)]
struct BlogPost {
    title: String,
    intro: Intro,
    list_items: Vec<String>,
    final_thoughts: String,
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

fn list_post_wizard() -> Result<Option<BlogPost>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("New List Post");

    let title = Input::with_theme(&theme)
        .with_prompt("What's the title? (Mention the desired outcome, e.g. 'More traffic|better visibility')")
        .default("Using Rust to your advantage".parse().unwrap())
        .interact()?;

    println!("We'll now collect some list items");
    let mut list_items = vec![];

    loop {
        let list_item = Input::with_theme(&theme)
            .with_prompt("What's the title of this list item? Make them benefit-focused")
            .interact()?;
        list_items.push(list_item);
        if !Confirm::with_theme(&theme)
            .with_prompt("Do you want to keep adding items?")
            .interact()?
        {
            break;
        }
    }

    let final_thoughts = Input::with_theme(&theme)
        .with_prompt("Final Thoughts - Provide a few bullet points on final thoughts.")
        .default("One or two final short tips".parse().unwrap())
        .interact()?;

    Ok(Some(BlogPost {
        title,
        intro: Intro::default(),
        list_items,
        final_thoughts: final_thoughts,
    }))
}

fn main() {
    let args = Args::parse();

    if args.list_post {
        match list_post_wizard() {
            Ok(None) => println!("Aborted."),
            Ok(Some(config)) => println!("{:#?}", config),
            Err(err) => println!("error: {}", err),
        }
    }
}
