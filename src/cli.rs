use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(short, long)]
    pub id: u64, 
    #[arg(short, long)]
    pub peers: String
}