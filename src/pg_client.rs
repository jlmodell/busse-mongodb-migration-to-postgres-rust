use crate::config::Config;

use sqlx::{postgres::{PgPoolOptions, PgRow}, PgPool};


pub struct PgClient {
    pool: PgPool,
}

impl PgClient {
    pub async fn new() -> Self {
        let config = Config::new();
        let conn_string = config.get_pg_conn_string();

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&conn_string)
            .await
            .unwrap();

        Self {
            pool,
        }
    }

    // todo
    pub async fn create_table_if_not_exists(&self, table: Option<&str>) {
        let mut sql = "".to_string();

        let _table = match table {
            Some(t) => {
                match t {
                    "sales" => {
                        sql = format!("CREATE TABLE IF NOT EXISTS {}
                        (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            key VARCHAR(255) NOT NULL,
                            distribution VARCHAR(255) NOT NULL,
                            rep VARCHAR(255) NOT NULL,
                            item VARCHAR(255) NOT NULL,
                            sale FLOAT NOT NULL,
                            quantity FLOAT NOT NULL,
                            uom VARCHAR(255) NOT NULL,
                            date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                            customer VARCHAR(255) NOT NULL,
                            ship_to_name VARCHAR(255) NOT NULL,
                            addr1 VARCHAR(255) NOT NULL,
                            addr2 VARCHAR(255) NOT NULL,
                            city VARCHAR(255) NOT NULL,
                            state VARCHAR(255) NOT NULL,
                            postal VARCHAR(255) NOT NULL DEFAULT '00000',
                            country VARCHAR(255),
                            contract VARCHAR(255),
                            cust_nbr VARCHAR(255),
                            gpo VARCHAR(255),
                            rebate FLOAT DEFAULT 0.0
                        )", "sales".to_string());
                    }

                    val => {
                        sql = format!("CREATE TABLE IF NOT EXISTS {} (id UUID PRIMARY KEY DEFAULT gen_random_uuid())", val.to_string());
                    }       
                }
            },
            None => panic!("No table specified."),
        };

        sqlx::query(&sql).execute(&self.pool).await.unwrap();
    }

    pub async fn query_respond_with_rows(&self, sql: Option<&str>) -> Vec<PgRow> {
        let _sql = match sql {
            Some(s) => s,
            None => panic!("No SQL statement specified."),
        };        

        let rows = sqlx::query(&_sql).fetch_all(&self.pool).await.unwrap();

        rows
    }
}


