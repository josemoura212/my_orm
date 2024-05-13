use crate::traits::entidades::HasId;

#[allow(unused)]
pub struct Materia {
    pub id: i32,
    pub titulo: String,
    pub descricao: String,
}

impl HasId for Materia {
    fn id(&self) -> i32 {
        self.id
    }

    fn campos_model(&self) -> Vec<(String, String)> {
        vec![
            ("titulo".to_string(), "varchar(50)".to_string()),
            ("descricao".to_string(), "varchar(255)".to_string()),
        ]
    }
}
