fn main(){
    println!("{}",fb(4));
    fn fb(num:i32)->i32{
        let mut first = 0;
        let mut second = 1;
        if num ==0 {
            return 0;
        }else if num==1 {
            return 1;
        }
        else if num>1 {
            for i in 0..num-2{
                let temp = second;
                second= first + second;
                first = temp;
            }
            return second;
        }
        else{
            -1
        }
    }
}