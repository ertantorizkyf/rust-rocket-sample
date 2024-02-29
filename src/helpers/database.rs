use crate::constants::database::DB_URL;

use mysql::*;

pub fn get_conn() -> PooledConn {
    let db_url = DB_URL;
    let pool = Pool::new(db_url).unwrap();
    let conn = pool.get_conn().unwrap();

    conn
}
