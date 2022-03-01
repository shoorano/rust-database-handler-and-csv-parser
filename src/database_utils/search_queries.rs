use super::database::*;
use super::utils::*;
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SearchQuery {
    pub id: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub account_id: String,
    pub search_query_id: String,
    pub date_range: Option<String>,
    pub impressions: Option<i32>,
    pub clicks: Option<i32>,
    pub conversions: Option<i32>,
    pub average_position: Option<f64>,
    pub cost: Option<f64>,
    pub average_cpc: Option<f64>,
    pub conversion_value: Option<f64>,
    pub cpa: Option<f64>,
    pub roas: Option<f64>,
    pub conversion_rate: Option<f64>,
    pub ctr: Option<f64>,
    pub show_on_graph: Option<String>,
    pub graph_order: Option<String>,
}

impl SearchQuery {
    pub fn init(
        id: String,
        created_at: Option<String>,
        updated_at: Option<String>,
        account_id: String,
        search_query_id: String,
        date_range: Option<String>,
        impressions: Option<i32>,
        clicks: Option<i32>,
        conversions: Option<i32>,
        average_position: Option<f64>,
        cost: Option<f64>,
        average_cpc: Option<f64>,
        conversion_value: Option<f64>,
        cpa: Option<f64>,
        roas: Option<f64>,
        conversion_rate: Option<f64>,
        ctr: Option<f64>,
        show_on_graph: Option<String>,
        graph_order: Option<String>
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
            average_cpc,
            conversion_value,
            cpa,
            roas,
            conversion_rate,
            ctr,
            show_on_graph,
            graph_order
        }
    }
}

