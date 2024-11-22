use serde_json::json;
use rocket_sync_db_pools::database;
use rocket::routes;



#[database("personal_budget")]
struct BudgetDb(diesel::SqliteConnection);


// 데이터 베이스 스키마 작성
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Transaction {
    id: Option<i32>,
    date: String,
    amount: i32,
    category: String,
    description: Option<String>,
}


// Rocket 플레임워크 
// CRUD API

// CRUD 핸들러
#[rocket::post("/transaction", format = "json", data = "<transaction>")]
async fn create_transaction(transaction: rocket::serde::json::Json<Transaction>) -> &'static str {
    "Transaction added successfully!"
}

// 라우트 정의
#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create_transaction])
}