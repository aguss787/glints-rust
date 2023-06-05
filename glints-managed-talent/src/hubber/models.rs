use crate::hubber::HubberResult;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use glints_infra::diesel_schema::hubbers;

#[derive(Debug, Queryable)]
pub struct Hubber {
    pub id: uuid::Uuid,
    pub code: String,
    pub name: String,
}

impl Hubber {
    pub async fn get_paged(
        conn: &mut impl AsyncConnection<Backend = Pg>,
        size: i64,
        offset: i64,
    ) -> HubberResult<Vec<Self>> {
        Ok(hubbers::dsl::hubbers
            .order(hubbers::dsl::code.asc())
            .limit(size)
            .offset(offset)
            .load::<Self>(conn)
            .await?)
    }

    pub async fn count(conn: &mut impl AsyncConnection<Backend = Pg>) -> HubberResult<i64> {
        Ok(hubbers::dsl::hubbers.count().first::<i64>(conn).await?)
    }
}
