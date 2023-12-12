// Separator <==> separator with n spaces left/right
// <> == No separator
// * == one any char
// ** == many any chars except newline
// *** == ignore line

// Escape sequence start with \

/*
v nl {v  char}

*/

// use itertools::Itertools;

use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    *,
};
pub mod math;

#[derive(Debug)]
struct SpecificationList {
    specifications: Vec<Specification>,
    macro_delimiters: Vec<Delimiter>,
}

#[derive(Debug)]
struct Specification {
    delimiter: Delimiter,
    container: ContainerType,
}

#[derive(Debug)]
enum Delimiter {
    None,
    NewLine,
    DoubleNewLine,
    Space,
    Custom(Box<str>),
}

#[derive(Debug)]
enum ContainerType {
    Vec(InnerType),
    Tuple(Vec<InnerType>),
    HashMap(InnerType, InnerType),
    HashSet(InnerType),
    String,
}

#[derive(Debug)]
enum InnerType {
    Integer,
    Float,
    Char,
    Nested(Box<Specification>),
}

impl Parse for Specification {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!()
    }
}

impl From<&str> for Delimiter {
    fn from(value: &str) -> Self {
        match value {
            "" => Delimiter::None,
            "nl" => Delimiter::NewLine,
            "nlnl" => Delimiter::DoubleNewLine,
            "sp" => Delimiter::Space,
            _ => Delimiter::Custom(value.into()),
        }
    }
}

fn parse_specs(specs: &str) -> SpecificationList {
    let mut lines = specs.lines();
    let mut specifications: Vec<Specification> =
        vec![syn::parse2::<Specification>(lines.next().unwrap().parse().unwrap()).unwrap()];
    let mut macro_delimiters = vec![];
    while let Some(line) = lines.next() {
        macro_delimiters.push(line.into());
        specifications
            .push(syn::parse2::<Specification>(lines.next().unwrap().parse().unwrap()).unwrap());
    }
    SpecificationList {
        specifications,
        macro_delimiters,
    }
}

pub fn choices<T: Copy>(glyphs: &[T], n_elements: usize) -> Vec<Vec<T>> {
    if n_elements == glyphs.len() {
        vec![glyphs.into()]
    } else if n_elements == 0 {
        vec![vec![]]
    } else {
        let g = glyphs[0];
        let mut new_choices_with = choices(&glyphs[1..], n_elements - 1);
        for choice in new_choices_with.iter_mut() {
            choice.push(g);
        }
        let new_choices_without = choices(&glyphs[1..], n_elements);

        new_choices_with.extend(new_choices_without);
        new_choices_with
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_vec_vec_char() {
        println!("{:?}", parse_specs("v nl {v  c}"))
    }
}

// impl From<(&str, &str)> for ContainerType {
//     fn from((container_ty, inner_ty): (&str, &str)) -> Self {
//         match container_ty {
//             "v" => ContainerType::Vec(InnerType::from(inner_ty)),
//             "t" => ContainerType::Tuple,
//             "hm" => {let key_ty =  ContainerType::HashMap},
//             "set" => ContainerType::HashSet(InnerType::from(inner_ty)),
//             "str" => ContainerType::String,
//             _ => panic!("Invalid container {container_ty}"),
//         }
//     }
// }

// impl From<&str> for InnerType {
//     fn from(value: &str) -> Self {
//         match value {
//             "i" => InnerType::Integer,
//             "c" => InnerType::Char,
//             "f" => InnerType::Float,
//             _ if value.starts_with('{') && value.ends_with('}') => InnerType::Nested(
//                 Specification::from(value.strip_prefix('{').unwrap().strip_suffix('}').unwrap())
//                     .into(),
//             ),
//             _ => panic!("Invalid inner {value}"),
//         }
//     }
// }

// impl From<&str> for Specification {
//     fn from(s: &str) -> Self {
//         let (container_ty, s) = s.split_once(' ').unwrap();
//         let (del, inner_ty) = s.split_once(' ').unwrap();
//         Specification {
//             delimiter: Delimiter::from(del),
//             container: ContainerType::from((container_ty, inner_ty)),
//         }
//     }
// }

// fn parse_specs(spec: &str) -> SpecificationList {
//     let mut lines = spec.lines();
//     let mut specifications = vec![Specification::from(lines.next().unwrap())];
//     let mut macro_delimiters = vec![];
//     while let Some(line) = lines.next() {
//         macro_delimiters.push(line.into());
//         specifications.push(lines.next().unwrap().into());
//     }
//     SpecificationList {
//         specifications,
//         macro_delimiters,
//     }
// }

// Nice to have: Lines, string, stringvec
// Tuple<Separator, Value types>
// Hashmap<Separator, key type, value type>
