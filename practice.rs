extern crate rand;

use std::io;
use rand::Rng;

struct Player {
    hp: i32,
    stamina: i32,
    power: i32,
    gold: i32,
    full_hp: i32,
    actions: i32,
    steps: i32,
}

#[derive(Clone)]
struct Enemy {
    name: String,
    hp: i32,
    stamina: i32,
    power: i32,
}

enum Direction {
    North,
    South,
    West,
    East,
}

enum EncounterType {
    Nothing,
    Bush,
    Meat,
    Water,
    Herb,
    IronOre,
    Enemy(EnemyType),
}

enum EnemyType {
    Rat,
    Wolf,
    Boar,
    Tiger,
    Dragon,
}

impl Player {
    fn new() -> Player {
        Player {
            hp: 100,
            stamina: 50,
            power: 10,
            gold: 0,
            actions: 0,
            steps: 0,
            full_hp: 100,
        }
    }

    fn walk(&mut self) {
        self.stamina -= 1;
        println!("You walked. Now your stamina is {}", self.stamina);
        self.steps +- 1;
    }

    // fn is_alive 
    // encounter_type and enemy_type variables type define

    fn encounter (&mut self, enemies: Vec<Enemy>) {
        let rng = rand::thread_rng();
        let encounter_type = match rng.gen_range(0..=100) {
            1..=17 => EncounterType::Nothing,
            18..=25 => EncounterType::Bush,
            26..=40 => EncounterType::Meat,
            41..=55 => EncounterType::Water,
            56..=69 => EncounterType:: Herb,
            70..=74 => EncounterType::IronOre,
            _=> {
                let enemy_type = match rng.gen_range(0..=100) {
                    1..=45 => EnemyType:: Rat,
                    46..=70 => EnemyType:: Wolf,
                    71..=85 => EnemyType:: Boar,
                    86..=95 => EnemyType:: Tiger,
                    _=> EnemyType:: Dragon,
                };
                EncounterType::Enemy(enemy_type)
            }
        };

        // enemies pass
        match encounter_type {
            EncounterType::Nothing => println!("Nothing happens"),
            EncounterType::Bush => {
                self.stamina -= 1;
                println!("You encountered a bush. Your stamina is reduced by 1");
            },
            EncounterType::Meat => {
                self.hp = std::cmp::min(self.hp + 4, 100);
                println!("You encountered meat. Your hp is increased by 4");
            },
            EncounterType::Water => {
                self.hp = std::cmp::min(self.hp + 2, 100);
                println!("You encountered water. Your hp is increased by 2");
            }

            EncounterType::Herb => {
                self.power += 1;
                println!("You encountered a herb. Your power is increased by 1");
            },
            EncounterType::IronOre => {
                self.power += 10;
                println!("You encountered an ironore. Your power is increased by 10");
            },
            EncounterType::Enemy(enemy_type) => {
                let enemy: Enemy = self.create_enemy(enemy_type, enemies) ;
                println!("You encountered a {} with HP: {}, Stamina: {}, Power: {}.", enemy.name, enemy.hp, enemy.stamina, enemy.power);
                self.fight_or_flee(enemy);
            }
        }

    }

    fn fight_or_flee(&mut self, enemy: Enemy) {

    }
    fn create_enemy(&self, enemy_type: EnemyType, enemies: Vec<Enemy>) -> Enemy {
        match enemy_type {
            Rat=> enemies[0].clone(),
            Wolf=> enemies[1].clone(),
            Boar=> enemies[2].clone(),
            Tiger=> enemies[3].clone(),
            Dragon=> enemies[4].clone(),
        }
    }
}


fn main () {

}