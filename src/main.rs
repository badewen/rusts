use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("guess the number game");

  let secret_number = rand::thread_rng().gen_range(1..101);
  
  for _i in 1..4 {
    println!("type the number");
    
    let mut ans = String::new();
    
    io::stdin()
      .read_line(&mut ans)
      .expect("failed to read line please try again");
      
    let ans: u8 = match ans.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match ans.cmp(&secret_number){
      Ordering::Less => println!("your number is too low"),
      Ordering::Greater => println!("your number is too big"),
      Ordering::Equal => {
        println!("you are rite");
        break;
      }
    }
  }
}
