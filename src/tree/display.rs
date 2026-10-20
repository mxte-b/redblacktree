use colored::Colorize;
use std::{fmt::{Display, Write}, format};

use crate::tree::{RbTree, enums::RbColor};

impl Display for RbTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.height() as u32;

        let Some(digits) = self.max().map(|e| e.key.ilog10() as i32 + 1) else {
            return f.write_str("Empty tree");
        };

        // Correction terms
        let correction_p = (digits / 2) - 1;
        let correction_s = (digits as f32 / 2_f32).ceil() as i32 - 2;

        // Running variables
        let mut growth: i32;
        let mut padding: i32;
        let mut skip = 0_i32;

        if height > 1 {
            growth = (digits + 2) * 2_i32.pow(height - 2) - 3;
            padding = growth - correction_p;
        }
        else {
            growth = 0;
            padding = 0;
        }

        // Indicies to check
        let mut indicies: Vec<Option<usize>> = vec![self.root];
        
        // We traverse the tree level-by-level, starting on the root level,
        // then we write to the formatter, update the running variables, 
        // and gather the indicies of the next level.
        for l in 0..=height {
            // Add padding if necessary
            if padding > 0 {
                f.write_str(&" ".repeat(padding as usize))?;
            }

            // Write keys
            for x in indicies.iter().enumerate() {
                let key =
                    x.1.map(|i| self.get(i))
                        .map(|n| {
                            let colorized = n.key.to_string();

                            if n.color == RbColor::Red {
                                colorized.red()
                            } else {
                                colorized.black()
                            }
                        })
                        .unwrap_or(" ".repeat(digits as usize).into());

                let skip_count = if l < height - 1 { 
                    skip 
                } else { 
                    if x.0 % 2 == 0 {
                        1
                    }
                    else {
                        3
                    }
                } as usize;

                let formatted = format!(
                    "{:0>digits$}{}", 
                    key, 
                    " ".repeat(skip_count), 
                    digits = digits as usize
                );
                
                f.write_str(formatted.as_str())?;
            }

            f.write_char('\n')?;

            // Gather new indicies
            let mut new_indicies: Vec<Option<usize>> = vec![];
            for idx in indicies.iter() {
                let node = idx.map(|i| self.get(i));

                new_indicies.push(node.and_then(|e| e.left));
                new_indicies.push(node.and_then(|e| e.right));
            }

            indicies = new_indicies;

            // Update running variables
            growth = (growth - 3) / 2;
            skip = padding - correction_s;
            padding = growth - correction_p;
        }

        Ok(())
    }
}
