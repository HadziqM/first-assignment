// thsis to make struct for all database transaction



// rocket have database trait to make it easy to bind your own struct into that very trait
use rocket_db_pools::{Database,sqlx};

//make struct that can send json file (need serialize trait)
#[derive(serde::Serialize,Clone,sqlx::FromRow,Debug)]
pub struct Refresh {
  id:i32,
  nama:String,
  kategori:String,
  harga:i32
}

#[derive(Database,Clone)]
#[database("name")]
pub struct Dbase(sqlx::PgPool);


impl Dbase {
    pub async fn add(&self,name:&str,harga:i32,kategori:&str)->Result<(),sqlx::Error> {
        sqlx::query("insert into produk (name,harga,kategori_id,status_id) values 
            ($1,$2,(select id from kategori where name=$3 limit 1),(select id from status where name='bisa dijual'))")
            .bind(name).bind(harga).bind(kategori).execute(&**self).await?;
        Ok(())
    }
    pub async fn delete(&self,id:i32)-> Result<(),sqlx::Error>{
        sqlx::query("delete from produk where id=$1").bind(id).execute(&**self).await?;
        Ok(())
    }
    pub async fn edit(&self,id:i32,name:&str,harga:i32)-> Result<(),sqlx::Error>{
        sqlx::query("update produk set name=$1,harga=$2 where id=$3").bind(name).bind(harga).bind(id).execute(&**self).await?;
        Ok(())
    }
    pub async fn list(&self)->Result<Vec<Refresh>,sqlx::Error> {
        let data = sqlx::query_as::<_,Refresh>("select produk.name as nama, produk.harga as harga, kategori.name as kategori,
            produk.id as id from produk inner join kategori on kategori.id=produk.kategori_id inner join status on status.id=produk.status_id
            where status.name='bisa dijual'").fetch_all(&**self).await?;
        Ok(data)
    }
}




//code for automatic testing
#[cfg(test)]
mod testing {
    use super::*;
    use crate::config::Postgres;

    impl Dbase {
        pub async fn new()->Result<Self,sqlx::Error>{
            let x = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5).connect(&Postgres::default().url()).await?;
            Ok(Self(x.into()))
        }
    }

    #[rocket::async_test]
    async fn list() {
        let db = Dbase::new().await.unwrap();
        println!("{:?}",db.list().await.unwrap());
    }
    #[rocket::async_test]
    async fn edit() {
        let db = Dbase::new().await.unwrap();
        println!("{:?}",db.edit(1,"edited",10).await.unwrap());
    }
    #[rocket::async_test]
    async fn delete() {
        let db = Dbase::new().await.unwrap();
        println!("{:?}",db.delete(3).await.unwrap());
    }
    #[rocket::async_test]
    async fn add() {
        let db = Dbase::new().await.unwrap();
        println!("{:?}",db.add("added",10,"CI MTH TINTA LAIN (IM)").await.unwrap());
    }
}
