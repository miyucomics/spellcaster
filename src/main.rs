#![warn(clippy::pedantic)]

mod trie;

use std::{cmp::Reverse, collections::HashSet};
use trie::Trie;

static LETTER_VALUES: [u32; 26] = [
    1, 4, 5, 3, 1, 5, 3, 4, 1, 7, 3, 3, 4, 2, 1, 4, 8, 2, 2, 2, 4, 5, 5, 7, 4, 8,
];

struct Solution {
    word: Vec<u8>,
    breadcrumbs: Vec<u8>,
}

struct TraverseContext<'a> {
    trie: &'a Trie,
    board: &'a Vec<u8>,
    neighbors: &'a Vec<Vec<u8>>,

    bitmask: u32,
    solutions: &'a mut Vec<Solution>,
    word_accumulator: &'a mut Vec<u8>,
    breadcrumb_accumulator: &'a mut Vec<u8>,
}

fn score_solution(
    solution: &Solution,
    double_letter_tile: Option<u8>,
    triple_letter_tile: Option<u8>,
    double_word_tile: Option<u8>,
) -> u32 {
    let mut score: u32 = 0;

    for (&ch, &pos) in solution.word.iter().zip(&solution.breadcrumbs) {
        let mut multiplier = 1;

        if Some(pos) == double_letter_tile {
            multiplier = 2;
        } else if Some(pos) == triple_letter_tile {
            multiplier = 3;
        }

        score += LETTER_VALUES[ch as usize] * multiplier;
    }

    if let Some(dw) = double_word_tile
        && solution.breadcrumbs.contains(&dw)
    {
        score *= 2;
    }

    if solution.word.len() >= 6 {
        score += 10;
    }

    score
}

fn traverse(
    trie_index: usize,
    board_position: u8,
    swaps_remaining: u8,
    context: &mut TraverseContext,
) {
    let current_letter = context.board[board_position as usize];
    let current_node = &context.trie.nodes[trie_index];

    for (letter_needed, &child_option) in current_node.children.iter().enumerate() {
        let letter_needed = u8::try_from(letter_needed).unwrap();
        if let Some(child_trie_index) = child_option {
            let cost = u8::from(current_letter != letter_needed);
            if swaps_remaining < cost {
                continue;
            }

            context.word_accumulator.push(letter_needed);
            context.breadcrumb_accumulator.push(board_position);

            if context.trie.nodes[child_trie_index as usize].is_terminal {
                context.solutions.push(Solution {
                    word: context.word_accumulator.clone(),
                    breadcrumbs: context.breadcrumb_accumulator.clone(),
                });
            }

            context.bitmask |= 1 << board_position;
            for neighbor in context.neighbors.get(board_position as usize).unwrap() {
                if context.bitmask & (1 << neighbor) == 0 {
                    traverse(
                        child_trie_index as usize,
                        *neighbor,
                        swaps_remaining - cost,
                        context,
                    );
                }
            }
            context.bitmask &= !(1 << board_position);

            context.word_accumulator.pop();
            context.breadcrumb_accumulator.pop();
        }
    }
}

fn solve_board(
    board: &Vec<u8>,
    dl: Option<u8>,
    tl: Option<u8>,
    dw: Option<u8>,
    swaps: u8,
    trie: &Trie,
    neighbors: &Vec<Vec<u8>>,
) {
    let mut solutions = Vec::new();
    let mut word_accumulator = Vec::with_capacity(25);
    let mut breadcrumb_accumulator = Vec::with_capacity(25);

    let mut context = TraverseContext {
        trie,
        board,
        neighbors,
        bitmask: 0,
        solutions: &mut solutions,
        word_accumulator: &mut word_accumulator,
        breadcrumb_accumulator: &mut breadcrumb_accumulator,
    };

    for pos in 0..25 {
        traverse(0, pos, swaps, &mut context);
    }

    let mut seen = HashSet::new();
    solutions.retain(|s| seen.insert(s.word.clone()));
    solutions.sort_by_key(|s| Reverse(score_solution(s, dl, tl, dw)));

    for solution in solutions.iter().take(10) {
        let score = score_solution(solution, dl, tl, dw);
        println!(
            "{:<13} {score:<5} {:?}",
            solution
                .word
                .iter()
                .map(|&byte| (byte + b'a') as char)
                .collect::<String>(),
            solution.breadcrumbs,
        );
    }
    println!();
}

fn main() {
    let dictionary = include_str!("dictionary.txt");
    let mut trie = Trie::new();
    dictionary
        .split_ascii_whitespace()
        .for_each(|word| trie.insert(word));

    let neighbor_cache: Vec<Vec<u8>> = (0..25)
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

    let board = "tssoknodovampenstxegzwtyi"
        .bytes()
        .map(|byte| byte - b'a')
        .collect();

    solve_board(&board, Some(14), None, None, 0, &trie, &neighbor_cache);
    solve_board(&board, Some(14), None, None, 1, &trie, &neighbor_cache);
    solve_board(&board, Some(14), None, None, 2, &trie, &neighbor_cache);
}
