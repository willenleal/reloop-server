use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long)]
    pub tmdb_key: String,
}
