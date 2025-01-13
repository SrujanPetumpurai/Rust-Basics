//Types of Iterations: 1.for i in vals
fn main(){
    //     let vals = v![2,3,4,5,5];
    //     for  value in vals{
    //         println!("{}",value)
    //     }
    // //2.Creating iterator using .iter()
    //     let num = vals.iter();
    //     for value in num{
    //         println!("{}",value)
    //     }
    // //3.Using .next
    //    while let Some(val)= num.next(){ //next returns a Option enum depending upon whether there is next element in the vector.
    //     println!("{}",val)
    //    }
    // //4. Creating IterMut
    //     let mut num2 = vec![2,3,4,5];//A mutable vector
    //     let iter_mut = num2.iter_mut();
    //     for value in num2{
    //         value = value*1;
    //     }
    //    //5.Creating iterator using into_iter
    //    //when you use for value in vals, without creating iter ,It does the same thing as creating and using the into_iter();
    //    let iter = vals.into_iter();//iter takes the ownership of the collection
    //    for value in iter{
    //     println!("{}",value)
    //    }
       //function that filters odd values and doubles them.
       let values2 = vec![1,2,3,4,5,6,7,8];
       let alter_vec = filter_double(values2);
       fn filter_double(values:Vec<i32>)->Vec<i32>{
        let iter = values.into_iter();
        let array = iter.filter(|v| v%2==1).map(|v| v*2);
        let array2:Vec<i32> = array.collect();
        return array2;
       }
       println!("{:?}",alter_vec);
    }
    