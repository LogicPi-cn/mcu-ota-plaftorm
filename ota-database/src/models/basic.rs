use diesel::PgConnection;
use rand::{distributions::*, thread_rng, Rng};

use crate::DbError;

pub fn random_string(len: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    return rand_string;
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(min..max);
    number
}

pub fn random_i32() -> i32 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(i32::MIN..i32::MAX);
    number
}

pub fn random_i64() -> i64 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(i64::MIN..i64::MAX);
    number
}

pub trait HasId {
    fn id(&self) -> i32;
}

pub trait CrudOperations<T0, Tn, Tu> {
    fn all(conn: &mut PgConnection) -> Result<Vec<T0>, DbError>;
    fn find(id: i32, conn: &mut PgConnection) -> Result<T0, DbError>;
    fn create(data: Tn, conn: &mut PgConnection) -> Result<T0, DbError>;
    fn update(id: i32, data: Tu, conn: &mut PgConnection) -> Result<T0, DbError>;
    fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, DbError>;
}
