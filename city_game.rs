use std::{io::{self, Read}, borrow::Borrow};
use rand::Rng;
#[derive(Clone, Copy)]
struct City {
    population: u32, //amount of people
    contentment: u32, // 0-100, happiness 
    pollution: u32, // 0-100, how polluted city is
    energy: u32,
    energy_used: u32,
    money: u32,
    space: u32,
    space_used: u32,
}
#[derive(Clone, Copy)]
struct Buildings {
    residence: u32,
    skyscraper: u32,
    coal_plant: u32,
    wind_plant: u32,
    business: u32,
    arena: u32,
    total: u32,
}
fn main() {
    let data = new_city();
    let mut user_city = data.0;
    let mut user_buildings = data.1;
    let city_name = data.2;
    while true {
        println!("\ntype build to build land, info to see your stats, quit to quit game, and next to progress a year");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("penis");
        input = input.trim().to_owned();
        if input == "build"{
            let updated = build(user_city, user_buildings);
            user_city = updated.0;
            user_buildings = updated.1;
        } else if input == "info" {
            info(user_city, user_buildings);
        } else if input == "next" {
            let updated = yearly_update(user_city, user_buildings);
            user_city = updated.0;
            user_buildings = updated.1;
        } else if input == "quit" {
            break
        };
    }
}
fn new_city() -> (City, Buildings, String){
    println!("choose a name for your city");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line");
    let mut user_city = City {
        population: 0,
        contentment: 50,
        pollution: 0,
        energy: 10,
        energy_used: 0,
        money: 250,
        space: 20,
        space_used: 0,
    };
    let mut user_buildings = Buildings {
        residence: 0,
        skyscraper: 0,
        coal_plant: 0,
        wind_plant: 0,
        business: 0,
        arena: 0,
        total: 0,
    };
    return (user_city, user_buildings, input)
}
fn info(city: City, buildings: Buildings) {
    println!("population: {}", city.population);
    println!("contentment: {}", city.contentment);
    println!("pollution: {}", city.pollution);
    println!("energy: {}", city.energy);
    println!("money: {}", city.money);    
    println!("space: {}", city.space);
    println!("residences: {}", buildings.residence);
    println!("skyscrapers: {}", buildings.skyscraper);
    println!("coal plants: {}", buildings.coal_plant);
    println!("wind plants: {}", buildings.wind_plant);
    println!("businesses: {}", buildings.business);
    println!("arenas : {}", buildings.arena);    
}
fn update(mut city: City, mut buildings: Buildings) -> (City, Buildings) {
    buildings.total = buildings.residence + buildings.skyscraper + buildings.coal_plant + buildings.wind_plant + buildings.business + buildings. arena;
    city.population = (buildings.residence * 5) + (buildings.skyscraper * 50);
    city.energy_used = (buildings.residence * 1) + (buildings.skyscraper * 10) + (buildings.business * 3) + (buildings.arena * 5);
    city.space_used = (buildings.residence * 1) + (buildings.skyscraper * 10) + (buildings.business * 2) + (buildings.arena * 4);
    city.energy = 10 + buildings.coal_plant * 5 + buildings.wind_plant * 3;
    return(city, buildings)
}
fn yearly_update(mut city: City, mut buildings: Buildings) -> (City, Buildings) {
    let updated = update(city, buildings);
    city = updated.0;
    buildings = updated.1;
    city.pollution += buildings.coal_plant * 5 + buildings.skyscraper * 2;
    if city.pollution > 100{
        city.pollution = 100
    }
    city.pollution -= buildings.wind_plant;
    if city.pollution < 1 {
        city.pollution = 0
    }
    city.contentment -= city.pollution;
    city.contentment += buildings.arena * 3;
    println!("population contentment is now {}", city.contentment);
    let taxes = city.population * city.contentment / 5;
    println!("taxes paid {}", taxes);
    city.money += taxes;
    return (city, buildings)
}
fn build(mut city: City, mut buildings: Buildings) -> (City, Buildings) {
    println!("what would you like to build? type the letter corresponding with the building\n
    residence: r -> costs $100, 1 energy, and 1 space\n
    skyscraper: s -> costs $1000, 8 energy, 7 space\n
    coal_plant: c -> costs $300, 2 energy, 3 space\n
    wind_plant: w -> costs $500, 1 energy, 3 space\n
    business: b -> costs $400, 3 energy, 2 space \n
    plot of land: l -> costs $100");
    let updated = update(city, buildings);
    city = updated.0;
    buildings = updated.1;
    let mut building = String::new();
    io::stdin().read_line(&mut building).expect("penis");
    building = building.trim().to_owned();
    if building != "r" && building != "s" && building != "c" && building != "w" && building != "b" && building != "l"{
        println!("not a building");
        return (city, buildings)
    } else if building == "r"{
        if city.energy - city.energy_used > 0 && city.space - city.space_used > 0 && city.money >= 100 {
            println!("building residence");
            buildings.residence += 1;
            city.money -= 100;
            return (city, buildings)
        } else if city.energy - city.energy_used < 1{
            println!("not enough energy");
            return (city, buildings)
        } else if city.space - city.space_used < 1 {
            println!("not enough space");
            return (city, buildings)
        } else if city.money < 100 {
            println!("not enough money");
            return (city, buildings)
        }
    } else if building == "s"{
        if city.energy - city.energy_used > 7 && city.space - city.space_used > 6 && city.money >= 1000 {
            println!("building skyscraper");
            buildings.skyscraper += 1;
            city.money -= 1000;
            return (city, buildings)
        } else {
            println!("cannot build skyscraper");
            return (city, buildings)
        }
    } else if building == "c"{
        if city.space - city.space_used > 2 && city.money >= 300 {
            println!("building coal plant");
            buildings.coal_plant += 1;
            city.money -= 300;
            return (city, buildings)
        } else {
            println!("cannot build coal plant");
            return (city, buildings)
        }
    } else if building == "w"{
        if city.space - city.space_used > 2 && city.money >= 500 {
            println!("building wind plant");
            buildings.wind_plant += 1;
            city.money -= 500;
            return (city, buildings)
        } else {
            println!("cannot build wind plant");
            return (city, buildings)
        }
    } else if building == "b"{
        if city.energy - city.energy_used > 2 && city.space - city.space_used > 1 && city.money >= 400 {
            println!("building business");
            buildings.business += 1;
            city.money -= 400;
            return (city, buildings)
        } else {
            println!("cannot build business");
            return (city, buildings)
        }
    } else if building == "l"{
        if city.money >= 100 {
            println!("building plot");
            city.space += 1;
            city.money -= 100;
            return (city, buildings)
        } else {
            println!("cannot build plot");
            return (city, buildings)
        }
    } else {
        return (city, buildings)
    }
}
