table! {
    departments (dept_no) {
        dept_no -> Char,
        dept_name -> Varchar,
    }
}

table! {
    dept_emp (emp_no, dept_no) {
        emp_no -> Integer,
        dept_no -> Char,
        from_date -> Date,
        to_date -> Date,
    }
}

table! {
    dept_manager (emp_no, dept_no) {
        emp_no -> Integer,
        dept_no -> Char,
        from_date -> Date,
        to_date -> Date,
    }
}

table! {
    employees (emp_no) {
        emp_no -> Integer,
        birth_date -> Date,
        first_name -> Varchar,
        last_name -> Varchar,
        hire_date -> Date,
    }
}

table! {
    salaries (emp_no, from_date) {
        emp_no -> Integer,
        salary -> Integer,
        from_date -> Date,
        to_date -> Date,
    }
}

table! {
    titles (emp_no, title, from_date) {
        emp_no -> Integer,
        title -> Varchar,
        from_date -> Date,
        to_date -> Nullable<Date>,
    }
}

joinable!(dept_emp -> departments (dept_no));
joinable!(dept_emp -> employees (emp_no));
joinable!(dept_manager -> departments (dept_no));
joinable!(dept_manager -> employees (emp_no));
joinable!(salaries -> employees (emp_no));
joinable!(titles -> employees (emp_no));

allow_tables_to_appear_in_same_query!(
    departments,
    dept_emp,
    dept_manager,
    employees,
    salaries,
    titles,
);
