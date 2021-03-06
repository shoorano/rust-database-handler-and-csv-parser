use mysql::*;
use super::utils::*;
use mysql::prelude::*;
use chrono::naive::{NaiveDateTime, NaiveDate, NaiveTime};

/// returns a database connection: PooledConn
pub fn get_conn() -> Result<PooledConn> {
    let opts = build_connection_options_from_env();
    let pool = Pool::new(opts)?;
    pool.get_conn()
}

/// receives a table name String which identifies the table name to be delete, then executes a drop
/// table mysql call
///
/// args:
///     conn: &mut PooledConn
///     table_name: String
///
pub fn drop_table(conn: &mut PooledConn, table_name: String) -> Result<()> {
    let query = format!("drop table {}", table_name);
    conn.query_drop(query)?;
    Ok(())
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
pub fn create_table(conn: &mut PooledConn, create_table_string: String) -> Result<()> {
    conn.query_drop(create_table_string)?;
    Ok(())
}

/// function that receives a query string, a mapper closure and a database connection 
/// and returns the query result which is mapped using the mapper closure
/// 
/// args:
///     conn: &mut PooledConn query: AsRef<str>,
///     mapper: FnMut(T) -> U
/// 
/// query example:
///     "select * from testing;";
/// mapper example:
///     |(arg_1, arg_2, arg_3)| {
///         Struct { arg_1, arg_2, arg_3 }
///     }
/// 
pub fn query_table<T, F, Q, U>(conn: &mut PooledConn, query: Q, mapper: F) -> Result<Vec<U>> 
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

/// query a table and returns a Row
pub fn query_table_new(conn: &mut PooledConn, query: String) -> Result<Row> {
    Ok(conn.exec_first(query, ())?.unwrap())
}

/// parses the mysql::Value::Date to chrono::naive::NaiveDateTime
pub fn parse_time_to_naive_date_time(date: Value) -> Option<NaiveDateTime> {
    match date {
        Value::Date(y, m, d, h, i, s, us) => Some(
                NaiveDateTime::new(
                    NaiveDate::from_ymd(y as i32, m as u32, d as u32),
                    NaiveTime::from_hms_nano(h as u32, i as u32, s as u32, us)
                )
            ),
        _ => None
    }
}
