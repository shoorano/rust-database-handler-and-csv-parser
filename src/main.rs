mod example_table;
use example_table::*;
mod utils;
use utils::*;
use mysql::*;
use mysql::prelude::*;


/// returns a database connection: PooledConn
fn get_conn() -> Result<PooledConn> {
    let opts = build_connection_options_from_env();
    let pool = Pool::new(opts)?;
    pool.get_conn()
}

/// function that receives a String and makes a call to create a mysql table
/// 
/// args:
///     conn: &mut PooledConn
///     create_table_string: String
/// 
/// create_table_string example:
///     r"CREATE TEMPORARY TABLE payment (
///     customer_id int not null,
///     amount int not null,
///     account_name text
///     )"
/// 
fn create_table(conn: &mut PooledConn, create_table_string: String) -> Result<()> {
    conn.query_drop(create_table_string)?;
    Ok(())
}

/// function that receives a query string, a mapper closure and a database connection 
/// and returns the query result which is mapped using the mapper closure
/// 
/// args:
///     conn: &mut PooledConn
///     query: AsRef<str>,
///     mapper: FnMut(T) -> U
/// 
/// query example:
///     "select * from testing;";
/// mapper example:
///     |(arg_1, arg_2, arg_3)| {
///         Struct { arg_1, arg_2, arg_3 }
///     }
/// 
fn query_table<T, F, Q, U>(conn: &mut PooledConn, query: Q, mapper: F) -> Result<Vec<U>> 
where
        Q: AsRef<str>,
        T: FromRow,
        F: FnMut(T) -> U,
{
    let result = conn
        .query_map(
            query,
            mapper,
        )?;
    Ok(result)
}

fn main() {
    let mut conn = get_conn().unwrap();
    let query = "select * from testing;";
    let mapper = |(arg_1, arg_2, arg_3)| {
        Example::init(arg_1, arg_2, arg_3)
    };
    let examples = vec![
        Example { arg_1: 1, arg_2: 2, arg_3: None },
        Example { arg_1: 3, arg_2: 4, arg_3: Some("foo".into()) },
        Example { arg_1: 5, arg_2: 6, arg_3: None },
        Example { arg_1: 7, arg_2: 8, arg_3: None },
        Example { arg_1: 9, arg_2: 10, arg_3: Some("bar".into()) },
    ];
    if let Ok(result) = query_table(&mut conn, query, mapper) {
        if result == examples {
            println!("passed");
        } else {
            println!("issue");
        }
    }
}