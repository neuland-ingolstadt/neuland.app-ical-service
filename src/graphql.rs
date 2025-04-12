use chrono::Utc;
use graphql_client::GraphQLQuery;

macro_rules! define_graphql_query {
    ($name:ident) => {
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = "src/gql/schema.json",
            query_path = "src/gql/queries.graphql",
            response_derives = "Debug, Serialize, Deserialize",
            scalar_derives = "Clone",
            variables_derives = "Clone",
            custom_scalars_module = "scalar_types"
        )]
        pub struct $name;
    };
}

define_graphql_query!(FetchEvents);
define_graphql_query!(NeulandEvents);

pub mod scalar_types {
    use super::*;
    pub type DateTime = chrono::DateTime<Utc>;
    pub type URL = String;
}
