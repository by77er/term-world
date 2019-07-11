use noise::{Seedable, NoiseFn, OpenSimplex};
use colored::*;
use rand::Rng;

fn main() {
    let mut simplex = OpenSimplex::new();
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();
    simplex = simplex.set_seed(seed);
    let mut buffer: [[char;40]; 40] = [[' '; 40]; 40];
    for row in 0..40 {
        for col in 0..40 {
            let mut gen = simplex.get([row as f64 / 9.0, col as f64 / 9.0]);
            let subgen = simplex.get([row as f64 / 7.0, col as f64 / 7.0]);
            gen *= 1.3;
            gen += (subgen / 3.0).abs();
            gen += 0.1;
            let gen2 = simplex.get([row as f64 / 5.0, col as f64 / 5.0]);
            if gen > 0.75 {
                buffer[row][col] = 'M';
            } else if gen > 0.65 {
                buffer[row][col] = 'h';
            } else if gen > 0.0 {
                if gen2 > 0.0 {
                    buffer[row][col] = '#';
                } else {
                    buffer[row][col] = 'T';
                }
            } else if gen > -0.06{
                buffer[row][col] = 'S';
            } else {
                buffer[row][col] = ' ';
            }
        }
    }
    for row in 0..40 {
        for col in 0..40 {
            match buffer[row][col] {
                '#' => print!("{0}{0}", " ".on_bright_green()),
                'T' => print!("{0}{0}", "#".green().on_bright_green()),
                'S' => print!("{0}{0}", ".".white().on_yellow().dimmed()),
                'h' => print!("{0}{0}", "#".bright_green().on_bright_black()),
                'M' => print!("{0}{0}", ".".bright_green().on_bright_black()),
                ' ' => print!("{0}{0}", "w".cyan().on_blue()),
                _ => ()
            }
        }
        println!();
    }
}
