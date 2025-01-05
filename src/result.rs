//Result is another type of enum returns either "OK" or an "Error"
use std::fs;

fn main(){
    let greeting_file_result = fs::read_to_string("Hello");
    //fs is a std library provided by rust, read_to_string is a function in fs which returns a Result
    //
    match greeting_file_result{
        Ok(file_content)=>{
            println!{"File succesfully read:{}",file_content};
        },
        Err(error)=>{
            println!{"File could not be read:{}",error};
        }
    }
}

//If the contents of the file are read(Ok output)=>File succesfully read:"Hi, this is Hello file in the root directory"
//If the file couldn't be read(Err output)=>File could not be read:The system cannot find the file specified. (os error 2)