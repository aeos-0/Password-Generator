use rand::{thread_rng, Rng};
//use rand::seq::SliceRandom;

fn main() {
    let mut rng = thread_rng();
    let mut list:Vec<&str>;
    list = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    
    println!("Unidentified values will be selected for you automatically");
    println!();
    println!("Enter the amount of characters your password should have ");
    let mut char_num = String::new();
    std::io::stdin()
        .read_line(&mut char_num)
        .expect("Failed to read line");
    char_num = char_num.trim().to_string();

        //Make char num an int
        let length: i32;
        match char_num.parse::<i32>() {
            Ok(number) => {
                length = number;
            }
            Err(_) => {
                println!("Invalid character number input, the default will be 12 characters");
                length = 12;
            }
        }

    println!("Would you like to include special characters? Please input 'y' or 'n'");
    let mut special = String::new(); 
    std::io::stdin()
        .read_line(&mut special)
        .expect("Failed to read line");
    special = special.trim().to_string();
    
    if special == "y" || special == "Y" {
        //Add special characters to the list
        println!("Special characters will be added");
        let mut special_char = vec!["!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "-", "+", "=",
        "~", "`", "{", "}", "[", "]", "|", "\\", ":", ";", "'", "\"", "<", ">", ".", "?", "/"];
        list.append(&mut special_char);     
    } else {
        println!("Special characters will not be added");
    }

    //Select random characters and print to the screen
    let mut password = String::new();
    //list.shuffle(&mut rng); //Shuffle random list
    for _ in 0..length {
        let num = rng.gen_range(0..list.len()); //Picks random character in the list

        password += list[num] //Adds character to the list
    }

    println!("The new generated paassword is:");
    println!("{}", password);
}
