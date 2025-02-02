 //declare a sturcture
 struct Laptop {
    brand_name:String,
    price:u32,
    quantity:u32,
 }
 fn main() {
     //initialize a structure
     let hp = Laptop {
        brand:String::from("HP"),
        price:650_000.00,
        quantity: 10,
     };
     let ibm = Laptop {
        brand:String::from("IBM"),
        price:755_000.00,
        quantity: 6,
     };
     let toshiba = Laptop {
        brand:String::from("Toshiba"),
        price:550_000.00,
        quantity: 10,
     };
     let dell = Laptop {
        brand:String::from("Dell"),
        price:850_000.00,
        quantity: 4,
     };
let total_cost = hp.total_cost(3) + ibm.total_cost(3) + toshiba.total_cost(3) + dell.total_cost(3);
println!("The total costt for purchasing 3 laptops from each brand is: #{}",total_cost);  
 }