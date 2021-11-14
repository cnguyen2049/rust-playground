use rand::Rng;

fn main() {
    println!("Summation A Random List of Numbers");

    let mut count = 0u32;
    let mut random_list_of_numbers: Vec<i32> = Vec::new(); 
   

    loop {
        
        let secret_number = rand::thread_rng().gen_range(1..101);
        random_list_of_numbers.push(secret_number);
        
        if count == 5 { 
            break;
        }
        count += 1;
    }

    println!("The random list of numbers {:?}",random_list_of_numbers);

    let numbers = random_list_of_numbers.iter();
    let sum = numbers.fold(0, |total, next| total + next);
    println!("Sumed total of the list :{}", sum)
}