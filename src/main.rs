extern crate mongodb;
use mongodb::bson::{doc, Document};
use rocket::serde::json::Json;
#[macro_use] extern crate rocket;

mod crud {
    pub mod func;
}

#[get("/get?<name>")]
async fn get_doc(name : &str) -> Json<Document> {
    let document = crud::func::get_document(name, "mydatabase", "mycollection").await;
    Json(document)
}
#[post("/insert", format = "json", data = "<data>")]
async fn insert(data: Json<Document>) {
    crud::func::insert_document(data, "mydatabase", "mycollection").await
}
#[put("/update?<age>", format = "json", data= "<data>")]
async fn update(data : Json<Document>, age : i8) {
    crud::func::update_document(data, age.try_into().unwrap(), "mydatabase", "mycollection").await
}
#[delete("/delete", format = "json", data = "<data>")]
async fn delete(data: Json<Document>) {
    crud::func::delete_document(data, "mydatabase", "mycollection").await
}
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
    .mount("/", routes![get_doc])
    .mount("/", routes![insert])
    .mount("/", routes![update])
    .mount("/", routes![delete])
    .launch()
    .await?;

    Ok(())
}
