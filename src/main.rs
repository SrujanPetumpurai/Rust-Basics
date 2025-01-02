fn main() {
    println!("{}",is_even(11));
    fn is_even(x:i32)->bool{
        if x%2 == 0{
            return true;
        }
        return false;
    }
}
