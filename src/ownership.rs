fn main(){
    let s1 = String::from("Hello world");//s1 is the owner of the string who's value is Hello world
    let s2 = s1;// Now s2 is the owner of the stirng.
    //println!("{}",s1); s1 throws an error because, s1 is invalid after its ownership was moved to s2.
    //instead you can do.
    let s3 =String::from("Hi there people");
    let s4: String = s3.clone();//this creates a new value for s4 in the heap which is the excat copy of s3 value.\
    let s5 = &s3;//here s5 is just borrowing the value of s3.
    println!("{}",s3);//Not throwing error.
}

//