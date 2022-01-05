use std::error::Error;

use blog_post::BlogPost;
use clap::Parser;
use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Editor, Input};

use crate::blog_post::ListItem;

mod blog_post;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Write a List Post
    /// List posts are best for non-chronological information. In other words, anything that doesn’t need to be in a specific order.
    #[clap(short, long)]
    list_post: bool,
    ///Print a number of list post examples
    #[clap(short, long)]
    show_list_post_examples: bool,
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
        let list_item_title: String = Input::with_theme(&theme)
            .with_prompt("What's the title of this list item? Make them benefit-focused")
            .interact()?;

        let mut list_item_content = None;
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add a few bullet points?")
            .interact()
            .unwrap()
        {
            if let Some(content) = Editor::new().edit("Add a few bullet points").unwrap() {
                list_item_content = Some(content);
            }
        }
        list_items.push(ListItem::new(&list_item_title, list_item_content));

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

    let should_number_list_items = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to number your list items?")
        .interact()
        .unwrap();

    Ok(Some(BlogPost::new(
        title,
        list_items,
        final_thoughts,
        should_number_list_items,
    )))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.show_list_post_examples {
        let list_post_example_headlines = vec![
            "XX Ways to [Desired Outcome]",
            "XX [Topic] Tips",
            "XX [Type] Tools",
            "XX Reasons Why [Problem]",
            "XX [Topic] Techniques",
            "XX [Products] For [Audience]",
            "\n",
            "Specific examples:",
            "\n",
            "17 Blogging Tips For Beginners (That Actually Work)",
            "13 Proven Tactics to Increase Your Blog Traffic",
            "86 Blog Post Ideas (With Successful Examples)",
            "29 Awesome SEO Blogs to Follow (Graded and Ranked)",
            "10 Google Ranking Factors You Shouldn't Ignore",
        ];

        for example in list_post_example_headlines {
            println!("{}", example);
        }
        println!("\nSource: https://ahrefs.com/blog/blog-post-templates/");
    } else if args.list_post {
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
