use std::process::id;

use inflector::Inflector;
use mysql::{prelude::Queryable, Pool, PooledConn};

use crate::traits::entidades::HasId;

pub struct MyORM {
    pool: Pool,
}

impl MyORM {
    pub fn new(connection: &str) -> Self {
        MyORM {
            pool: Pool::new(connection).unwrap(),
        }
    }
    fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    #[allow(unused)]
    fn table_name<T: HasId>(&self, model: &T) -> String {
        std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap()
            .to_snake_case()
    }

    #[allow(unused)]
    pub fn detalhe_tablea<T: HasId>(&self, model: &T) -> String {
        let mut sql = String::from("id integer not null auto_increment,");

        let nome_campos = model.campos_model();

        for (field, tipo) in nome_campos {
            sql.push_str(&format!("{} {},", field, tipo));
        }

        sql.push_str("primary key (id)");

        sql
    }

    pub fn create_table<T: HasId>(&self, model: T) {
        let mut conn = self.get_conn();

        let table_name = self.table_name(&model);
        let detalhe = self.detalhe_tablea(&model);

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {}s (
                {}
        )",
            table_name, detalhe
        );
        println!("===============================");
        println!("{}", sql);
        println!("===============================");
        conn.query_drop(sql).unwrap();
    }

    pub fn incluir<T: HasId>(&self, model: &T) -> i32 {
        let mut conn = self.get_conn();

        let table_name = self.table_name(model);
        let campos = model.campos_model();

        let sql = format!("INSERT INTO {}s ", table_name,);

        let result = conn.query_first(sql).unwrap();

        result.unwrap()
    }
}
