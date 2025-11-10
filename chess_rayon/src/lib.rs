use rayon::prelude::*;

fn q1(position_scores: Vec<i32>) -> i32 {
    // TODO: Find the total sum of all scores in parallel, by replacing iter() with par_iter().
    position_scores.par_iter().sum()
}

#[test]
fn test_q1_parallel_evaluation() {
    let scores: Vec<i32> = vec![
        3, 1, 9, -5, 1, 3, 3, -1, -3, 5, 1, 0, -9, 3, -3, 1, 5, -5, 3, 1, -1, 3, 3, 0,
    ];
    assert_eq!(q1(scores), 18, "Q1 Failed: Incorrect total sum.");
}

fn q2(king_safety_ratings: Vec<u8>) -> Vec<bool> {
    // This funciton should transform king_safety_rating (a Vec<u8>) into a Vec<bool> where elements are true if >= 5.
    // TODO: Make this code parallel, by replacing iter() with par_iter().
    king_safety_ratings
        .par_iter()
        // Remember, iter()/par_iter() produces references
        .map(|&rating| rating >= 5)
        .collect()
}

#[test]
fn test_q2_parallel_king_safety_check() {
    let ratings: Vec<u8> = vec![2, 7, 3, 9, 1, 6, 4, 8, 2, 5, 10];
    let expected: Vec<bool> = vec![
        false, true, false, true, false, true, false, true, false, true, true,
    ];
    assert_eq!(
        q2(ratings),
        expected,
        "Q2 Failed: Incorrect boolean mapping."
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate(char, u8);

fn q3(all_pieces: Vec<Coordinate>) -> Vec<Coordinate> {
    // This function should extract all Coordinates where the file is on the Queenside ('a' through 'd').
    // TODO: Replace the following code with a single statement using into_par_iter(), filter(),
    // and collect()

    all_pieces.into_par_iter().filter(|coord|coord.0 <= 'd').collect()

    // for coord in all_pieces.into_par_iter() {
    //     if coord.0 <= 'd' {
    //         v.push(coord);
    //     }
    // }
    //
    // v
}

#[test]
fn test_q3_parallel_piece_filtering() {
    let all_pieces: Vec<Coordinate> = vec![
        Coordinate('e', 4),
        Coordinate('a', 7),
        Coordinate('h', 2),
        Coordinate('d', 5),
        Coordinate('c', 8),
        Coordinate('f', 1),
        Coordinate('b', 6),
        Coordinate('g', 3),
        Coordinate('a', 2),
    ];
    let expected_result: Vec<Coordinate> = vec![
        Coordinate('a', 7),
        Coordinate('d', 5),
        Coordinate('c', 8),
        Coordinate('b', 6),
        Coordinate('a', 2),
    ];
    let mut result = q3(all_pieces);
    result.sort_by_key(|c| (c.0, c.1));
    let mut expected = expected_result;
    expected.sort_by_key(|c| (c.0, c.1));
    assert_eq!(result, expected, "Q3 Failed: Incorrect pieces filtered.");
}

fn q4(attack_counts: Vec<u8>) -> usize {
    // This function should count the total number of squares that have an attack count of 3.
    // TODO: Replace the following function with a single statement using par_iter(), filter(), and
    // count()

    attack_counts.par_iter().filter(|count| **count == 3).count()

}

#[test]
fn test_q4_parallel_counting() {
    let attack_counts: Vec<u8> = vec![1, 3, 0, 5, 3, 2, 3, 1, 4, 3, 0, 6];
    assert_eq!(q4(attack_counts), 4, "Q4 Failed: Incorrect count.");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
    id: u32,
    base_score: i32,
}

fn q5(potential_moves: Vec<Move>) -> Vec<Move> {
    // This function should:
    //      0. Use par_iter() to iterate in parallel
    //      1. For all moves in potential_moves, produces a new move with 10 added to ites base_score (use map())
    //      2. Use filter() to eliminate moves with base_scores <= 50
    //      3. collect()

    potential_moves.par_iter().map(|m| Move {id: m.id, base_score: m.base_score + 10,}).filter(|m|m.base_score > 50).collect()

    // // Replace the following code
    // let mut v = Vec::new();
    // for m in potential_moves {
    //     let new_m = Move {
    //         id: m.id,
    //         base_score: m.base_score + 10,
    //     };
    //     if new_m.base_score > 50 {
    //         v.push(new_m);
    //     }
    // }
    //
    // v
}

#[test]
fn test_q5_parallel_move_generation() {
    let potential_moves: Vec<Move> = (0..20)
        .map(|i| Move {
            id: i,
            base_score: (i as i32) * 5 + 30,
        })
        .collect();
    let expected_ids: Vec<u32> = (3..20).collect();
    let result_ids: Vec<u32> = q5(potential_moves).iter().map(|m| m.id).collect();
    assert_eq!(
        result_ids, expected_ids,
        "Q5 Failed: Incorrect moves filtered/mapped."
    );
}

fn calculate_zobrist_hash(state_id: u64) -> u64 {
    // fake
    (state_id.wrapping_mul(0x9E3779B97F4A7C15)).wrapping_add(state_id)
}

fn q6(game_states: Vec<u64>) -> Vec<u64> {
    // This function should calculate the Zobrist hash for every state in parallel.
    // TODO: Use par_iter(), map(), and collect()

    game_states.par_iter().map(|state|calculate_zobrist_hash(*state)).collect()

    // Replace the following code
    // let mut v = Vec::new();
    // for state in game_states {
    //     v.push(calculate_zobrist_hash(state));
    // }
    // v
}

#[test]
fn test_q6_parallel_position_hashing() {
    let num_states = 1000;
    let game_states: Vec<u64> = (0..num_states).collect();
    let expected: Vec<u64> = game_states
        .iter()
        .map(|&state_id| calculate_zobrist_hash(state_id))
        .collect();
    assert_eq!(q6(game_states), expected, "Q6 Failed: Hashes do not match.");
}

fn q7(board_attack_levels: &mut Vec<u8>) {
    // This function should mutate board_attack_levels so that any value > 4 becomes 4.
    // TODO: Use par_iter_mut() and for_each() (No collect() needed!)

    board_attack_levels.par_iter_mut().for_each(|level|if *level > 4{*level = 4})

    // Replace the following code
    // for level in board_attack_levels.iter_mut() {
    //     if *level > 4 {
    //         *level = 4;
    //     }
    // }
}

#[test]
fn test_q7_parallel_mutable_iteration() {
    let mut levels: Vec<u8> = vec![1, 5, 2, 6, 3, 7, 4, 8, 0, 5, 1, 6, 2, 7, 3, 8];
    let expected: Vec<u8> = vec![1, 4, 2, 4, 3, 4, 4, 4, 0, 4, 1, 4, 2, 4, 3, 4];
    q7(&mut levels);
    assert_eq!(
        levels, expected,
        "Q7 Failed: Values were not clamped correctly."
    );
}

fn q8(game_material_codes: Vec<u8>) -> Option<u8> {
    // TODO: Use into_par_iter() and find_any() (no collect() needed!) to find an element that equals 10 or 11.
    // Note that find_any() returns an Option, which is why this function does too

    game_material_codes.into_par_iter().find_any(|&code| code == 10 || code == 11)

    // Replace the following code
    // for code in game_material_codes.into_iter() {
    //     if code == 10 || code == 11 {
    //         return Some(code);
    //     }
    // }
    // return None;
}

#[test]
fn test_q8_parallel_search() {
    let present: Vec<u8> = vec![1, 3, 5, 8, 2, 11, 4, 6, 9, 7];
    assert_eq!(
        q8(present.clone()),
        Some(11),
        "Q8 Failed: Did not find the material code."
    );
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SearchNode {
    depth: u8,
    evaluation: i32,
}

fn q9(search_nodes: Vec<SearchNode>) -> Vec<i32> {
    // TODO:
    //      1. Use into_par_iter() to consume the vector in parallell
    //      2. filter() all nodes with a depth > 5
    //      3. map() each node to its evaluation score
    //      4. collect() the scores and return them

    search_nodes.into_par_iter().filter(|node| node.depth > 5).map(|node|node.evaluation).collect()

    // Replace the following code
    // let mut v = Vec::new();
    // for node in search_nodes.into_iter() {
    //     if node.depth > 5 {
    //         v.push(node.evaluation);
    //     }
    // }
    //
    // v
}

#[test]
fn test_q9_into_par_iter() {
    let search_nodes = vec![
        SearchNode {
            depth: 4,
            evaluation: 100,
        },
        SearchNode {
            depth: 7,
            evaluation: 850,
        },
        SearchNode {
            depth: 2,
            evaluation: -50,
        },
        SearchNode {
            depth: 6,
            evaluation: 200,
        },
        SearchNode {
            depth: 8,
            evaluation: 9999,
        },
    ];
    let mut expected = vec![850, 200, 9999];
    expected.sort();

    let mut result = q9(search_nodes);
    result.sort();

    assert_eq!(
        result, expected,
        "Q9 Failed: Did not correctly filter and map evaluations."
    );
}

fn q10(move_scores: &mut [i32]) {
    // TODO: Use par_sort() to sort the move scores in ASCENDING order (lowest first)

    // Edit the following code
    move_scores.par_sort();
}

#[test]
fn test_q10_par_sort() {
    let mut move_scores = vec![100, -50, 200, 50, 150];
    let expected = vec![-50, 50, 100, 150, 200]; // Ascending order (default)

    q10(&mut move_scores);
    assert_eq!(
        move_scores, expected,
        "Q10 Failed: Scores were not sorted in ascending order using par_sort."
    );
}

fn q11(move_scores: &mut [i32]) {
    // TODO: Use par_sort_by() to sort the move scores in  DESCENDING order (highest first)

    // Edit the following code
    move_scores.par_sort_by(|a, b| b.cmp(a));
}

#[test]
fn test_q11_par_sort_by() {
    let mut move_scores = vec![100, -50, 200, 50, 150];
    let expected = vec![200, 150, 100, 50, -50]; // Descending order

    q11(&mut move_scores);
    assert_eq!(
        move_scores, expected,
        "Q11 Failed: Scores were not sorted in descending order using par_sort_by."
    );
}

fn q12(board: &mut [u8]) {
    // This function should iterate (using par_chunks_mut()) over 8-element chunks of the board
    // for_each() chunk, iterate over the elements (using regular iter_mut()) and multiply every element by 2.

    board.par_iter().for_each(|board|board.par_chunks_mut()).iter_mut(val|*val *= 2);

    // Replace the following code
    // for chunk in board.chunks_mut(8) {
    //     for val in chunk.iter_mut() {
    //         *val *= 2;
    //     }
    // }
}

#[test]
fn test_q12_par_chunks_mut() {
    let mut board: Vec<u8> = vec![
        1, 2, 3, 4, 1, 2, 3, 4, // Chunk 1
        0, 1, 0, 1, 0, 1, 0, 1, // Chunk 2
    ];
    let expected: Vec<u8> = vec![2, 4, 6, 8, 2, 4, 6, 8, 0, 2, 0, 2, 0, 2, 0, 2];

    q12(&mut board);
    assert_eq!(
        board, expected,
        "Q12 Failed: Parallel chunk modification did not multiply values correctly."
    );
}

fn q13(board_material: &[u8]) -> Vec<u8> {
    // This function should iterate over the board_material in 8-element chunks and sum each chunk
    // together in parallel.

    // TODO:
    //      1. Use par_chunks(8) to iterate over 8-element chunks
    //      2. Use map() to apply iter().sum() to each chunk
    //      3. collect() the sums
    // calculate the sum of the material and collect the sums.

    // Replace the following code
    let mut v = Vec::new();
    for chunk in board_material.chunks(8) {
        let mut sum = 0;
        for val in chunk.iter() {
            sum += val;
        }
        v.push(sum);
    }
    v
}

#[test]
fn test_q13_par_chunks() {
    let board_material: Vec<u8> = vec![
        // Rank 1: R, N, B, Q, K, B, N, R (5+3+3+9+0+3+3+5 = 31)
        5, 3, 3, 9, 0, 3, 3, 5, // Rank 2: P, P, P, P, P, P, P, P (1*8 = 8)
        1, 1, 1, 1, 1, 1, 1, 1, // Rank 3 (Empty)
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let expected: Vec<u8> = vec![31, 8, 0];

    let result = q13(&board_material);
    assert_eq!(
        result, expected,
        "Q13 Failed: Parallel chunk evaluation did not correctly sum material by rank."
    );
}
