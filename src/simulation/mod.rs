use crate::*;


pub fn simulate() {
    println!("Beginning crate solving simulation...");
    let mut abandonded_crate : AbandonedCrate = AbandonedCrate::generate();
    let read_and_parse = || -> (usize, usize, usize, usize) {
        let mut buffer : String = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("error : could not read the user's input");
                return (0, 0, 0, 0);
            },
        };
        buffer = buffer.trim().to_string();
        if buffer.len() == 4 {
            let num1 : usize = buffer[0..1].parse().unwrap_or(0);
            let num2 : usize = buffer[1..2].parse().unwrap_or(0);
            let num3 : usize = buffer[2..3].parse().unwrap_or(0);
            let num4 : usize = buffer[3..4].parse().unwrap_or(0);
            return (num1, num2, num3, num4)
        }
        drop(buffer);
        eprintln!("error : user input did not match format");
        return (0, 0 ,0 ,0)
    };

    let nums = read_and_parse();
    println!("{:?}", nums)


}