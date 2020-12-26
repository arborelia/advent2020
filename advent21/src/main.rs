#[macro_use]
extern crate pest_derive;
use eyre::Result;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FoodParser;

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
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
        ingredients,
        allergens,
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

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let foods = parse_foods(&input)?;
    for food in foods.iter() {
        println!("{:?}", food);
    }
    Ok(())
}
