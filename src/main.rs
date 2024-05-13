use crate::models::{
    aluno::{Aluno, AlunoNota},
    materia::Materia,
};
use orm::my_orm::MyORM;

mod models {
    pub mod aluno;
    pub mod materia;
}
mod core_config {
    pub mod config;
}

mod orm {
    pub mod my_orm;
}

mod traits {
    pub mod entidades;
}

fn main() {
    let connection = core_config::config::get_mysql_db_alunos_path();

    let aluno = Aluno {
        id: 1,
        nome: "João".to_string(),
        matricula: "123456".to_string(),
    };

    let aluno_nota = AlunoNota {
        id: 1,
        aluno_id: aluno.id,
        nota: 10.0,
    };

    let materia = Materia {
        id: 1,
        titulo: "Matemática".to_string(),
        descricao: "Matemática para logica de programção".to_string(),
    };

    let repo = MyORM::new(&connection);

    repo.create_table(aluno);
    repo.create_table(aluno_nota);
    repo.create_table(materia);

    println!("Tabela criada com sucesso!")
}
