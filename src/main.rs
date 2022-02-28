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

    let result = drop_table(&mut conn, "search_queries".to_owned())
        .expect("could not create the table");
    
    let result = create_table(&mut conn, create_table_string)
        .expect("could not create the table");

    let query = "select * from search_queries;";
    let mapper = |(
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
    )| {
        SearchQuery::init(
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
        )
    };
    let search_queries = vec![
        SearchQuery::init(
            "1".to_owned(),
            Some("2022-23-03".to_owned()),
            Some("2022-23-03".to_owned()),
            "1-1".to_owned(),
            "1-1-1".to_owned(),
            Some("last_7_days".to_owned()),
            Some(1000),
            Some(129),
            Some(12),
            Some(3.0),
            Some(4.50),
            Some(3.4),
            Some(4.2),
            Some(2.1),
            Some(34.5),
            Some(23.4),
            Some(33.0),
            Some("yes".to_owned()),
            Some("second".to_owned()) 
        ),
    ];
    if let Ok(result) = query_table(&mut conn, query, mapper) {
        if result == search_queries {
            println!("passed");
        } else {
            println!("issue");
        }
    }
}
