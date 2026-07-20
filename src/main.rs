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
    let mut config_name: String = String::from("config");
    if let Some(name) = cli.config {
        config_name = name;
    }
    let mut cfg: cli::SSCConfig = confy::load("secure-sport-cola", config_name.as_str())?;

    let username = cli.username.unwrap_or(cfg.username.clone());
    let room = cli.room.unwrap_or(cfg.room);
    if cfg.username.is_empty() {
        if username.is_empty() {
            eprintln!(
                "Username must be provided either through the --username argument or the config file."
            );
            std::process::exit(1);
        }

        cfg.username = username.clone();
        confy::store("secure-sport-cola", config_name.as_str(), &cfg)?;
    }
    if cfg.url.is_empty() {
        eprintln!("URL is not defined in the config file.");
        std::process::exit(1);
    }
    let member_id = match get_member_id(&cfg.url, &username).await {
        Ok(mem_id) => mem_id,
        Err(err) => {
            if err.to_string() == "error decoding response body" {
                eprintln!(
                    "You do not have access to the stregsystem, try changing networks or starting a VPN."
                );
                std::process::exit(1);
            } else if err
                .to_string()
                .starts_with("error sending request for url (http")
            {
                eprintln!("Couldn't send reqest. Possibly missing internet connection.");
                std::process::exit(1);
            }
            panic!("{}", err);
        }
    };

    if cli.balance {
        let balance = match get_member_balance(&cfg.url, &member_id).await {
            Ok(bal) => bal,
            Err(err) => panic!("{}", err),
        };
        println!("Balance: {}", (balance as f32) / 100.0);
        return Ok(());
    }
    if cli.list {
        let products = match get_active_products(&cfg.url, room).await {
            Ok(products) => products,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        let named_products = match get_named_products(&cfg.url).await {
            Ok(named_products) => named_products,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };
        println!("Active products:");
        for (id, product) in products {
            let short_id: i32 = id.parse().expect("Couldn't convert to integer");
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
            let mut short: &String = &String::from("");
            if !shorts.is_empty() {
                short = &shorts[rand::rng().random_range(0..shorts.len())];
            }
            // println!("{}", short);
            println!(
                "{:4} {:7} {:11} | {}",
                id,
                format!("({})", (product.price as f32) / 100.0),
                short,
                product.name
            );
        }
        return Ok(());
    }

    if cli.buystring.is_empty() {
        eprintln!("No buy string given, doing nothing.");
        return Ok(());
    }

    let sale_req = SaleRequest {
        member_id,
        room,
        buystring: format!("{} {}", &username, cli.buystring.join(" ")),
    };

    println!("{:?}", sale_req);

    match post_sale(&cfg.url, sale_req).await {
        Ok(_) => {}
        Err(err) => {
            println!(
                "Error happend during sale, please check with the official stregsystem wether you have bought your items. \n Error: {}",
                err
            );
        }
    }
    Ok(())
}
