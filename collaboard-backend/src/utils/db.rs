use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager};

use crate::{DbPool, Error, utils};

pub fn establish_connection()->DbPool{
    let database_url=(*utils::constants::DATABASE_URL).clone();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).map_err(|_| Error::FailToGetPool).expect("Fail to get pool");
    pool
}

