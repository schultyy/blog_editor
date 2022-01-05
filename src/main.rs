use std::error::Error;

use blog_post::BlogPost;
use clap::Parser;
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input};

mod blog_post;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Write a List Post
    #[clap(short, long)]
    list_post: bool,
}

fn list_post_wizard() -> Result<Option<BlogPost>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    println!("New List Post");

    let mut title = Input::with_theme(&theme)
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

    if Confirm::with_theme(&theme)
        .with_prompt(format!(
            "Include Number in Post title? >> \"{} {}\"",
            list_items.len(),
            title
        ))
        .interact()?
    {
        title = format!("{} {}", list_items.len(), title);
    }

    let final_thoughts = Input::with_theme(&theme)
        .with_prompt("Final Thoughts - Provide a few bullet points on final thoughts.")
        .default("One or two final short tips".parse().unwrap())
        .interact()?;

    Ok(Some(BlogPost::new(title, list_items, final_thoughts)))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.list_post {
        let blog_post = match list_post_wizard() {
            Ok(None) => {
                eprintln!("Aborted.");
                std::process::exit(1)
            }
            Ok(Some(post)) => post,
            Err(err) => {
                eprintln!("error: {}", err);
                std::process::exit(1)
            }
        };

        let filename: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("What's the filename of the new blog post?")
            .default("post.md".to_string())
            .interact_text()
            .unwrap();
        blog_post.save(filename)?;
    }

    Ok(())
}
