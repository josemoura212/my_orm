use serde::{Deserialize, Serialize};

use crate::traits::entidades::HasId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aluno {
    pub id: i32,
    pub nome: String,
    pub matricula: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlunoNota {
    pub id: i32,
    pub aluno_id: i32,
    pub nota: f32,
}

impl HasId for Aluno {
    fn id(&self) -> i32 {
        self.id
    }

    fn campos_model(&self) -> Vec<String> {
        vec!["nome".to_string(), "matricula".to_string()]
    }
}
