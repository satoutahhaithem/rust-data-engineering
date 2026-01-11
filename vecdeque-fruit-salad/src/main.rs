/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.

Enhanced version with three challenges:
1. Allow user to add fruits to either end of the queue
2. Use choose() to select a random fruit from the salad
3. Remove fruits from either end and display the result
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::io::{self, Write};

fn print_fruit_salad(fruit: &VecDeque<String>) {
    println!("\n🥗 Current Fruit Salad:");
    if fruit.is_empty() {
        println!("   (empty)");
    } else {
        for (i, item) in fruit.iter().enumerate() {
            if i != fruit.len() - 1 {
                print!("   {}, ", item);
            } else {
                println!("{}", item);
            }
        }
    }
}

fn add_fruit(fruit: &mut VecDeque<String>) {
    println!("\n--- Add Fruit to Salad ---");
    print!("Enter fruit name: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let fruit_name = input.trim().to_string();

    if fruit_name.is_empty() {
        println!("Fruit name cannot be empty!");
        return;
    }

    println!("Add to (1) Front or (2) Back? (Enter 1 or 2): ");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            fruit.push_front(fruit_name.clone());
            println!("✓ Added '{}' to the front!", fruit_name);
        }
        "2" => {
            fruit.push_back(fruit_name.clone());
            println!("✓ Added '{}' to the back!", fruit_name);
        }
        _ => println!("Invalid choice! Please enter 1 or 2."),
    }

    print_fruit_salad(fruit);
}

fn remove_fruit(fruit: &mut VecDeque<String>) {
    println!("\n--- Remove Fruit from Salad ---");

    if fruit.is_empty() {
        println!("Cannot remove: Salad is empty!");
        return;
    }

    println!("Remove from (1) Front or (2) Back? (Enter 1 or 2): ");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            if let Some(removed) = fruit.pop_front() {
                println!("✓ Removed '{}' from the front!", removed);
            }
        }
        "2" => {
            if let Some(removed) = fruit.pop_back() {
                println!("✓ Removed '{}' from the back!", removed);
            }
        }
        _ => println!("Invalid choice! Please enter 1 or 2."),
    }

    print_fruit_salad(fruit);
}

fn pick_random_fruit(fruit: &VecDeque<String>) {
    println!("\n--- Pick a Random Fruit ---");

    if fruit.is_empty() {
        println!("Cannot pick: Salad is empty!");
        return;
    }

    let mut rng = thread_rng();
    if let Some(random_fruit) = fruit.iter().collect::<Vec<_>>().choose(&mut rng) {
        println!("🎲 Randomly selected: '{}'", random_fruit);
    }
}

fn main() {
    let mut fruit: VecDeque<String> = VecDeque::new();

    // Initialize with some fruits
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    println!("=== VecDeque Fruit Salad Challenge ===");
    println!("Initial fruits added to the back:");
    print_fruit_salad(&fruit);

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<String> = fruit_vec.into_iter().collect();

    println!("\n✓ Fruits shuffled!");
    print_fruit_salad(&fruit);

    // Add fruits to both ends of the queue after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    println!("\n✓ Added Pomegranate to front, Fig and Cherry to back!");
    print_fruit_salad(&fruit);

    // Interactive menu for challenges
    loop {
        println!("\n=== Menu ===");
        println!("1. Add a fruit to either end");
        println!("2. Remove a fruit from either end");
        println!("3. Pick a random fruit");
        println!("4. Exit");
        print!("\nChoice (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => add_fruit(&mut fruit),
            "2" => remove_fruit(&mut fruit),
            "3" => pick_random_fruit(&fruit),
            "4" => {
                println!("\n👋 Final Fruit Salad:");
                print_fruit_salad(&fruit);
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice! Please enter 1-4."),
        }
    }
}
