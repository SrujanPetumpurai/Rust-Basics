struct User{
    first_name:String,
    last_name:String,
    age:i32,
}

struct Rect{
    length:i32,
    bredth:i32
}

//Implementing to Rect struct.
//self is like this in js class.

impl Rect{
    fn area(&self)->i32{
        self.length*self.bredth
    }
}

fn main(){

    let user1 = User{
        first_name:String::from("SRUJAN"),
        last_name:String::from("PETUMPURI") ,
        age:22
    };
    let react1 = Rect{
        length:10,
        bredth:4,
    };
    println!("User1 first name is: {}",user1.first_name);
    println!("Area of the reactangle is :{}",react1.area())
}