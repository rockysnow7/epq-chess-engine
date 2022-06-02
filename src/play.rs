#![allow(dead_code)]

use crate::engine::{Engine, print_board};
use chess::{Board, BoardStatus, Game, GameResult, ChessMove, Color};
use std::io::{self, Write};
use text_io::read;

/// Return true if a given string is a legal move, or false otherwise.
fn is_valid_move(board: &Board, player_move_str: &str) -> bool {
    let player_move = ChessMove::from_san(board, player_move_str);
    match player_move {
        Ok(m) => board.legal(m),
        Err(_) => false,
    }
}

/// Play a game of chess as a given colour against a given engine.
pub fn play_game(player_color: Color, engine: &mut Engine) {
    let mut board = Board::default();
    print_board(&board);

    // main game loop
    while board.status() == BoardStatus::Ongoing {
        if board.side_to_move() == player_color {
            loop {
                print!("Player move: ");
                let _ = io::stdout().flush();
                let player_move_str: String = read!("{}\n");

                if is_valid_move(&board, &player_move_str) {
                    let mut temp_board = board.clone();
                    let player_move = ChessMove::from_san(&board, &player_move_str).unwrap();
                    board.make_move(player_move, &mut temp_board);
                    board = temp_board;
                    break;
                } else {
                    println!("Invalid move.");
                }
            }
            println!();
        } else {
            let best_move = engine.best_move(&board, false);
            let mut temp_board = board.clone();
            board.make_move(best_move, &mut temp_board);
            board = temp_board;
            println!("Engine move: {}\n", best_move);
        }

        print_board(&board);
        println!("Eval: {}\n", engine.evaluate_board(&board));
    }
}

/// Play a game of chess between two engines and return +1 if white
/// wins, -1 if black wins, and 0 if it's a draw.
pub fn play_engines(engine_white: &mut Engine, engine_black: &mut Engine, show: bool) -> i8 {
    let mut game = Game::new();
    if show {
        print_board(&game.current_position());
    }

    // main game loop
    while game.result().is_none() {
        if game.can_declare_draw() {
            game.declare_draw();
        }

        if game.side_to_move() == Color::White {
            let best_move = engine_white.best_move(&game.current_position(), false);
            game.make_move(best_move);

            if show {
                println!("White: {}\n", best_move);
                print_board(&game.current_position());
            }
        } else {
            let best_move = engine_black.best_move(&game.current_position(), false);
            game.make_move(best_move);

            if show {
                println!("Black: {}\n", best_move);
                print_board(&game.current_position());
            }
        }
    }

    match game.result() {
        Some(GameResult::WhiteCheckmates) | Some(GameResult::BlackResigns) => {
            if show {
                println!("White wins!");
            }

            1
        }
        Some(GameResult::BlackCheckmates) | Some(GameResult::WhiteResigns) => {
            if show {
                println!("Black wins!");
            }

            -1
        }
        Some(GameResult::Stalemate) | Some(GameResult::DrawDeclared) => {
            if show {
                println!("Draw!");
            }

            0
        }
        _ => {
            panic!("Unexpected game result!");
        }
    }
}