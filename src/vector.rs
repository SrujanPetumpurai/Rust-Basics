fn even_vec(vec:Vec<i32>)->Vec<i32>{
    let mut vect1 = Vec::new();
    for val in vec{
        if val%2 ==0{
            vect1.push(val);
        }
    }
    return vect1;
}

fn main(){
    let mut v1 = Vec::new();//one way to create a vector
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("{:?}",v1);
    let mut v2 = vec![1,2,7,4,8];//Easier way to create a vector
    println!("{:?}",v2);
    let ans = even_vec(v2);
    print!("{:?}",ans)
}

