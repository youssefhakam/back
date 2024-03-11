extern crate mongodb;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::{Client, options::ClientOptions};
use rocket::serde::json::Json;


pub async fn connexion_database(db_name : &str, coll_name: &str) -> Collection<Document> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(db_name);
    let coll = db.collection::<Document>(coll_name);
    coll
}

pub async fn get_document(name : &str, db_name:&str, coll_name:&str) -> Document {
    let coll = connexion_database(db_name, coll_name).await;
    let filter = doc!{"name": String::from(name)};
    let result = coll.find_one(Some(filter), None).await.unwrap();
    match result {
        Some(doc) => doc,
        None => panic!("No document found"),
    }
}
pub async fn insert_document(data : Json<Document>, db_name: &str, coll_name: &str) {
    let coll = connexion_database(db_name, coll_name).await;
    let doc = data.into_inner();
    //coll.insert_one(doc, None).await.unwrap();
    coll.insert_one(doc, None).await.unwrap();
}
pub async fn update_document(filter : Json<Document>, age : u8,  db_name: &str, coll_name: &str) {
    let coll = connexion_database(db_name, coll_name).await;
    let filter = filter.into_inner(); //doc! {"name": "John"};
    let update = doc! {"$set": {"age" : i32::from(age)}};
    coll.update_one(filter, update, None).await.unwrap();
}
pub async fn delete_document(delete : Json<Document>, db_name: &str, coll_name: &str) {
    let coll = connexion_database(db_name, coll_name).await;
    let filter = delete.into_inner();
    coll.delete_one(filter, None).await.unwrap();
}