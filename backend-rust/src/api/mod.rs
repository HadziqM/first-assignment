//this to make a router

use rocket::{get,post};
use crate::database::{Dbase,Refresh};
use rocket::serde::json::Json;
use rocket::http::Status;


pub fn reg()->Vec<rocket::Route>{
    rocket::routes![list,delete,add,edit]
}


#[get("/list")]
async fn list(db:&Dbase)->Option<Json<Vec<Refresh>>> {
    let data = db.list().await.ok()?;
    Some(Json(data))
}

#[get("/delete/<id>")]
async fn delete(id:i32,db:&Dbase)->Status {
    match db.delete(id).await {
        Ok(_) => Status::Ok,
        Err(x) => {
                println!("{x}:?");
                Status::NotFound
            }
    }
}

#[derive(serde::Deserialize)]
struct AddRow {
    nama:String,
    harga:i32,
    kategori:String
}

#[post("/add", format="json", data="<row>")]
async fn add(row:Json<AddRow>,db:&Dbase)->Status {
    match db.add(&row.nama,row.harga,&row.kategori).await {
        Ok(_)=>Status::Ok,
        Err(x) => {
            println!("{x}:?");
            Status::NotFound
        }
    }
}

#[derive(serde::Deserialize)]
struct EditRow {
    nama:String,
    harga:i32,
    id:i32
}

#[post("/edit", format="json", data="<row>")]
async fn edit(row:Json<EditRow>,db:&Dbase)->Status{
    match db.edit(row.id,&row.nama,row.harga).await {
        Ok(_) => Status::Ok,
        Err(x) => {
            println!("{x}:?");
            Status::NotFound
        }
    }
}
