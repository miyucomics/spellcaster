mod trie;
use std::collections::HashSet;
use trie::{Trie, TrieNode};

struct Solution {
    word: String,
    breadcrumbs: Vec<u8>,
}

struct TraverseContext<'a> {
    board: &'a Vec<u8>,
    bitmask: u32,
    solutions: &'a mut Vec<Solution>,
    word_accumulator: &'a mut String,
    breadcrumb_accumulator: &'a mut Vec<u8>,
}

static LETTER_VALUES: [i8; 26] = [
    1, 4, 5, 3, 1, 5, 3, 4, 1, 7, 3, 3, 4, 2, 1, 4, 8, 2, 2, 2, 4, 5, 5, 7, 4, 8,
];

fn score_solution(
    solution: &Solution,
    double_letter_tile: Option<u8>,
    triple_letter_tile: Option<u8>,
    double_word_tile: Option<u8>,
) -> i32 {
    let mut score: i32 = 0;

    for (ch, &pos) in solution.word.chars().zip(&solution.breadcrumbs) {
        let mut multiplier = 1;

        if Some(pos) == double_letter_tile {
            multiplier = 2;
        } else if Some(pos) == triple_letter_tile {
            multiplier = 3;
        }

        score += LETTER_VALUES[ch as usize - 'a' as usize] as i32 * multiplier;
    }

    if let Some(dw) = double_word_tile {
        if solution.breadcrumbs.iter().any(|&p| p as u8 == dw) {
            score *= 2;
        }
    }

    if solution.word.len() >= 6 {
        score += 10;
    }

    score
}

fn traverse(
    node: &TrieNode,
    pos: u8,
    remaining_errors: u8,
    context: &mut TraverseContext,
    neighbors_cache: &Vec<Vec<u8>>,
) {
    let current_letter = context.board[pos as usize];

    for (&path, child_node) in &node.children {
        let cost = if current_letter == path { 0 } else { 1 };
        if remaining_errors < cost {
            continue;
        }

        context.word_accumulator.push(path as char);
        context.breadcrumb_accumulator.push(pos);

        if child_node.is_terminal {
            context.solutions.push(Solution {
                word: context.word_accumulator.clone(),
                breadcrumbs: context.breadcrumb_accumulator.clone(),
            });
        }

        context.bitmask |= 1 << pos;
        for neighbor in neighbors_cache.get(pos as usize).unwrap() {
            if (context.bitmask >> neighbor) & 1 == 0 {
                traverse(
                    child_node,
                    *neighbor,
                    remaining_errors - cost,
                    context,
                    neighbors_cache,
                );
            }
        }
        context.bitmask &= !(1 << pos);

        context.word_accumulator.pop();
        context.breadcrumb_accumulator.pop();
    }
}

fn solve_board(
    board: &String,
    dl: Option<u8>,
    tl: Option<u8>,
    dw: Option<u8>,
    swaps: u8,
    trie: &Trie,
    neighbors_cache: &Vec<Vec<u8>>,
) {
    let mut context = TraverseContext {
        board: &mut board.as_bytes().to_vec(),
        bitmask: 0,
        solutions: &mut Vec::new(),
        word_accumulator: &mut String::new(),
        breadcrumb_accumulator: &mut Vec::new(),
    };

    for pos in 0..25 {
        traverse(&trie.root, pos, swaps, &mut context, &neighbors_cache);
    }

    let mut seen = HashSet::new();
    context.solutions.retain(|s| seen.insert(s.word.clone()));

    context
        .solutions
        .sort_by_key(|s| -score_solution(s, dl, tl, dw));

    for solution in context.solutions.iter().take(5) {
        let score = score_solution(solution, dl, tl, dw);
        println!("{:<10} {:?} {}", solution.word, solution.breadcrumbs, score);
    }
    println!();
}

fn main() {
    let dictionary = include_str!("dictionary.txt");
    let mut trie = Trie::new();
    dictionary
        .split_ascii_whitespace()
        .for_each(|word| trie.insert(word));

    // this is a surprisingly expensive calculation because of the double loop, conditional, and bounds
    // we'll precompute it just once
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
                        neighbors.push((ny * 5 + nx) as u8);
                    }
                }
            }
            neighbors
        })
        .collect();

    let board = String::from("tssoknodovampenstxegzwtyi");
    solve_board(&board, Some(14), None, None, 0, &trie, &neighbor_cache);
    solve_board(&board, Some(14), None, None, 1, &trie, &neighbor_cache);
    solve_board(&board, Some(14), None, None, 2, &trie, &neighbor_cache);
}
