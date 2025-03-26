use chrono::Utc;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/queries.graphql",
    response_derives = "Debug, Serialize, Deserialize",
    scalar_derives = "Clone",
    variables_derives = "Clone",
    custom_scalars_module = "scalar_types"
)]
pub struct FetchEvents;

pub mod scalar_types {
    use super::*;
    pub type DateTime = chrono::DateTime<Utc>;
    pub type URL = String;
}
