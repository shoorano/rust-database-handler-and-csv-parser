use mysql::*;
use super::database::*;
use chrono::naive::NaiveDateTime;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SearchQuery {
    pub id: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
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
    /// creates a SearchQuery struct
    pub fn init(
        id: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
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
    /// receives a mysqsl::Value::Row struct
    /// returns a SearchQuery struct from row data
    pub fn from_row(row: &mut Row) -> Self {
        Self::init(
            row.take("id").unwrap(),
            parse_time_to_naive_date_time(row.take("created_at").unwrap()),
            parse_time_to_naive_date_time(row.take("updated_at").unwrap()),
            row.take("account_id").unwrap(),
            row.take("search_query_id").unwrap(),
            row.take("date_range"),
            row.take("impressions"),
            row.take("clicks"),
            row.take("conversions"),
            row.take("average_position"),
            row.take("cost"),
            row.take("average_cpc"),
            row.take("conversion_value"),
            row.take("cpa"),
            row.take("roas"),
            row.take("conversion_rate"),
            row.take("ctr"),
            row.take("show_on_graph"),
            row.take("graph_order"),
        )
    }
    /// create the search_query table
    pub fn create_table(conn: &mut PooledConn) {
        let create_table_string = "
            create table search_queries (
            id char(36) not null primary key,
            created_at timestamp,
            updated_at timestamp,
            account_id char(36) not null,
            search_query_id char(36) not null,
            date_range char(100),
            impressions int,
            clicks int,
            conversions int,
            average_position decimal(11,1),
            cost decimal(11,2),
            average_cpc decimal(11,2),
            conversion_value decimal(11,2),
            cpa decimal(11,2),
            roas decimal(11,2),
            conversion_rate decimal(11,2),
            ctr decimal(11,2),
            show_on_graph varchar(191),
            graph_order varchar(191)
        );
        ".to_owned();
        create_table(conn, create_table_string)
            .expect("could not create search_query table");
    }
}

