// declare a structure
struct Employee {
    ceo:String,
    company:String,
    age:String,
}
fn main() {
    //initialize a stucture
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56.to_string(),
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51.to_string(),
    };
    //pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}
// fetch values of specific structure fields using the
// Operator and print it to the console
fn display( emp:Employee){
    println!("Name is :{} company is {} age display
        {}",emp.ceo,emp.company,emp.age);
}