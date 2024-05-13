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
            .to_lowercase()
    }

    #[allow(unused)]
    pub fn detalhe_tablea<T: HasId>(&self, model: &T) -> String {
        let mut sql = String::from("id integer not null auto_increment,");

        let nome_campos = model.campos_model();

        for field in nome_campos {
            sql.push_str(&format!("{} varchar(255),", field));
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
}

//     pub fn get_all(&self) -> Option<Vec<Aluno>> {
//         let mut conn = self.get_conn();

//         let query = "SELECT
//                                 alu.matricula,
//                                 alu.nome,
//                             group_concat(nota.materia,':',nota.nota separator ';') as notas
//                             FROM aluno alu inner join nota on alu.matricula = nota.matricula
//                             GROUP BY alu.matricula;";
//         let alunos: Vec<Aluno> = conn
//             .query_map(
//                 query,
//                 |(matricula, nome, notas): (String, String, String)| Aluno {
//                     matricula,
//                     nome,
//                     notas: notas
//                         .split(';')
//                         .map(|nota| {
//                             let mut nota = nota.split(':');
//                             AlunoNota {
//                                 disciplina: nota.next().unwrap().to_string(),
//                                 nota: nota.next().unwrap().parse().unwrap(),
//                             }
//                         })
//                         .collect(),
//                 },
//             )
//             .unwrap();

//         Some(alunos)
//     }

//     pub fn get_by_matricula(&self, matricula: String) -> Option<Aluno> {
//         let mut conn = self.get_conn();

//         let query = "SELECT
//                                 alu.matricula,
//                                 alu.nome,
//                             group_concat(nota.materia,':',nota.nota separator ';') as notas
//                             FROM aluno alu inner join nota on alu.matricula = nota.matricula
//                             WHERE alu.matricula = :matricula
//                             GROUP BY alu.matricula;";
//         let aluno: Option<Aluno> = conn
//             .exec_first(
//                 query,
//                 params! {
//                     "matricula" => &matricula
//                 },
//             )
//             .unwrap()
//             .map(|(matricula, nome, notas): (String, String, String)| Aluno {
//                 matricula,
//                 nome,
//                 notas: notas
//                     .split(';')
//                     .map(|nota| {
//                         let mut nota = nota.split(':');
//                         AlunoNota {
//                             disciplina: nota.next().unwrap().to_string(),
//                             nota: nota.next().unwrap().parse().unwrap(),
//                         }
//                     })
//                     .collect(),
//             });

//         aluno
//     }

//     pub fn save(&self, aluno: Aluno) {
//         let mut conn = self.get_conn();
//         let mut tx = conn.start_transaction(TxOpts::default()).unwrap();

//         tx.exec_drop(
//             "INSERT INTO aluno (matricula, nome) VALUES (:matricula, :nome)",
//             params! {
//                 "matricula" => &aluno.matricula,
//                 "nome" => aluno.nome,
//             },
//         )
//         .unwrap();

//         for nota in &aluno.notas {
//             tx.exec_drop(
//                 "INSERT INTO nota (matricula, materia, nota) VALUES (:matricula, :materia, :nota)",
//                 params! {
//                     "matricula" => &aluno.matricula,
//                     "materia" => &nota.disciplina,
//                     "nota" => &nota.nota,
//                 },
//             )
//             .unwrap();
//         }

//         tx.commit().unwrap();
//     }

//     pub fn change(&self, aluno: Aluno) {
//         let mut conn = self.get_conn();

//         conn.exec_drop(
//             "UPDATE aluno SET nome = :nome WHERE matricula = :matricula",
//             params!(
//                 "nome" => aluno.nome,
//                 "matricula" => &aluno.matricula
//             ),
//         )
//         .unwrap();

//         for nota in &aluno.notas {
//             conn.exec_drop(
//                 "UPDATE nota SET nota = :nota WHERE matricula = :matricula AND materia = :materia",
//                 params!(
//                     "nota" => &nota.nota,
//                     "matricula" => &aluno.matricula,
//                     "materia" => &nota.disciplina
//                 ),
//             )
//             .unwrap();
//         }
//     }

//     pub fn delete(&self, matricula: String) {
//         let mut conn = self.get_conn();
//         let mut tx = conn.start_transaction(TxOpts::default()).unwrap();

//         tx.exec_drop(
//             "DELETE FROM nota WHERE matricula = :matricula",
//             params!(
//                 "matricula" => &matricula
//             ),
//         )
//         .unwrap();

//         tx.exec_drop(
//             "DELETE FROM aluno WHERE matricula = :matricula",
//             params!(
//                 "matricula" => &matricula
//             ),
//         )
//         .unwrap();

//         tx.commit().unwrap();
//     }
// }
