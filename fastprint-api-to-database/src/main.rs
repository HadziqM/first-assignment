#![allow(unused,dead_code)]


use std::collections::HashMap;
use std::path::Path;
use itertools::Itertools;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};


//struc for json config
#[derive(serde::Deserialize)]
struct FastprintPost{
    username:String,
    password:String
}

impl Default for FastprintPost {
    fn default() -> Self {
        let path = Path::new("..").join("config.json");
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }
}

impl FastprintPost {
    fn params(&self) -> HashMap<String,String> {
        let mut par = HashMap::new();
        par.insert("username".to_owned(),self.username.to_owned());
        par.insert("password".to_owned(),self.password.to_owned());
        par
    }
}

//struct for api output
#[derive(Debug,serde::Deserialize)]
struct FastPrintOut {
    data:Vec<ApiData>
}
#[derive(Debug,serde::Deserialize)]
struct ApiData {
    id_produk:String,
    nama_produk:String,
    kategori:String,
    harga:String,
    status:String
}

impl FastPrintOut {
    async fn get_data() -> Result<Self,reqwest::Error> {
        let client = reqwest::Client::new();
        let data = client.post("https://recruitment.fastprint.co.id/tes/api_tes_programmer")
            .form(&FastprintPost::default().params())
            .send()
            .await?.json().await?;
        Ok(data)
    }
    fn get_unique_status(&self)->Vec<String> {
        let mut status = Vec::new();
        for x in &self.data {
            status.push(x.status.to_owned());
        }
        status.into_iter().unique().collect()
    }
    fn get_unique_kategori(&self)->Vec<String> {
        let mut status = Vec::new();
        for x in &self.data {
            status.push(x.kategori.to_owned());
        }
        status.into_iter().unique().collect()
    }
}


//struct for postgres interface
#[derive(Debug)]
struct Dbase(Pool<Postgres>);


//make struct for json config
#[derive(serde::Deserialize,Clone,Debug)]
struct PostConf {
    host:String,
    password:String,
    username:String,
    database:String,
    port:u16
}

impl Default for PostConf {
    fn default() -> Self {
        let path = std::path::Path::new("..").join("backend-rust").join("config.json");
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }
}

impl PostConf {
    fn url(&self)-> String {
        format!("postgress://{}:{}@{}:{}/{}"
            ,self.username
            ,self.password
            ,self.host
            ,self.port
            ,self.database
        )
    }
}

impl std::ops::Deref for Dbase {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Dbase {
    async fn connect() -> Result<Self,sqlx::Error> {
        Ok(Self(PgPoolOptions::new()
            .max_connections(10)
            .connect(&PostConf::default().url())
            .await?))
    }
    async fn insert_status(&self,fastprint:&FastPrintOut) -> Result<(),sqlx::Error> {
        for i in fastprint.get_unique_status() {
            sqlx::query("insert into status (name) Values ($1)").bind(i).execute(&**self).await?;
        }
        Ok(())
    }
    async fn insert_kategori(&self,fastprint:&FastPrintOut) -> Result<(),sqlx::Error> {
        for i in fastprint.get_unique_kategori() {
            sqlx::query("insert into kategori (name) Values ($1)").bind(i).execute(&**self).await?;
        }
        Ok(())
    }
    async fn insert_produk(&self,fastprint:&FastPrintOut) -> Result<(),sqlx::Error> {
        for i in &fastprint.data {
            sqlx::query("insert into produk (name,harga,kategori_id,status_id) Values 
                ($1,$2,(select id from kategori where name=$3 limit 1),(select id from status where name=$4 limit 1))")
                .bind(&i.nama_produk)
                .bind(i.harga.parse::<i32>().unwrap())
                .bind(&i.kategori)
                .bind(&i.status)
                .execute(&**self)
                .await?;
        }
        Ok(())
    }
    async fn migrate(&self,fastprint:&FastPrintOut) -> Result<(),sqlx::Error> {
        self.insert_status(fastprint).await?;
        self.insert_kategori(fastprint).await?;
        self.insert_produk(fastprint).await?;
        Ok(())
    }
}


//main function connect database the igrate data from API
#[tokio::main]
async fn main() {
    let db = Dbase::connect().await.unwrap();
    match FastPrintOut::get_data().await {
        Ok(x) => {
            db.migrate(&x).await.unwrap();
            println!("success");
        }
        Err(x)=> println!("{x:?}")
    }
}
