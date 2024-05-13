pub trait HasId {
    #[allow(unused)]
    fn id(&self) -> i32;
    fn campos_model(&self) -> Vec<(String, String)>;
}
