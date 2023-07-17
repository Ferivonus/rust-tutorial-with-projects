use std::thread;
use std::time::Duration;
use rand::Rng;

struct Character {
    name: String,
    health: i32,
}

impl Character {
    fn new(name: String, health: i32) -> Character {
        Character {
            name,
            health,
        }
    }

    fn attack(&self, target: &mut Character) {
        let damage = rand::thread_rng().gen_range(1..=10);
        target.take_damage(damage);
        println!("{} attacked {} and dealt {} damage!", self.name, target.name, damage);
        thread::sleep(Duration::from_secs(2));
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health < 0 {
            self.health = 0;
        }
        println!("{} took {} damage! Health: {}", self.name, damage, self.health);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn main() {
    let mut player = Character::new(String::from("Player"), 100);
    let mut enemy = Character::new(String::from("Enemy"), 100);

    loop {
        player.attack(&mut enemy);
        if !enemy.is_alive() {
            println!("{} defeated {}!", player.name, enemy.name);
            break;
        }

        enemy.attack(&mut player);
        if !player.is_alive() {
            println!("{} defeated {}!", enemy.name, player.name);
            break;
        }
    }
}


/*
    Cargo.toml:

    [package]
    name = "rust-tutorial-with-projects"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    rand = "0.8.4"
*/