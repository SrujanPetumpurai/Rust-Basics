use std::collections::HashMap;
//HashMap Operations:
//Add elements = users.insert
//Access element = users.get(&Keyvalue)
//Remove element = users.remove(&1)
//
fn main(){
    let mut user:HashMap<String,i32> = HashMap::new();
    users.insert(1,String::from("SRUJAN"));
    users.insert(2,String::from("SURAJ"));
    users.insert(3,String::from("NARUJ"));
    for (key,value) in user.iter(){
        println!("{}:{}",key,value);
    }
    

}