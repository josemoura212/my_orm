use serde::{Deserialize, Serialize};

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
