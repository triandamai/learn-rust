use employee::Employee;
use std::io;

pub fn collect_employee()->Employee{
    let mut name = String::from("");

    let mut category = String::from("");
    let mut category_fail = true;

    let mut marital= String::from("");
    let mut marital_fail= true;
    
    let mut children = String::from("0");
    let mut children_fail = true;

   
    println!("Input name : ");
    io::stdin()
        .read_line(&mut name)
        .expect("Slaahs");

    while category_fail {
        category = String::from("");
        println!("Input Category[A,B,C] : ");
        io::stdin()
            .read_line(&mut category)
            .expect("Input A,B or C");

        if category.trim() == "A" || 
            category.trim() == "B" || 
            category.trim() == "C" ||
            category.trim() == "a" ||
            category.trim() == "b" ||
            category.trim() == "c" {
            category_fail = false;
        }else {
            category_fail = true;
        }
    }

            
    while marital_fail {
            marital = String::from("");

            println!("Already Married?[Y/N] :");
            io::stdin()
                .read_line(&mut marital)
                .expect("Input Y or N!");


            if marital.trim() == "Y" || marital.trim() == "y" || marital.trim() == "N" || marital.trim() == "n" {
                marital_fail = false
            }else{
                marital_fail = true
            }
    }

    if marital.trim() == "Y" || marital.trim() == "y" {
        while children_fail {
            children = String::from("0");
            println!("Input number of children[1,2,3...] :");
            io::stdin()
                .read_line(&mut children)
                .expect("Please input number only!");

               
            if children.trim().parse().unwrap_or(300) != 300{
                children_fail = false;
            }else{
                children_fail = true;
            }
        }
    }
    

     return Employee{
         name,
         category,
         marital_status:marital.trim() == "Y" || marital.trim() == "N" || marital.trim() == "y" || marital.trim() == "n",
         number_of_children:children.trim().parse().unwrap_or(0),
         marital_insurance:0,
         children_insurance:0,
         salary:0
    };  
}


pub mod employee;