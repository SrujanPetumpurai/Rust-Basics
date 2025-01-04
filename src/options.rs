//Option enum lets you return either some value or None
fn find_first_a(s:String)->Option<i32>{
    for(index,character) in s.chars().enumerate(){
      if character =='a'{
          return Some(index as i32);
      }
    }
    return None;
}

fn main(){
    let my_string = String::from("Babba");
    match find_first_a(my_string){
        Some(index)=>println!("The index of the first letter a is:{}",index),
        None =>println!("The letter a is not found in the string")
    };
}