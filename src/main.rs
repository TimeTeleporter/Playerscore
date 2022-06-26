#[warn(unused_assignments)]

use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

trait Playerlist {
    fn read_from_data() -> Vec<Player>;
    fn add_player(self) -> Vec<Player>;
    fn remove_player(self) -> Vec<Player>;
    fn edit_wins(self) -> Vec<Player>;
    fn write_to_data(self);
    fn print_data(&self);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Player {
    name: String,
    wins: u32,
}

// This needs the Playerlist Trait as else we cannot implement methods on Basic types.
impl Playerlist for Vec<Player> {
    fn read_from_data() -> Vec<Player> {
        let file = File::open("data.txt");

        let mut file: File = match file {
            Ok(file) => file,
            Err(e) => {
                print!("{:?}", e);
                return Vec::new();
            }
        };
        
        let mut content: String = String::new();
        file.read_to_string(&mut content).expect("Hoppla 5");

        let mut vec: Vec<Player> = Vec::new();
        for line in content.lines() {
            let (name, wins) = match line.split(", ").collect_tuple() {
                Some(result) => result,
                None => {
                    continue;
                }
            };
            let wins: u32 = match wins.parse() {
                Ok(result) => result,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
            vec.push(Player {name: name.to_string(), wins: wins});
        }

        vec
    }

    fn add_player(mut self) -> Vec<Player> {
        loop {
            println!("Adding players\n");
            let input: String = get_player_name();
            match input.as_str() {
                "" | "quit" => { break; },
                name => { self.push(Player {name: name.to_owned(), wins: 0} ); },
            }
        }

        self.into_iter().unique().collect()
    }

    fn remove_player(mut self) -> Vec<Player> {
        loop {
            print!("Removing a player\n");
            let input: String = get_player_name();

            match input.as_str() {
                "" | "quit" => {break; },
                _ => {},
            }

            self = self.into_iter().filter(| Player { name, wins: _} | name.as_str() != input).collect();
        }

        self
    }

    fn edit_wins(mut self) -> Vec<Player> {
        loop {
            println!("Editing playerscores\n");
            let input: String = get_player_name();
            match input.as_str() {
                "" | "quit" => { break; }
                inputname => {
                    let ( index, Player { name, wins} ) = match self.clone()
                        .into_iter()
                        .enumerate()
                        .filter(| ( _ , Player { name, wins: _ } ) | name.as_str() == inputname )
                        .next() {
                            Some(tpl) => tpl,
                            None => {
                                println!("No such player was found");
                                continue;
                            }
                        };

                    let mut input = String::new();
                    
                    loop {
                        println!("{}'s score is {}, how much would you like to change it?", name, wins);
                        std::io::stdin().read_line(&mut input).expect("Failed to read player edit");
                        match input.trim().parse::<i32>() {
                            Ok(output) => {
                                self[index] = Player { name: name, wins: ((wins as i32) + output) as u32};
                                break;
                            },
                            Err(e) => {
                                println!("{:?}", e);
                                continue;
                            },
                        }
                    }
                }
            }
        }

        self
    }

    fn write_to_data(self) {
        let mut file = File::create("data.txt").expect("Hoppla 2");
        for Player {name, wins} in self {
            let string: String = name + ", " + &wins.to_string() + "\n";
            file.write_all(string.as_ref()).expect("Hoppla 3");
        }
    }

    fn print_data(&self) {
        println!("We currently have the following players:");
        for player in self {
            println!("{}: {} wins.", player.name, player.wins);
        }
    }
}

// Refactored the input method
fn get_player_name() -> String {
    println!("Enter the name of the player or press enter to quit");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_lowercase()
}

enum Menu {
    ChooseAction,
    AddPlayer,
    RemovePlayer,
    EditWins,
    Exit,
}

fn main() {
    let mut players: Vec<Player> = Vec::read_from_data();

    let mut menu = Menu::ChooseAction;

    loop {
        match menu {
            Menu::ChooseAction => {
                players.print_data();

                println!("What do you want to do?\n\
                \n\
                0. Home\n\
                1. Add Player\n\
                2. Remove Player\n\
                3. Edit Wins\n\
                4. Exit\n\
                ");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read");
                let input = input.trim().parse::<u32>();

                let input: u32 = match input {
                    Ok(result) => result,
                    Err(_) => continue,
                };

                match input {
                    1 => { menu = Menu::AddPlayer; }
                    2 => { menu = Menu::RemovePlayer; }
                    3 => { menu = Menu::EditWins; }
                    4 => { menu = Menu::Exit; }
                    _ => {}
                }
            }
            Menu::AddPlayer => {
                players = players.add_player();
                menu = Menu::EditWins;
            }
            Menu::RemovePlayer => {
                players = players.remove_player();
                menu = Menu::ChooseAction;
            }
            Menu::EditWins => {
                players = players.edit_wins();
                menu = Menu::ChooseAction;
            }
            Menu::Exit => {
                break;
            }
        }
    }

    players.print_data();

    players.write_to_data();

    
}