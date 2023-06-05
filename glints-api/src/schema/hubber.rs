use async_graphql::{ComplexObject, SimpleObject};

#[derive(SimpleObject, Debug)]
#[graphql(complex)]
pub struct Hubber {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[ComplexObject]
impl Hubber {
    async fn computed_string(&self) -> String {
        format!("{} - {} - {}", self.id, self.code, self.name)
    }
}
