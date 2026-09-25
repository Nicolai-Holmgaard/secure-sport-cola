use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SSCConfig {
    pub username: String,
    pub room: i32,
    pub url: String,
}

impl ::std::default::Default for SSCConfig {
    fn default() -> Self {
        Self {
            username: "".into(),
            room: 10,
            url: "http://stregsystem.fklub.dk".into(),
        }
    }
}

#[derive(Parser)]
#[command(version, about="CLI to secure a Sport-cola", long_about = None)]
pub struct CliOptions {
    /// The username to use for the purchase. If not provided, it will be read from the config file.
    #[arg(short, long)]
    pub username: Option<String>,
    /// Change the configuration file, usefull for using multiple stregsystems.
    #[arg(short, long)]
    pub config: Option<String>,
    /// To change the room to interact with
    #[arg(short, long)]
    pub room: Option<i32>,
    /// The buy string, it works the same as the buy string in the stregsystem, but without the
    /// username, since that is provided by the --username argument or the config file.
    pub buystring: Vec<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get 10 lastest purchases
    History { username: Option<String> },
    /// List the active products in the room.
    List { room_in: Option<i32> },
    /// Print the member's balance
    Balance { username: Option<String> },
}
