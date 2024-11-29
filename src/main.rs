
use std::io;
use rand::Rng;
use rand::thread_rng;

fn guess_checker(x : &str,y : &str)->bool{
    return x == y;
}


fn main() {
    
    let guess_list = ["grapes","banana","orange"];
    let mut rng = thread_rng();
    // 0 1 2 
    let index = rng.gen_range(0..guess_list.len());
    let random_fruit  = guess_list[index];
    println!("random fruit = {}",random_fruit);
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input){
            Ok(_)=>{
                let fruit_selected = input.trim().to_lowercase();
                println!("fruit_selected ={}",fruit_selected);
                
                if !guess_list.contains(&fruit_selected.as_str()){
                    println!("Fruit entered does not found");
                    continue;
            }

            if guess_checker(&fruit_selected,random_fruit){
                println!("You are winner");
                break;
            }else{
                println!("Try again");
            }
            
        }   
        Err(error)=>{
            println!("error = {}",error);
        }
        }
    }

}
