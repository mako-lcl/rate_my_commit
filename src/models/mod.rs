use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Rating {
    pub rating: isize,
    pub emoji: String,
    pub comment: String,
}
