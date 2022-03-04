pub mod database_utils {
    pub mod search_queries;
    pub mod utils;
    pub mod database; 
}
use database_utils::search_queries::*;
use database_utils::database::*;

fn main() {
    let mut conn = get_conn().unwrap();
    let query = "select * from search_queries;".to_owned();
    let search_query = SearchQuery::from_row(
        &mut query_table_new(&mut conn, query).unwrap()
    );
    println!("{:?}", search_query);
}
