#![allow(unused)]
#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]
#![feature(iter_from_coroutine)]

use clap::{App, Arg};
use std::str::FromStr;

fn increasing_u32(input: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    std::iter::from_coroutine(#[coroutine] move || {
        let mut input = input;
        match input.next() {
            Some(mut anterior) => {
                yield anterior;
                while let Some(num_actual) = input.next() {
                    match anterior < num_actual {
                        true => {
                            yield num_actual;
                            anterior = num_actual;
                        },
                        false => continue,
                    }}},
            None => {}
        }
    })}


fn increasing_generic<T: PartialOrd + Clone>(mut input: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let mut anterior: Option<T> = None;
    std::iter::from_coroutine(#[coroutine] move || {
        while let Some(generic_actual) = input.next() {
            match &anterior {
                Some(generic_anterior) if generic_actual > *generic_anterior => {
                    yield generic_actual.clone(); //No pude encontrar una forma de adaptar esta funcion para solo utilizar move
                    anterior = Some(generic_actual);
                }
                None => {
                    yield generic_actual.clone();
                    anterior = Some(generic_actual);
                }
                _ => {}
            }}
    })}

fn main() {
    let matches = App::new("Tarea iteradores")
        .arg(Arg::new("numeros")
            .long("numeros")
            .takes_value(true)
            .multiple_values(true))
        .arg(Arg::new("textos")
            .long("textos")
            .takes_value(true)
            .multiple_values(true))
        .get_matches();

    if let Some(numeros) = matches.values_of("numeros") {
        let numeros: Vec<u32> = numeros
            .map(|s| u32::from_str(s).unwrap())
            .collect();
        for x in increasing_u32(numeros.into_iter()) {
            print!("{} ", x);
        }
        println!();
    }

    if let Some(textos) = matches.values_of("textos") {
        let textos: Vec<String> = textos.map(|s| s.to_string()).collect();
        for x in increasing_generic(textos.into_iter()) {
            print!("{} ", x);
        }
        println!();
    }
}

