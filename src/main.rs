use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Write a List Post
    #[clap(short, long)]
    list_post: bool,
}

fn main() {
    let args = Args::parse();

    if args.list_post {
        println!("YAY A list Post");
    }
}
