use std::{collections::HashMap};

fn main() {
    // let mut v: Vec<i32> = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v2 = vec![1,2,3,4,5];

    // // Reading Elements
    // let third: &i32 = &v2[2];

    // println!("The third element is {}",third);

    // let third:Option<&i32> = v.get(2);

    // match third {
    //     Some(third) => println!("The third element is {}",third),
    //     None => println!("There is no third element"),
    // }

    // // Mutable and Immutable refs
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {}", first);

    // // Iterating 
    // let mut v = vec![100, 32, 57];
    
    // for i in &mut v {
    //     *i += 50;
    // }
    // for i in &v {
    //     println!("{}", i);
    // } 

    // // Using Enums to Store Multiple Types
    
    // enum SpreadSheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    
    // let row = vec![
    //     SpreadSheetCell::Int(3),
    //     SpreadSheetCell::Text(String::from("blue")),
    //     SpreadSheetCell::Float(10.12)
    // ];

    // // Strings
    // let mut s = String::new();

    // let data = "initial contents";

    // let s = data.to_string();

    // let s = "initial contents".to_string();

    // let s = String::from("initial contents");

    // let hello = String::from("درود");

    // // Updating Strings
    // let mut s = String::from("foo");
    // s.push_str("bar");

    // // Concatenation

    // let s1 = String::from("Hello");
    // let s2 = String::from("world");

    // let s3 = s1 + &s2;

    // Format with format! macro
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{}-{}-{}", s1, s2, s3);

    // // Indexing Strings
    // let s1 = String::from("hello");
    // let h = s1[0];

    // // Slicing Strings
    // let hello = "Здравствуйте";
    // let s = &hello[0..4];

    // // Iterating over strings
    // for c in "Зд".chars() {
    //     println!("{}", c);
    // }

    // for b in "Зд".bytes() {
    //     println!("{}", b);
    // }

    // // HashMaps
    // // Create a New Hash Map
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // // Accessing Values 
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);

    // // Updating a Hash Map
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);

    // println!("{:?}", scores);

    // use std::collections::HashMap;

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    // Practices

    // // Median and Mode

    // let mut a = vec![1,4,2,6,4,3,0,9];

    // a.sort();

    // println!("Median = {}",a[(a.len()/2)-1]);

    // let mut mod_map = HashMap::new();

    // for num in &a {
    //     let count = mod_map.entry(num).or_insert(0);
    //     *count +=1;
    // }

    // let mut mode = &a[0];
    // for (key,value) in & mod_map {
    //     if mode<value {
    //         mode = key
    //     }
    // }

    // println!("Mode = {mode}");

    // Pig latin

    // let phrase = "Some phrase about something";

    // let vowels = vec!['a','e','i','o','u'];

    // let mut new_phrase = String::new();

    // for word in phrase.split_whitespace() {
    //     let mut ending = String::from("hay");
    //     let mut index = 0;
    //     let mut new_word = String::new();
    //     for c in word.chars(){
    //         if index == 0 && !vowels.contains(&c){
    //             ending = format!("{}ay",c.to_string());

    //         } else {
    //             new_word = new_word + &String::from(c.to_string());
    //         }  
    //         index+=1;          
    //     }

    //     new_phrase = new_phrase + &format!("{}-{} ",new_word,ending);
    // }
    // println!("{}",new_phrase);

    // // Dep and employees
    // let mut my_company = Company::new(String::from("CS"));

    // my_company.add_employee(String::from("IT"), String::from("Dev"));

    // let my_employees = my_company.get_department(String::from("IT"));

    // match my_employees {
    //     Some(d)=>println!("employees in IT dep : {:?}", d),
    //     None=>println!("No employees")
    // }






}

// struct Company {
//     name:String,
//     employees:HashMap<String,Vec<String>>
// }
// impl Company {
//     fn new(name:String)->Self{
//         Self {
//             name,
//             employees:HashMap::new()
//         }
//     }
//     fn add_employee(&mut self,dep:String,employee:String){
//         let found_dep = self.employees.entry(dep).or_insert(vec![]);
//         found_dep.sort();
//         match found_dep.binary_search(&employee) {
//             Err(_)=>found_dep.push(employee),
//             _=>println!("Found Employee")
//         }
//     }

//     fn get_department(&self,dep:String)->Option<&Vec<String>>{
//         self.employees.get(&dep)
//     }
// }