struct Employee {
    name:String,
    Company:String,
    age:u32
}

fn main() {
    let emp1 = Employee {
        Company:String::from("Enrst & Young"),
        name:String::from("Ebibiong Jessica"),
        age:25
    };
    println!("Name = {} \n",emp1.name);
    println!("Company = {} \n",emp1.Company);
    println!("Age = {} \n",emp1.age);
}