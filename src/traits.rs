//Traits are like interfacfaces in other languages


pub trait Summarise{//First letter should be Capital
    fn summary(&self)->String{
       return String::from("Hellow hody");//Default implementation
    }
 }
 
 struct User{
    name:String,
    age:i32
 }
 
 
 impl Summarise for User{
    fn summary(&self)->String{
        return format!("{}:{}",self.name,self.age);//format! returns a string instead of printing it like println!()
    }
 }
 
 //impl Summarise for User{} //This returns the default behaviour of the trait 
 
 fn main(){
    let user = User{
       name:String::from("Srujan"),
       age:22
    };
    //pub fn another(item:&impl Summarise){ //reference to that which has Summarise implemented
 //println!("{}",item.summary())
    //}
    let output = user.summary();
    println!("{}",output);
 }