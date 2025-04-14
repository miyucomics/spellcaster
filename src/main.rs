mod trie;
use std::collections::HashSet;

use trie::{Trie, TrieNode};

struct Solution {
    word: String,
    breadcrumbs: Vec<i8>,
}

struct TraverseContext<'a> {
    board: &'a mut Vec<Option<char>>,
    solutions: &'a mut Vec<Solution>,
    word_accumulator: &'a mut String,
    breadcrumb_accumulator: &'a mut Vec<i8>,
}

static LETTER_VALUES: [i8; 26] = [
    1, 4, 5, 3, 1, 5, 3, 4, 1, 7, 3, 3, 4, 2, 1, 4, 8, 2, 2, 2, 4, 5, 5, 7, 4, 8,
];

fn score_solution(
    solution: &Solution,
    double_letter_tile: i8,
    triple_letter_tile: i8,
    double_word_tile: i8,
) -> i32 {
    let mut score: i32 = 0;

    for (ch, &pos) in solution.word.chars().zip(&solution.breadcrumbs) {
        let mut multiplier = 1;
        if pos == double_letter_tile {
            multiplier = 2;
        } else if pos == triple_letter_tile {
            multiplier = 3;
        }
        score += LETTER_VALUES[ch as usize - 'a' as usize] as i32 * multiplier;
    }

    if solution.breadcrumbs.contains(&double_word_tile) {
        score *= 2;
    }

    if solution.word.len() >= 6 {
        score += 10;
    }

    score
}

fn traverse(
    node: &TrieNode,
    pos: i8,
    remaining_errors: u8,
    context: &mut TraverseContext,
    neighbors_cache: &Vec<Vec<i8>>,
) {
    // this should never be None because we test before traversing onto this tile
    // and the only other time traverse is called is to begin a search on a given tile, where all tiles should be unused
    let current_letter = context.board[pos as usize].unwrap();

    for (&ch, child_node) in &node.children {
        let cost = if current_letter == ch { 0 } else { 1 };
        if remaining_errors < cost {
            continue;
        }

        context.word_accumulator.push(ch);
        context.breadcrumb_accumulator.push(pos);

        if child_node.is_terminal {
            context.solutions.push(Solution {
                word: context.word_accumulator.clone(),
                breadcrumbs: context.breadcrumb_accumulator.clone(),
            });
        }

        context.board[pos as usize] = None;
        for neighbor in neighbors_cache.get(pos as usize).unwrap() {
            if context.board[*neighbor as usize].is_some() {
                traverse(
                    child_node,
                    *neighbor,
                    remaining_errors - cost,
                    context,
                    neighbors_cache,
                );
            }
        }
        context.board[pos as usize] = Some(current_letter);

        context.word_accumulator.pop();
        context.breadcrumb_accumulator.pop();
    }
}

fn get_location(label: &str) -> i8 {
    println!("{}", label);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}

fn main() {
    let dictionary = include_str!("dictionary.txt");
    let mut trie = Trie::new();
    dictionary
        .split_ascii_whitespace()
        .for_each(|word| trie.insert(word));

    // this is a surprisingly expensive calculation because of the double loop, conditional, and bounds
    // we'll precompute it just once
    let neighbor_cache: Vec<Vec<i8>> = (0..25)
        .map(|pos| {
            let x = pos % 5;
            let y = pos / 5;
            let mut neighbors = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if (0..5).contains(&nx) && (0..5).contains(&ny) {
                        neighbors.push(ny * 5 + nx);
                    }
                }
            }
            neighbors
        })
        .collect();

    loop {
        let mut board_input = String::new();
        println!("Board:");
        std::io::stdin()
            .read_line(&mut board_input)
            .expect("Failed to read line");
        let board_input = board_input.trim().to_lowercase();

        if board_input.len() != 25 {
            println!("Board must be exactly 25 letters.");
            continue;
        }

        let dl = get_location("Double letter location:");
        let tl = get_location("Triple letter location:");
        let dw = get_location("Double word location:");

        println!("Swaps allowed:");
        let allowed_errors: u8 = {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().parse().unwrap_or(0)
        };

        let mut context = TraverseContext {
            board: &mut board_input.chars().map(Some).collect(),
            solutions: &mut Vec::new(),
            word_accumulator: &mut String::new(),
            breadcrumb_accumulator: &mut Vec::new(),
        };

        for pos in 0..25 {
            traverse(
                &trie.root,
                pos,
                allowed_errors,
                &mut context,
                &neighbor_cache,
            );
        }

        let mut seen = HashSet::new();
        context.solutions.retain(|s| seen.insert(s.word.clone()));

        context
            .solutions
            .sort_by_key(|s| -score_solution(s, dl, tl, dw));

        for solution in context.solutions.iter().take(10) {
            let score = score_solution(solution, dl, tl, dw);
            println!("{:<10} {:?} {}", solution.word, solution.breadcrumbs, score);
        }
        println!();
    }
}
