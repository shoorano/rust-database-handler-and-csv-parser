#[derive(Debug, PartialEq, Eq)]
pub struct SearchQueryPerformance {
    pub id: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub account_id: String,
    pub search_query_id: String,
    pub date_range: Option<String>,
    pub impressions: Options<i32>,
    pub clicks: Options<i32>,
    pub conversions: Options<i32>,
    pub average_position: Options<f64>,
    pub cost: Options<f64>,
    pub arg_3: Option<String>,
}

impl SearchQueryPerformance {
    pub fn init(
        id: String,
        created_at: Option<String>,
        updated_at: Option<String>,
        account_id: String,
        search_query_id: String,
        date_range: Option<String>,
        impressions: Options<i32>,
        clicks: Options<i32>,
        conversions: Options<i32>,
        average_position: Options<f64>,
        cost: Options<f64>,
        arg_3: Option<String>,
    ) -> Self {
        Self {
            id,
            created_at,
            updated_at,
            account_id,
            search_query_id,
            date_range,
            impressions,
            clicks,
            conversions,
            average_position,
            cost,
            show_on_graph,
        }
    }
}