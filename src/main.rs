use std::io::{self, Write};
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input miner address");
    io::stdin().read_line(&mut miner_addr).unwrap();
    println!("Difficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).unwrap();
    let diff = difficulty.trim().parse::<u32>().unwrap_or(1);

    println!("Generating genesis block");

    let mut chain = blockchain::Chain::new(miner_addr, diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sender).unwrap();
                print!("enter receiver address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut receiver).unwrap();
                print!("Enter amount: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount).unwrap();

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed "),
                }
                println!("{:?}", chain);
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).unwrap();
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed Update Difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).unwrap();
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            _ => {
                println!("Invalid option, please retry");
                continue;
            }
        }
    }
}
