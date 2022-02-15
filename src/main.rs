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

// fn insert_into_table<T>(conn: &mut PooledConn, table: String, data: Vec<T>) {
//     // Now let's insert testing to the database
//     // conn.exec_batch(
//     //     r"INSERT INTO testing (customer_id, amount, account_name)
//     //     VALUES (:customer_id, :amount, :account_name)",
//     //     data.iter().map(|p| params! {
//     //         "customer_id" => p.customer_id,
//     //         "amount" => p.amount,
//     //         "account_name" => &p.account_name,
//     //     })
//     // )?;
// }

/// function that receives a query string and a database connect 
/// and makes a call to create a mysql table
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
        Example { arg_1, arg_2, arg_3 }
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