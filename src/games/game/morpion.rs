use maud::{html, Markup};

use super::game::Game;
use crate::games::game_id::GameId;

pub struct Morpion {
    id: String,
    name: String,
}

impl Morpion {
    pub fn new(game_id: GameId) -> Self {
        Morpion {
            id: game_id.id,
            name: game_id.name,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

fn get_cell_css<'a>() -> &'a str {
    "border-2 min-w-36 min-h-36"
}

fn game_box(row: usize, col: usize, is_first_player: bool) -> Markup {
    let id = format!("box-{}-{}", row, col);
    html! {
        button #(id).(get_cell_css()) {
            #test
                .text-8xl.opacity-0.w-full.h-full."hover:opacity-100".flex.items-center.justify-center { 
                @if is_first_player {"X"} @else {"O"} 
            }
        }
    }
}

fn checked_box(checked_by: bool) -> Markup {
    html! {
        div.(get_cell_css()) {
            #test
                .text-8xl.w-full.h-full.flex.items-center.justify-center { 
                @if checked_by {"X"} @else {"O"} 
            }
        }
    }
}

impl Game for Morpion {
    fn render(&self) -> Markup {
        let is_first_player = true;
        let current_game= &mut [[0; 3]; 3];
        current_game[1][1] = 1;
        current_game[2][2] = 2; 
        html! {
            #game_contener.flex.items-center.justify-center.w-full.h-full.pt-4 {
                #game_grid.grid.grid-cols-3.grid-rows-3 {
                    @for i in 0..3 {
                        @for j in 0..3 {
                            @if current_game[i][j] == 0 {
                                (game_box(i, j, is_first_player))
                            } @else {
                                (checked_box(current_game[i][j] == 1))
                            }
                        }
                    }
                }
            }
        }
    }
}
