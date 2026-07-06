use crate::api::client::{
    get_active_products, get_member_balance, get_member_id, get_named_products, post_sale,
};
use crate::api::types::SaleRequest;
use clap::Parser;
use rand::RngExt;
mod api;
mod cli;

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let cli = cli::CliOptions::parse();
    let mut cfg: cli::SSCConfig = confy::load("secure-sport-cola", "config")?;

    let username = cli.username.unwrap_or(cfg.username.clone());
    let room = cli.room.unwrap_or(cfg.room.clone());
    if cfg.username.is_empty() {
        if username.is_empty() {
            eprintln!(
                "Username must be provided either through the --username argument or the config file."
            );
            std::process::exit(1);
        }

        cfg.username = username.clone();
        confy::store("secure-sport-cola", "config", &cfg)?;
    }
    let member_id = get_member_id(&cfg.url, &username).await.unwrap();

    if cli.balance {
        let balance = get_member_balance(&cfg.url, &member_id).await.unwrap();
        println!("Balance: {}", balance);
        return Ok(());
    }
    if cli.list {
        let products = get_active_products(&cfg.url, room).await.unwrap();
        let named_products = get_named_products(&cfg.url).await.unwrap();
        println!("Active products:");
        for (id, product) in products {
            let short_id: i32 = id.parse().unwrap();
            let shorts: Vec<String> = named_products
                .clone()
                .into_iter()
                .filter_map(|short| {
                    if short.1 == short_id {
                        Some(short.0)
                    } else {
                        None
                    }
                })
                .collect();
            // let short = shorts[rand::rng().random_range(0..shorts.len())];
            // println!("{}", short);
            println!(
                "{:4} {:?} {:7} {}",
                id,
                shorts,
                format!("({})", product.price),
                product.name
            );
        }
        return Ok(());
    }

    let sale_req = SaleRequest {
        member_id,
        room: room,
        buystring: format!("{} {}", &username, cli.buystring.join(" ")),
    };

    println!("{:?}", sale_req);

    post_sale(&cfg.url, sale_req).await.unwrap();
    Ok(())
}
