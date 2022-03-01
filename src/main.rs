pub mod database_utils {
    pub mod search_queries;
    pub mod utils;
    pub mod database; // because it's not `pub` it won't be visible outside of `ui`
}
use database_utils::search_queries::*;
use database_utils::utils::*;
use database_utils::database::*;

fn main() {
    let mut conn = get_conn().unwrap();
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

    let query = "select * from search_queries;".to_owned();
    query_table_new(&mut conn, query);
}
