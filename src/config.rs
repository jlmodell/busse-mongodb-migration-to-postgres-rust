use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    mongo: MongoConfig,
    pg: PgConfig,    
}

#[derive(Deserialize, Debug)]
struct MongoConfig {
    uri: String,
}

#[derive(Deserialize, Debug)]
struct PgConfig {
    conn_string: String,
}

impl Config {
    pub fn new() -> Self {
        let config_file = std::fs::read_to_string("config.toml").unwrap();
        let config: Config = toml::from_str(&config_file).unwrap();

        config
    }

    pub fn get_mongo_uri(&self) -> &str {
        &self.mongo.uri
    }

    pub fn get_pg_conn_string(&self) -> &str {
        &self.pg.conn_string
    }
}
