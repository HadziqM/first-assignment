pub mod api;
pub mod database;
mod config;


use rocket_db_pools::Database;
use rocket::fs::FileServer;

//let run the built frontent into rocket fileserver
fn fileserver() -> FileServer {
    //path into frontend output file
    let path = std::path::Path::new("..").join("frontend").join("dist");
    FileServer::from(path)
}



#[rocket::launch]
fn rocket()-> _ {
    rocket::custom(config::db_conf())
        .mount("/",fileserver())
        .mount("/api",api::reg())
        .attach(database::Dbase::init())
}
