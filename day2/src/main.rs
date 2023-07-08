use Rps::*;
use Outcome::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

struct Round(Rps, Rps);

struct RiggedRound(Rps, Outcome);

impl RiggedRound {
    fn play(&self) -> Rps {
        let possible = vec![Rock, Paper, Scissors];
        *possible.iter()
            .filter(|&&rps| (Round(self.0, rps)).outcome() == self.1)
            .next().unwrap()
    }

    fn score(&self) -> u32 {
        let my_contrib: u32 = self.play() as u32;
        let outcome_contrib: u32 = self.1 as u32;
        my_contrib + outcome_contrib
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        let Round(opponent, player) = self;
        if opponent == player{
            return Draw;
        }
        match opponent {
            Rock => if *player == Paper {
                Win
            } else { Loss },
            Paper => if *player == Scissors {
                Win
            } else { Loss },
            Scissors => if *player == Rock {
                Win
            } else { Loss },
        }
    }

    fn score(&self) -> u32 {
        let my_contrib: u32 = self.1 as u32;
        let outcome_contrib: u32 = self.outcome() as u32;
        my_contrib + outcome_contrib
    }
}

impl From<&str> for Round {
    fn from(s: &str) -> Round {
        let mut opponent = Rock;
        let mut player = Rock;
        for (i, rps) in s.split(" ").enumerate() {
            match i {
                0 => opponent = match rps {
                    "A" => Rock,
                    "B" => Paper,
                    "C" => Scissors,
                    _ => panic!("Invalid character for opponent action")
                },
                1 => player = match rps {
                    "X" => Rock,
                    "Y" => Paper,
                    "Z" => Scissors,
                    _ => panic!("Invalid character for player action")
                },
                _ => panic!("Too many characters")
            }
        }

        Round(opponent, player)
    }
}

impl From<&str> for RiggedRound {
    fn from(s: &str) -> RiggedRound {
        let mut opponent = Rock;
        let mut outcome = Loss;
        for (i, c) in s.split(" ").enumerate() {
            match i {
                0 => opponent = match c {
                    "A" => Rock,
                    "B" => Paper,
                    "C" => Scissors,
                    _ => panic!("Invalid character for opponent action")
                },
                1 => outcome = match c {
                    "X" => Loss,
                    "Y" => Draw,
                    "Z" => Win,
                    _ => panic!("Invalid character for player action")
                },
                _ => panic!("Too many characters")
            }
        }

        RiggedRound(opponent, outcome)
    }
}

fn main() {
    let contents: Vec<String> = include_str!("../input.txt").lines()
        .map(|s| s.to_string()).collect();
    // let rounds: Vec<Round> = contents.iter().map(|s| s.as_str().into()).collect(); // part 1
    let rounds: Vec<RiggedRound> = contents.iter().map(|s| s.as_str().into()).collect();

    println!("Total score: {}", rounds.iter().map(|round| round.score()).sum::<u32>());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_rps() {
        let possible = vec![Rock, Paper, Scissors];
        let results = (&possible).iter()
            .map(|&rps0| (&possible).iter()
                .map(move |&rps1| (Round(rps0,rps1)).outcome()))
            .flatten().collect::<Vec<Outcome>>();
        let should_be = vec![Draw, Win, Loss, Loss, Draw, Win, Win, Loss, Draw];
        assert_eq!(should_be, results);
    }

    #[test]
    fn check_example() {
        let contents: Vec<String> = vec!["A Y".into(), "B X".into(), "C Z".into()];
        let rounds: Vec<Round> = contents.iter().map(|s| s.as_str().into()).collect();
        assert_eq!(vec![8, 1, 6], rounds.iter().map(|round| round.score()).collect::<Vec<u32>>())
    }

    #[test]
    #[ignore]
    fn check_rps_2() {
        let possible = vec![Rock, Paper, Scissors];
        let results = (&possible).iter()
            .map(|&rps0| (&possible).iter()
                .map(move |&rps1| (Round(rps0,rps1)).outcome()))
            .flatten().collect::<Vec<Outcome>>();
        let should_be = vec![Draw, Win, Loss, Loss, Draw, Win, Win, Loss, Draw];
        assert_eq!(should_be, results);
    }

    #[test]
    fn check_example_2() {
        let contents: Vec<String> = vec!["A Y".into(), "B X".into(), "C Z".into()];
        let rounds: Vec<RiggedRound> = contents.iter().map(|s| s.as_str().into()).collect();
        assert_eq!(vec![4, 1, 7], rounds.iter().map(|round| round.score()).collect::<Vec<u32>>())
    }
}
