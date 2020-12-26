#[macro_use]
extern crate pest_derive;
use eyre::Result;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FoodParser;

#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn only_child(pair: Pair<Rule>) -> Pair<Rule> {
    pair.into_inner().next().unwrap()
}

fn interpret_food(ast: Pairs<Rule>) -> Food {
    let mut ingredients: Vec<String> = Vec::new();
    let mut allergens: Vec<String> = Vec::new();
    for pair in ast {
        match pair.as_rule() {
            Rule::ingredients => {
                ingredients = pair
                    .into_inner()
                    .map(|ingredient| ingredient.as_str().to_string())
                    .collect();
            }
            Rule::allergens => {
                allergens = only_child(pair)
                    .into_inner()
                    .map(|ingredient| ingredient.as_str().to_string())
                    .collect();
            }
            _ => unreachable!(),
        }
    }
    Food {
        ingredients: HashSet::from_iter(ingredients.into_iter()),
        allergens: HashSet::from_iter(allergens.into_iter()),
    }
}

fn parse_foods(input: &str) -> Result<Vec<Food>> {
    let parsed = FoodParser::parse(Rule::foodlist, &input)?.next().unwrap();
    let mut foods: Vec<Food> = vec![];
    for foodline in parsed.into_inner() {
        match foodline.as_rule() {
            Rule::food => {
                let ast = foodline.into_inner();
                foods.push(interpret_food(ast));
            }
            _ => unreachable!(format!("{:?}", foodline.as_rule())),
        }
    }
    Ok(foods)
}

fn all_ingredients(foods: &[Food]) -> HashSet<String> {
    let mut ingredient_set: HashSet<String> = HashSet::new();
    for food in foods {
        ingredient_set.extend(food.ingredients.iter().cloned());
    }
    ingredient_set
}

fn all_allergens(foods: &[Food]) -> Vec<String> {
    let mut allergen_set: HashSet<String> = HashSet::new();
    for food in foods {
        allergen_set.extend(food.allergens.iter().cloned());
    }
    Vec::from_iter(allergen_set)
}

fn solve_allergens(foods: &[Food]) -> (HashMap<String, String>, HashSet<String>) {
    let allergens = all_allergens(&foods);
    let n_allergens = allergens.len();
    let mut remaining_ingredients = all_ingredients(&foods);
    let mut known_allergens: HashMap<String, String> = HashMap::new();

    while known_allergens.len() < n_allergens {
        for allergen in allergens.iter() {
            if !known_allergens.contains_key(allergen) {
                let mut possible_ingredients: HashSet<String> = remaining_ingredients.clone();
                for food in foods.iter() {
                    if food.allergens.contains(allergen) {
                        possible_ingredients = &possible_ingredients & &food.ingredients;
                    }
                }
                println!("{} could be: {:?}", allergen, possible_ingredients);
                if possible_ingredients.len() == 1 {
                    let ingredient = possible_ingredients.iter().next().unwrap().to_owned();
                    println!("{} is {}", allergen, ingredient);
                    known_allergens.insert(allergen.clone(), ingredient.clone());
                    remaining_ingredients.remove(&ingredient);
                }
            }
        }
    }
    let unsafe_ingredients: HashSet<String> = HashSet::from_iter(known_allergens.values().cloned());
    let safe_ingredients: HashSet<String> = all_ingredients(&foods)
        .iter()
        .cloned()
        .filter(|ingr| !unsafe_ingredients.contains(ingr))
        .collect();
    println!("safe ingredients: {:?}", safe_ingredients);
    (known_allergens, safe_ingredients)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let foods = parse_foods(&input)?;

    let (known_allergens, safe_ingredients) = solve_allergens(&foods);
    let mut n_safe_ingredients: usize = 0;
    for food in foods.iter() {
        let food_safe: HashSet<String> = &food.ingredients & &safe_ingredients;
        n_safe_ingredients += food_safe.len();
    }
    let mut sorted_allergens: Vec<String> = known_allergens.keys().cloned().collect();
    sorted_allergens.sort();
    let canonical_unsafe: Vec<String> = sorted_allergens
        .iter()
        .map(|allergen| known_allergens[allergen].clone())
        .collect();
    println!("Safe ingredient instances: {}", n_safe_ingredients);
    println!("Unsafe ingredients: {}", canonical_unsafe.join(","));

    Ok(())
}
