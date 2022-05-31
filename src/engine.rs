#![allow(dead_code)]

use chess::{Board, BoardStatus, ChessMove, Color, File, Game, GameResult, MoveGen, Rank, Square};
use neuroflow::FeedForward;
use std::io::Read;
use serde::{Serialize, Deserialize};
use indicatif::ProgressBar;

const NUM_FEATURES: usize = 65;

/// Print a character-based representation of a given board.
pub fn print_board(board: &Board) {
    for i in (0..8).rev() {
        for j in 0..8 {
            let square = Square::make_square(Rank::from_index(i), File::from_index(j));
            let piece = board.piece_on(square);
            let color = board.color_on(square);

            if piece == None {
                print!(". ");
            } else {
                print!("{} ", piece.unwrap().to_string(color.unwrap()));
            }
        }
        println!("");
    }
    println!("");
}

/// Return an tensor of features representing a given board.
fn features(board: &Board) -> [f64; NUM_FEATURES] {
    let mut features = [0.0; NUM_FEATURES];
    features[NUM_FEATURES - 1] = if board.side_to_move() == Color::White {
        1.0
    } else {
        -1.0
    };

    for i in 0..8 {
        for j in 0..8 {
            let square = Square::make_square(Rank::from_index(i), File::from_index(j));
            let piece = board.piece_on(square);
            let color = board.color_on(square);

            if piece != None {
                features[i * 8 + j] = (piece.unwrap().to_index() as f64 + 1.0)
                    * if color.unwrap() == Color::White {
                        1.0
                    } else {
                        -1.0
                    };
            }
        }
    }

    features
}

fn evaluator_nn() -> FeedForward {
    FeedForward::new(&[NUM_FEATURES as i32, 32, 1])
}

#[derive(Serialize, Deserialize)]
pub struct Engine {
    search_depth: u8, // in ply
    uses_pruning: bool,
    eval_nn: FeedForward,
}

impl Engine {
    /// Create a new engine with the given search depth and an untrained evaluator
    /// neural network.
    pub fn new(search_depth: u8, uses_pruning: bool) -> Engine {
        Engine {
            search_depth: search_depth,
            uses_pruning: uses_pruning,
            eval_nn: evaluator_nn(),
        }
    }

    /// Load an engine from the given file.
    pub fn new_from_file(filename: &str) -> Engine {
        let mut file = std::fs::File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let engine: Engine = serde_json::from_str(&contents).unwrap();

        engine
    }

    /// Save the engine to the given file in JSON format.
    pub fn save(&self, filename: &str) {
        let serialized = serde_json::to_string(&self).unwrap();
        std::fs::write(filename, serialized).unwrap();
    }

    /// Return the evaluation of a terminal node.
    fn evaluate_terminal(&mut self, board: &Board) -> f64 {
        let board_features = features(board);
        let out = self.eval_nn.calc(&board_features);
        //println!("{:?}", out);

        out[0]
    }

    /// Return the evaluation of a non-terminal node by the negamax algorithm.
    fn evaluate_nonterminal(&mut self, board: &Board, depth: u8) -> f64 {
        if depth == 0 || board.status() != BoardStatus::Ongoing {
            return self.evaluate_terminal(board)
                * if board.side_to_move() == Color::White {
                    1.0
                } else {
                    -1.0
                };
        }

        let mut value = std::f64::NEG_INFINITY;
        for m in MoveGen::new_legal(board) {
            let mut temp_board = board.clone();
            board.make_move(m, &mut temp_board);
            value = value.max(-self.evaluate_nonterminal(&temp_board, depth - 1));
        }

        value
    }

    /// Return the evaluation of a non-terminal node by the negamax algorithm, with
    /// various pruning algorithms applied.
    fn evaluate_nonterminal_pruned(&mut self, board: &Board, depth: u8) -> f64 {
        0.0
    }

    /// Public interface to the `evaluate_nonterminal` function.
    pub fn evaluate_board(&mut self, board: &Board) -> f64 {
        if self.uses_pruning {
            self.evaluate_nonterminal_pruned(board, self.search_depth)
        } else {
            self.evaluate_nonterminal(board, self.search_depth)
        }
    }

    /// Return the best move for a given board.
    pub fn best_move(&mut self, board: &Board, show: bool) -> ChessMove {
        let legal_moves = MoveGen::new_legal(board);
        let mut best_value = std::f64::NEG_INFINITY;
        let mut best_move = ChessMove::new(Square::A1, Square::A1, None); // null move, avoids warning for uninitialised return value

        if show {
            println!("Searching {} moves...\n", legal_moves.len());
        }
        for m in legal_moves {
            let mut temp_board = board.clone();
            board.make_move(m, &mut temp_board);
            let value = self.evaluate_board(&temp_board);
            if value > best_value {
                best_value = value;
                best_move = m;
            }
        }

        //println!("{:?}", features(board));

        best_move
    }

    /// Play a game between this engine and itself, and return +1 if white wins, -1 if
    /// black wins, and 0 if it is a draw, along with a vector of the features of each
    /// position from the game.
    fn play_self(&mut self, show: bool) -> (i8, Vec<[f64; NUM_FEATURES]>) {
        let mut game = Game::new();
        let mut positions = vec![features(&game.current_position())];

        if show {
            println!("NEW GAME\n");
            print_board(&game.current_position());
        }

        while game.result().is_none() {
            if game.can_declare_draw() {
                game.declare_draw();
            }

            let best_move = self.best_move(&game.current_position(), show);
            game.make_move(best_move);

            if show {
                print_board(&game.current_position());
                println!("{:?}: {}\n", !game.side_to_move(), best_move);
            }
            positions.push(features(&game.current_position()));
        }

        let result = game.result().unwrap();
        if result == GameResult::WhiteCheckmates || result == GameResult::BlackResigns {
            if show {
                println!("White wins.");
            }
            return (1, positions);
        }
        if result == GameResult::BlackCheckmates || result == GameResult::WhiteResigns {
            if show {
                println!("Black wins.");
            }
            return (-1, positions);
        }

        if show {
            println!("Draw.");
        }
        (0, positions)
    }

    /// Train the engine via self-play, playing the given number of games.
    pub fn train(&mut self, num_games: u32, show: bool) {
        let mut white_wins = 0;
        let mut black_wins = 0;
        let mut draws = 0;

        let pb = ProgressBar::new(num_games as u64);
        for _ in 0..num_games {
            pb.inc(1);

            let (winner, features) = self.play_self(show);
            if winner == 1 {
                white_wins += 1;
            } else if winner == -1 {
                black_wins += 1;
            } else if winner == 0 {
                draws += 1;
            }

            for i in 0..features.len() {
                self.eval_nn.fit(&features[i], &[winner as f64]);
            }
        }
        pb.finish();
        println!("Training finished.\nWhite wins: {}\nBlack wins: {}\nDraws: {}", white_wins, black_wins, draws);
    }

    /// Trains the engine and then saves it to a file.
    pub fn train_and_save(&mut self, num_games: u32, show: bool, filename: &str) {
        self.train(num_games, show);
        self.save(filename);
    }
}