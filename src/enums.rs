
enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run(){
    let _avatar1 = Movement::Left;
    let _avatar2 = Movement::Up;
    let _avatar3 = Movement::Right;
    let _avatar4 = Movement::Down;

    move_avatar(_avatar1);
    move_avatar(_avatar2);
    move_avatar(_avatar3);
    move_avatar(_avatar4);
}