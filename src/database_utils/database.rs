use super::utils::*;
use mysql::*;
use mysql::prelude::*;

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

pub fn query_table_new(conn: &mut PooledConn, query: String) -> Result<()> {
    let row: Row = conn.exec_first(query, ())?.unwrap();
    println!("{:?}", row);
    for i in 0..row.len() {
        println!("{:?}", row[i]);
    };
    Ok(())
}
