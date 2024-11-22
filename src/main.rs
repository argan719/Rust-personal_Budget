use serde_json::json;
use rocket_sync_db_pools::database;
use rocket::routes;

#[macro_use]
extern crate diesel;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::routes;



#[database("personal_budget")]
struct BudgetDb(diesel::SqliteConnection);










######


#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::database;

#[database("budget_db")]
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

#[derive(Insertable, Queryable)]
#[diesel(table_name = transactions)]
struct DbTransaction {
    id: Option<i32>,
    description: String,
    amount: f64,
}

#[derive(diesel::table)]
#[diesel(table_name = "transactions")]
pub struct Transactions;

// CRUD 핸들러
#[post("/transaction", data = "<transaction>")]
async fn create_transaction(
    db: BudgetDb,
    transaction: Json<Transaction>,
) -> Json<&'static str> {
    let new_transaction = DbTransaction {
        id: None,
        description: transaction.description.clone(),
        amount: transaction.amount,
    };

    db.run(move |conn| {
        diesel::insert_into(transactions::table)
            .values(&new_transaction)
            .execute(conn)
    })
    .await
    .map(|_| Json("Transaction added successfully!"))
    .unwrap_or(Json("Failed to add transaction."))
}

// 라우트 정의
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(BudgetDb::fairing())
        .mount("/", routes![create_transaction])
}
