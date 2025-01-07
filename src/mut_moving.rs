fn create_string(){
    let mut s1 = String::from("Hello,");
    println!("{}",s1);
    print_s2(&mut s1);//Now both s1 and s2 point at the same value(Note:s1 is still the only owner of the value)
    println!("{}",s1)

}

fn print_s2(s2:&mut String){
    s2.push_str("World");
    println!("{}",s2);
}

fn main(){
   create_string();
}