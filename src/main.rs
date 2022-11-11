mod employee_utils;
mod calculator_utils;

use employee_utils::collect_employee;

fn main() {
    let mut emp = collect_employee();

    emp.print();
    
}
