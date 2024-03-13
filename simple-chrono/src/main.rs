use chrono::{Days, Local};
use std::ops::Add;

fn main() {
    {
        let now = Local::now();
        println!("{}", now);
    }
    
    let now = Local::now();
    let day = now.date_naive();
    println!("{}", day);
    
    let delta = Days::new(3);
    let cal = day.add(delta);
    println!("{}", cal);   
}

/* 
4/5 -> (+3 day) => 4/8
in_set_sql_day  <= 4/8

== in_set_sql_day ==
4/6 true
4/8 true
4/10 false
4.25 false

== sqlx code ==
select * from item
where in_set_sql_day <= ($1)
*/