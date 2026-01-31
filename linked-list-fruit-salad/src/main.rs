/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.

Enhanced version with three challenges:
1. Allow user to add fruits at any position in the LinkedList
2. Use choose() to select a random fruit from the salad
3. Remove fruits from any position and display the result
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;
use std::io::{self, Write};

fn print_fruit_salad(fruit: &LinkedList<String>) {
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
        println!("   Total fruits: {}", fruit.len());
    }
}

fn add_fruit(fruit: &mut LinkedList<String>) {
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

    println!("\nChoose position:");
    println!("  0 - Front");
    for i in 1..fruit.len() {
        println!("  {} - After position {}", i, i);
    }
    println!("  {} - Back (end)", fruit.len());
    print!("> ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if let Ok(position) = choice.trim().parse::<usize>() {
        if position == 0 {
            fruit.push_front(fruit_name.clone());
            println!("✓ Added '{}' to the front!", fruit_name);
        } else if position >= fruit.len() {
            fruit.push_back(fruit_name.clone());
            println!("✓ Added '{}' to the back!", fruit_name);
        } else {
            // Split the list at the position, insert, and rejoin
            let mut back = fruit.split_off(position);
            fruit.push_back(fruit_name.clone());
            fruit.append(&mut back);
            println!("✓ Added '{}' at position {}!", fruit_name, position);
        }
    } else {
        println!("Invalid position!");
    }

    print_fruit_salad(fruit);
}

fn remove_fruit(fruit: &mut LinkedList<String>) {
    println!("\n--- Remove Fruit from Salad ---");

    if fruit.is_empty() {
        println!("Cannot remove: Salad is empty!");
        return;
    }

    println!("\nChoose position to remove from:");
    println!("  0 - Front");
    for i in 1..fruit.len() - 1 {
        println!("  {} - Position {}", i, i);
    }
    if fruit.len() > 1 {
        println!("  {} - Back (end)", fruit.len() - 1);
    }
    print!("> ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if let Ok(position) = choice.trim().parse::<usize>() {
        if position == 0 {
            if let Some(removed) = fruit.pop_front() {
                println!("✓ Removed '{}' from the front!", removed);
            }
        } else if position >= fruit.len() - 1 {
            if let Some(removed) = fruit.pop_back() {
                println!("✓ Removed '{}' from the back!", removed);
            }
        } else {
            // Split the list at the position, remove first element from back half, and rejoin
            let mut back = fruit.split_off(position);
            if let Some(removed) = back.pop_front() {
                println!("✓ Removed '{}' from position {}!", removed, position);
                fruit.append(&mut back);
            }
        }
    } else {
        println!("Invalid position!");
    }

    print_fruit_salad(fruit);
}

fn pick_random_fruit(fruit: &LinkedList<String>) {
    println!("\n--- Pick a Random Fruit ---");

    if fruit.is_empty() {
        println!("Cannot pick: Salad is empty!");
        return;
    }

    let mut rng = thread_rng();
    // Convert to Vec to use choose() method
    if let Some(random_fruit) = fruit.iter().collect::<Vec<_>>().choose(&mut rng) {
        println!("🎲 Randomly selected: '{}'", random_fruit);
    }
}

fn main() {
    let mut fruit: LinkedList<String> = LinkedList::new();

    // Initialize with some fruits
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    println!("=== LinkedList Fruit Salad Challenge ===");
    println!("Initial fruits added to the back:");
    print_fruit_salad(&fruit);

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList 
    isn't a common operation in practice. I included 
    it in this example to keep the code as similar as possible 
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit_vec.into_iter().collect();

    println!("\n✓ Fruits shuffled!");
    print_fruit_salad(&fruit);

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    println!("\n✓ Added Pomegranate to front, Fig and Cherry to back!");
    print_fruit_salad(&fruit);

    // Interactive menu for challenges
    loop {
        println!("\n=== Menu ===");
        println!("1. Add a fruit at any position");
        println!("2. Remove a fruit from any position");
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
