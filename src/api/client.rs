use crate::api::endpoints;
use crate::api::types::{
    History, MemberBalance, MemberId, MemberInfo, Product, SaleRequest, SaleResponse,
};
use colored::Colorize;
use std::collections::HashMap;
use std::process::exit;

pub async fn get_member_id(
    api_url: &str,
    username: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    let url = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_ID_ENDPOINT.replace("{username}", username)
    );
    let resp = reqwest::get(url).await?;

    if resp.status() == 400 {
        println!("There is no account with the name of: {}", username);
        exit(1);
    }

    let json = resp.json::<MemberId>().await?;

    Ok(json.member_id)
}

pub async fn get_member_balance(
    api_url: &str,
    member_id: &i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_BALANCE_ENDPOINT
            .replace("{member_id}", member_id.to_string().as_str())
    );
    let resp = reqwest::get(url).await?.json::<MemberBalance>().await?;
    Ok(resp.balance)
}

#[allow(dead_code)]
pub async fn get_member_info(
    api_url: &str,
    member_id: &i32,
) -> Result<MemberInfo, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_INFO_ENDPOINT.replace("{member_id}", member_id.to_string().as_str())
    );
    let resp = reqwest::get(url).await?.json::<MemberInfo>().await?;
    Ok(resp)
}

pub async fn get_member_history(
    api_url: &str,
    member_id: &i32,
) -> Result<History, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_MEMBER_HISTORY_ENDPOINT
            .replace("{member_id}", member_id.to_string().as_str())
    );
    let resp = reqwest::get(url).await?.json::<History>().await?;
    Ok(resp)
}

#[allow(dead_code)]
pub async fn get_named_products(
    api_url: &str,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let url: String = format!("{}{}", api_url, endpoints::GET_NAMED_PRODUCTS_ENDPOINT);
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, i32>>()
        .await?;
    Ok(resp)
}

pub async fn get_active_products(
    api_url: &str,
    room_id: i32,
) -> Result<HashMap<String, Product>, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}{}",
        api_url,
        endpoints::GET_ACTIVE_PRODUCTS_ENDPOINT.replace("{room_id}", room_id.to_string().as_str())
    );
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, Product>>()
        .await?;
    Ok(resp)
}

pub async fn post_sale(
    api_url: &str,
    sale_request: SaleRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let url: String = format!("{}{}", api_url, endpoints::SALES_ENDPOINT);
    let client = reqwest::Client::new();
    let resp = client.post(url).json(&sale_request).send().await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            println!("{}", "Sale posted successfully.".green());
        }
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
            eprintln!(
                "Internal server error while posting sale. Possibly due to invalid buystring or other issues."
            );
            return Err("Internal server error".into());
        }
        _ => {
            let text = resp.text().await?;
            eprintln!("Error posting sale: {}", text);
            return Err(text.into());
        }
    }

    let json_resp = resp.json::<SaleResponse>().await?;

    match json_resp.status {
        200 => {
            if let Some(values) = json_resp.values {
                if values.is_ballmer_peaking {
                    if let (Some(minutes), Some(seconds)) = (values.bp_minutes, values.bp_seconds) {
                        println!(
                            "Ballmer Peaking for {} minutes and {} seconds",
                            minutes, seconds
                        );
                    } else {
                        println!("Ballmer Peaking, but no time provided.");
                    }
                }
                if values.caffeine > 0.0 {
                    println!("Caffeine: {}mg", values.caffeine);
                }
                if values.promille > 0.0 {
                    println!("Promille: {}‰", values.promille);
                }
                println!("Cost: {}", values.cost as f32 / 100.0);
                println!("Member balance: {}", values.member_balance);
                if values.member_has_low_balance {
                    println!("{}", "Warning: Low balance".red().on_yellow());
                }
            }
        }
        500 => {
            eprintln!("Internal server error while posting sale.");
            return Err("Internal server error".into());
        }
        403 => {
            eprintln!(
                "Forbidden: You do not have permission to post this sale. Possibly due to insufficient balance."
            );
            return Err("Forbidden".into());
        }
        _ => {
            eprintln!("Error posting sale: {}", json_resp.msg);
            return Err(json_resp.msg.into());
        }
    }
    // TODO: Handle response, check for errors, etc.

    Ok(())
}
