use chrono::NaiveDate;
use diesel::mysql::types::Datetime;
use diesel::sql_types::Date;

#[derive(Queryable,Debug,Serialize)]
pub struct Employees {
    pub emp_no: i32,
    pub birth_date: NaiveDate,
    pub first_name: String,
    pub last_name: String,
    pub hire_date: NaiveDate
}
