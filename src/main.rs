use std::{
    error::Error,
    io::{self},
    println,
};

use redblacktree::tree::RbTree;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tree = RbTree::new();

    let mut buf = String::new();
    let mut running = true;

    while running {
        buf.clear();
        io::stdin().read_line(&mut buf)?;

        let trimmed = buf.trim();
        if let Ok(choice) = trimmed.parse::<u32>() {
            tree.insert(choice);

            println!("Tree after insert:\n{tree}");
        } else {
            if trimmed == "q" {
                running = false;
                println!("Quitting.");
            }

            println!("Invalid format.");
        }
    }
    Ok(())
}
