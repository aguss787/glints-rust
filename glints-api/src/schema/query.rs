use crate::schema::common::Paginated;
use crate::schema::hubber::Hubber;
use crate::{APIModule, GraphQLResult};
use async_graphql::{Context, Object};
use glints_managed_talent::hubber::{HubberAPI, PaginationOptions};
use shaku::HasComponent;
use std::cmp::max;
use std::sync::Arc;

#[derive(Default)]
pub struct Query {}

#[Object]
impl Query {
    async fn hubbers(
        &self,
        ctx: &Context<'_>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> GraphQLResult<Paginated<Hubber>> {
        let module: &APIModule = ctx.data().unwrap();
        let hubber_service: Arc<dyn HubberAPI> = module.resolve();

        let page_size = page_size.unwrap_or(10);
        let page_no = max(page.unwrap_or(1), 1);
        let offset = page_size * (page_no - 1);

        let hubbers = hubber_service
            .list_hubber(&PaginationOptions {
                size: page_size.into(),
                offset: offset.into(),
            })
            .await?
            .into_iter()
            .map(|i| Hubber {
                id: i.id,
                code: i.code,
                name: i.name,
            })
            .collect();

        Ok(Paginated {
            data: hubbers,
            total: hubber_service.count_hubber().await?,
            page_no,
            page_size,
        })
    }
}
