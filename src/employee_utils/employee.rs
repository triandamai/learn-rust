#[derive(Debug)]
pub struct Employee{
    pub name:String,
    pub category:String,
    pub marital_status:bool,
    pub number_of_children:i32,
    pub salary:i32,
    pub marital_insurance:i32,
    pub children_insurance:i32
}
impl Employee{
    fn calculate_category(&mut self){
        if self.category.trim() == "A" {
            self.salary = 1_000_000;
        }else if self.category.trim() == "B" {
            self.salary = 2_000_000;
        }else if self.category.trim() == "C" {
            self.salary = 3_000_000;
        }

    }

    fn calculate_marital(&mut self){
        if self.marital_status {
            if self.category.trim() == "A" {
                self.marital_insurance = 150_000;
            }else if self.category.trim() == "B" {
                self.marital_insurance = 250_000;
            }else if self.category.trim() == "C" {
                self.marital_insurance = 350_000;
            }
        }
    }

    fn calculate_children(&mut self){
        if self.marital_status{
            if self.number_of_children < 3{
                if self.category.trim() == "A" {
                    self.children_insurance = 150_000 * self.number_of_children;
                }else if self.category.trim() == "B" {

                    self.children_insurance = 250_000 * self.number_of_children;
                }else if self.category.trim() == "C" {
                    self.children_insurance = 350_000 * self.number_of_children;
                }
            }else {
                if self.category.trim() == "A" {
                    self.children_insurance = 150_000 * 2;
                }else if self.category.trim() == "B" {
                    self.children_insurance = 250_000 * 2;
                }else if self.category.trim() == "C" {
                    self.children_insurance = 350_000 * 2;
                }
            }
        }
    }

    pub fn print(&mut self){
        self.calculate_category();
        self.calculate_marital();
        self.calculate_children();

        println!("Nama : {}",self.name);
        println!("Gaji Pokok : {}",self.salary);
        println!("Tunjangan Istri : {}",self.marital_insurance);
        println!("Tunjangan Anak : {}",self.children_insurance);

    }

}