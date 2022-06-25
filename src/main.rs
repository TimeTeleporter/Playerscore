use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

trait Playerlist {
    fn read_from_data() -> Vec<Player>;
    fn add_entry(self) -> Vec<Player>;
    fn edit_wins(self) -> Vec<Player>;
    fn write_to_data(self);
    fn print_data(&self);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Player {
    name: String,
    wins: u32,
}

impl Playerlist for Vec<Player> {
    fn read_from_data() -> Vec<Player> {
        let mut file = File::open("data.txt").expect("Hoppla 4");
        
        let mut content = String::new();
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

    fn add_entry(mut self) -> Vec<Player> {
        loop {
            let mut input = String::new();
            println!("Enter a new player:");
            std::io::stdin().read_line(&mut input).expect("Failed player to enter");
            let input = input.trim().to_lowercase(); 
            match input.as_str() {
                "" | "quit" => { break; }
                name => { self.push(Player {name: name.to_owned(), wins: 0} ); }
            }
        }

        self.into_iter().unique().collect()
    }

    fn edit_wins(mut self) -> Vec<Player> {
        loop {
            let mut input = String::new();
            println!("What player do you want to edit? Write ''quit'' to quit.");
            std::io::stdin().read_line(&mut input).expect("Failed to read player to edit");
            let stringinput = input.trim().to_lowercase();
            match stringinput.as_str() {
                "" | "quit" => { break; }
                inputname => {
                    let ( index, Player { name, wins} ) = match self.clone()
                        .into_iter()
                        .enumerate()
                        .filter(| ( index, Player { name, wins } ) | name.as_str() == inputname )
                        .next() {
                            Some(tpl) => tpl,
                            None => {
                                println!("No such player was found");
                                continue;
                            }
                        };

                    let mut change: u32 = 0;
                    let mut input = String::new();
                    
                    loop {
                        println!("{}'s score is {}, how much would you like to change it?", name, wins);
                        std::io::stdin().read_line(&mut input).expect("Failed to read player edit");
                        match input.trim().parse() {
                            Ok(output) => {
                                change = output;
                                break;
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                continue;
                            }
                        }
                    }
                    
                    self[index] = Player { name: name, wins: (wins + change)};
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

enum Menu {
    ChooseAction,
    AddPlayer,
    EditWins,
    Exit,
}

fn main() {
    let mut players: Vec<Player> = Vec::read_from_data();

    println!("Database reads");
    players.print_data();

    let mut menu = Menu::ChooseAction;

    loop {
        match menu {
            Menu::ChooseAction => {
                println!("What do you want to do?\n\
                \n\
                0. Start\n\
                1. Add Player\n\
                2. Edit Wins\n\
                3. Exit\n\
                ");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read");
                let input: u32 = input.trim().parse().unwrap();

                match input {
                    1 => { menu = Menu::AddPlayer; }
                    2 => { menu = Menu::EditWins; }
                    3 => { menu = Menu::Exit; }
                    _ => {}
                }
            }
            Menu::AddPlayer => {
                players = players.add_entry();
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