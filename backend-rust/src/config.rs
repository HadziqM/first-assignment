//config for postgresql
use serde::Deserialize;
use rocket::figment::Figment;


//make struct for json config
#[derive(Deserialize,Clone,Debug)]
pub struct Postgres {
    host:String,
    password:String,
    username:String,
    database:String,
    port:u16
}

impl Default for Postgres {
    fn default() -> Self {
        let path = std::path::Path::new(".").join("config.json");
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }
}

impl Postgres {
    pub fn url(&self)->String {
        format!("postgress://{}:{}@{}:{}/{}"
            ,self.username
            ,self.password
            ,self.host
            ,self.port
            ,self.database
        )
    }
}

// rocket config can be customize with Figment
// im naming the database "name"
pub fn db_conf()-> Figment {
    rocket::Config::figment().merge(("databases.name",rocket_db_pools::Config {
        url:Postgres::default().url(),
        max_connections:10,
        min_connections:None,
        idle_timeout:None,
        connect_timeout:3
    }))
}



#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn load_config() {
        println!("{:?}",Postgres::default());
    }
}
