struct Game {
    id: usize,
    draws: Vec<CubeSet>,
}

impl Game {
    fn parse(line: &str) -> Game {
        let (game, draws) = line.split_once(": ").unwrap();
        let id = game.split_once(' ').unwrap().1.parse::<usize>().unwrap();
        let draws = draws.split("; ").map(CubeSet::parse).collect();
        Game { id, draws }
    }

    fn is_possible(&self, bag: &CubeSet) -> bool {
        self.draws.iter().all(|draw| draw.is_possible(bag))
    }

    fn minimum_bag(&self) -> CubeSet {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in &self.draws {
            min_red = min_red.max(draw.red);
            min_green = min_green.max(draw.green);
            min_blue = min_blue.max(draw.blue);
        }
        CubeSet { red: min_red, green: min_green, blue: min_blue }
    }
}

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn parse(draw: &str) -> CubeSet {
        let mut parts = draw.split(' ');
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        while let (Some(num), Some(colour)) = (parts.next(), parts.next()) {
            if colour.starts_with("red") {
                red = num.parse().unwrap();
            } else if colour.starts_with("green") {
                green = num.parse().unwrap();
            } else if colour.starts_with("blue") {
                blue = num.parse().unwrap();
            }
        }
        CubeSet { red, green, blue }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

trait Draw {
    fn is_possible(&self, bag: &CubeSet) -> bool;
}

impl Draw for CubeSet {
    fn is_possible(&self, bag: &CubeSet) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }
}

fn main() {
    let input = include_str!("../../input/day02");
    let games = input.lines().map(Game::parse).collect::<Vec<_>>();

    const PART_1_BAG: CubeSet = CubeSet { red: 12, green: 13, blue: 14 };
    let sum_of_possible_games: usize = games.iter()
        .filter(|game| game.is_possible(&PART_1_BAG))
        .map(|game| game.id)
        .sum();
    println!("Part 1: {}", sum_of_possible_games);
    
    let sum_of_minimum_powers: usize = games.iter()
        .map(|game| game.minimum_bag().power())
        .sum();
    println!("Part 2: {}", sum_of_minimum_powers);
}