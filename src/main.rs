#![warn(clippy::pedantic)]

mod trie;

use std::{cmp::Reverse, collections::HashSet};
use trie::Trie;

static LETTER_VALUES: [u8; 26] = [
    1, 4, 5, 3, 1, 5, 3, 4, 1, 7, 3, 3, 4, 2, 1, 4, 8, 2, 2, 2, 4, 5, 5, 7, 4, 8,
];

struct Solution {
    word: Vec<u8>,
    path: Vec<u8>,
    score: u8,
}

impl Solution {
    fn get_word(&self) -> String {
        self.word
            .iter()
            .map(|&byte| (byte + b'a') as char)
            .collect::<String>()
    }
}

struct Context<'a> {
    trie: &'a Trie,
    board: &'a [u8; 25],
    neighbors: &'a Vec<Vec<u8>>,

    board_mask: u32,

    solutions: &'a mut Vec<Solution>,
    word_builder: &'a mut Vec<u8>,
    trail_builder: &'a mut Vec<u8>,

    double_letter: Option<u8>,
    triple_letter: Option<u8>,
    double_word: Option<u8>,
}

fn traverse(
    trie_index: usize,
    board_position: u8,
    swaps: u8,
    score: u8,
    double: bool,
    context: &mut Context,
) {
    let current_letter = context.board[board_position as usize];
    let current_node = &context.trie.nodes[trie_index];

    for (letter_needed, &child_option) in current_node.children.iter().enumerate() {
        let letter_needed = u8::try_from(letter_needed).unwrap();
        if let Some(child_index) = child_option {
            let cost = u8::from(current_letter != letter_needed);
            if swaps < cost {
                continue;
            }

            let multiplier = if Some(board_position) == context.double_letter {
                2
            } else if Some(board_position) == context.triple_letter {
                3
            } else {
                1
            };

            let next_score = score + LETTER_VALUES[letter_needed as usize] * multiplier;
            let has_encountered_double = double || Some(board_position) == context.double_word;

            context.word_builder.push(letter_needed);
            context.trail_builder.push(board_position);

            if context.trie.nodes[child_index as usize].is_terminal {
                let mut final_score = next_score;
                if has_encountered_double {
                    final_score *= 2;
                }
                if context.word_builder.len() >= 6 {
                    final_score += 10;
                }

                context.solutions.push(Solution {
                    word: context.word_builder.clone(),
                    path: context.trail_builder.clone(),
                    score: final_score,
                });
            }

            context.board_mask |= 1 << board_position;
            for neighbor in context.neighbors.get(board_position as usize).unwrap() {
                if context.board_mask & (1 << neighbor) == 0 {
                    traverse(
                        child_index as usize,
                        *neighbor,
                        swaps - cost,
                        next_score,
                        has_encountered_double,
                        context,
                    );
                }
            }
            context.board_mask &= !(1 << board_position);

            context.word_builder.pop();
            context.trail_builder.pop();
        }
    }
}

fn solve_board(
    board: &[u8; 25],
    trie: &Trie,
    neighbors: &Vec<Vec<u8>>,

    swaps: u8,

    double_letter: Option<u8>,
    triple_letter: Option<u8>,
    double_word: Option<u8>,
) {
    let mut solutions = Vec::new();
    let mut word_builder = Vec::with_capacity(25);
    let mut trail_builder = Vec::with_capacity(25);

    let mut context = Context {
        trie,
        board,
        neighbors,
        board_mask: 0,

        solutions: &mut solutions,
        word_builder: &mut word_builder,
        trail_builder: &mut trail_builder,

        double_letter,
        triple_letter,
        double_word,
    };

    for pos in 0..25 {
        traverse(0, pos, swaps, 0, false, &mut context);
    }

    let mut seen = HashSet::new();
    solutions.retain(|s| seen.insert(s.word.clone()));
    solutions.sort_by_key(|s| Reverse(s.score));

    for word in solutions.iter().take(10) {
        println!("{:<13} {:<5} {:?}", &word.get_word(), word.score, word.path);
    }

    println!();
}

fn main() {
    let dictionary = include_str!("dictionary.txt");
    let mut trie = Trie::new();
    dictionary
        .split_ascii_whitespace()
        .for_each(|word| trie.insert(word));

    let neighbors: Vec<Vec<u8>> = (0..25)
        .map(|pos| {
            let x = pos % 5;
            let y = pos / 5;

            let mut neighbors = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx: i8 = x + dx;
                    let ny: i8 = y + dy;
                    if (0..5).contains(&nx) && (0..5).contains(&ny) {
                        neighbors.push(u8::try_from(ny * 5 + nx).unwrap());
                    }
                }
            }
            neighbors
        })
        .collect();

    let board = b"tssoknodovampenstxegzwtyi".map(|byte| byte - b'a');
    solve_board(&board, &trie, &neighbors, 0, Some(14), None, None);
    solve_board(&board, &trie, &neighbors, 1, Some(14), None, None);
    solve_board(&board, &trie, &neighbors, 2, Some(14), None, None);
}
