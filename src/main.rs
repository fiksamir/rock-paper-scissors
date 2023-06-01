use std::io;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::fmt;
use std::str;
use std::io::Write;

enum RockPaperScissorsGuess { 
    Rock, 
    Paper,
    Scissors,
}

#[derive(Debug)]
enum ParseRockPaperScissorsGuessError {
    Unknown(String),
}

enum RockPaperScissorsCompare {
    RockCrushesScissors,
    PaperCoversRock,
    ScissorsCutPaper,
}
 
enum RockPaperScissorsResult {
    Win(RockPaperScissorsCompare),
    Loss(RockPaperScissorsCompare),
    Tie(String),
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}
  
impl fmt::Display for RockPaperScissorsGuess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsGuess::Rock    => write!(f, "Камінь"),
            RockPaperScissorsGuess::Paper   => write!(f, "Папір"),
            RockPaperScissorsGuess::Scissors => write!(f, "Ножиці"),
        }
    }
}

impl Distribution<RockPaperScissorsGuess> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockPaperScissorsGuess {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => RockPaperScissorsGuess::Rock,
            1 => RockPaperScissorsGuess::Paper,
            2 => RockPaperScissorsGuess::Scissors,
            _ => unreachable!(),
        }
    }
}

impl str::FromStr for RockPaperScissorsGuess {
    type Err = ParseRockPaperScissorsGuessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "к" | "камінь"    => Ok(RockPaperScissorsGuess::Rock),
            "п" | "папір"   => Ok(RockPaperScissorsGuess::Paper),
            "н" | "ножиці" => Ok(RockPaperScissorsGuess::Scissors),
            _   => Err(ParseRockPaperScissorsGuessError::Unknown(s.to_string())),
        }
    }
}

impl Compare<RockPaperScissorsGuess, RockPaperScissorsResult> for RockPaperScissorsGuess{
    fn compare(&self, b: &RockPaperScissorsGuess) -> RockPaperScissorsResult {
        match self {
            RockPaperScissorsGuess::Rock => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Tie(self.to_string()),
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::PaperCoversRock),
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::RockCrushesScissors)
                }
            },
            RockPaperScissorsGuess::Paper => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::PaperCoversRock),                        
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Tie(self.to_string()),                    
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::ScissorsCutPaper),                        
                }
            },
            RockPaperScissorsGuess::Scissors => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::RockCrushesScissors),
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::ScissorsCutPaper),
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Tie(self.to_string()),
                }
            }			
        }
    }
}
 
impl fmt::Display for RockPaperScissorsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsResult::Win(result) => {
                match result {
                    RockPaperScissorsCompare::RockCrushesScissors => write!(f, "Камінь ламає ножиці"),
                    RockPaperScissorsCompare::PaperCoversRock => write!(f, "Папір покриває камінь"),
                    RockPaperScissorsCompare::ScissorsCutPaper => write!(f, "Ножиці ріжуть папір"),
                }
            },
            RockPaperScissorsResult::Loss(result) => {
                match result {
                    RockPaperScissorsCompare::RockCrushesScissors => write!(f, "Камінь ламає ножиці"),
                    RockPaperScissorsCompare::PaperCoversRock => write!(f, "Папір покриває камінь"),
                    RockPaperScissorsCompare::ScissorsCutPaper => write!(f, "Ножиці ріжуть папір"),
                }
            },
            RockPaperScissorsResult::Tie(result) => write!(f, "{result}"),
        }
    }
}

fn main() {
    println!("Привіт, нумо пограємо в Камінь Ножиці Папір!");
    println!("Переможе той, хто першим виграє 3 раунди.");
 
    let mut player_wins: i32 = 0;
    let mut comp_wins: i32 = 0;
    let mut round:i32  = 1;
    let mut quit: bool = false;

    'game: loop { // гра       

        'round: loop{ // раунд

            println!("Раунд {}:", round);

            let comp_move: RockPaperScissorsGuess = rand::thread_rng().gen();

            print!("Будь ласка, обери (к)амінь, (п)апір, або (н)ожиці:");

            let _ = match io::stdout().flush() {
                Ok(str) => str,
                Err(_) => {
                    println!("Помилка консолі.\n");
                    quit = true;
                    break 'game;
                }
            };
            
            let mut player_move = String::new();

            io::stdin()
                .read_line(&mut player_move)
                .expect("Помилка читання");

            let player_move: Result<RockPaperScissorsGuess, ParseRockPaperScissorsGuessError>
                = player_move.trim().parse();

            let player_move = match player_move {
                Ok(player_move_val) => {
                    println!("");
                    println!("Ти обрав {}", player_move_val);
                    println!("Я обрав {}", comp_move);
                    round += 1;
                    player_move_val
                },
                Err(ParseRockPaperScissorsGuessError::Unknown(s)) => {  // перевірка виходу з гри
                    match &s[..] {
                        "q" | "quit" | "в" | "вихід" => {
                            println!("Вже йдеш? Добре.");
                            quit = true;
                            break 'game;
                        },
                        _            => {
                            println!("\"{}\" не вірний вибір, спробуй ще раз.\n",s);
                            continue 'round
                        },
                    }
                }            
            };

            let result: RockPaperScissorsResult = player_move.compare(&comp_move);
    
            match result {
                RockPaperScissorsResult::Win(_) => {
                    player_wins += 1;
                    println!("{}", result);
                    println!("Ти виграв цей раунд.\n");
                },
                RockPaperScissorsResult::Tie(_) => println!("Нічия...\n"),
                RockPaperScissorsResult::Loss(_) => {
                    comp_wins += 1;
                    println!("{}", result);
                    println!("Ти програв цей раунд.\n");
                },
            }
            break 'round;
        }
        //кінець раунду

        // перевірка рахунку
        if player_wins == 3 {
            println!("Вітаю, ти переможець у грі! До зустрічі!\n");
            break 'game;
        } else if comp_wins == 3 {
            println!("На жаль...Ти програв цю гру! Щасти наступного разу.\n");
            break 'game;
        } else {
            println!("У тебе {} перемог, у мене - {}.\n", player_wins, comp_wins);           
        }
    }

    if quit == true {
        println!("Що ж... дякую за гру. Шкода, що ти йдеш так швидко.");
    }
}
