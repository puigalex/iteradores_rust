#![allow(unused)]
#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]
#![feature(iter_from_coroutine)]

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
    let v = vec![1, 2, 4, 2, 1, 5, 0];
    for x in increasing_u32(v.into_iter()) {
        print!("{x} ");
    }

    let v = vec![1, 2, 4, 2, 1, 5, 0];
    for x in increasing_generic(v.into_iter()) {
        print!("{x} ");
    }
    let strings = vec![String::from("foo"), String::from("bar"), String::from("zoo"), String::from("art")];
    for x in increasing_generic(strings.into_iter()) {
        print!("{x} ");
    }
}

