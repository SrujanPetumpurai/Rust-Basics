enum Direction {
    North,
    East,
    South,
    West
}

fn move_around(direction:Direction)->String{
    //fn takes a argument of type Direciton
    //Moves Character
     String::from("Moved the character")
}
fn main(){
    let direction_1 = Direction::North;
    let direction_2 = Direction::South;
   println!("{}",move_around(direction_1)); 

}

