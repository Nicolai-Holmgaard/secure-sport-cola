use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberInfo {
    pub balance: i32,
    pub username: String,
    pub active: bool,
    pub name: String,
    pub signup_due_paid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberBalance {
    pub balance: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberId {
    pub member_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sale {
    pub timestamp: String,
    pub product: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub name: String,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleRequest {
    pub member_id: i32,
    pub buystring: String,
    pub room: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleResponse {
    pub status: i32,
    pub msg: String,
    pub values: Option<SaleResponseValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleResponseValues {
    pub order: SaleResponseOrder,
    pub promille: f32,
    pub is_ballmer_peaking: bool,
    pub bp_minutes: Option<i32>,
    pub bp_seconds: Option<i32>,
    pub caffeine: f32,
    pub cups: i32,
    pub product_contains_caffeine: bool,
    pub is_coffee_master: bool,
    pub cost: i32,
    pub give_multibuy_hint: bool,
    pub sale_hints: Option<String>,
    pub member_has_low_balance: bool,
    pub member_balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleResponseOrder {
    pub room: i32,
    pub member: i32,
    pub created_on: String,
    pub items: Vec<i32>,
}
