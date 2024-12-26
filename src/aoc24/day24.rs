use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Instruction {
    AND(String, String),
    OR(String, String),
    XOR(String, String),
}

impl Instruction {
    fn is_and(&self) -> bool {
        match self {
            Self::AND(_, _) => true,
            _ => false,
        }
    }

    fn is_or(&self) -> bool {
        match self {
            Self::OR(_, _) => true,
            _ => false,
        }
    }

    fn is_xor(&self) -> bool {
        match self {
            Self::XOR(_, _) => true,
            _ => false,
        }
    }

    fn operands(&self) -> (&String, &String) {
        match self {
            Self::AND(a, b) | Self::OR(a, b) | Self::XOR(a, b) => (a, b),
        }
    }

    fn try_resolve(&self, wires: &HashMap<String, Option<bool>>) -> Option<bool> {
        match self {
            Self::AND(a, b) => {
                if let Some(a_val) = wires[a] {
                    if let Some(b_val) = wires[b] {
                        let val = a_val && b_val;
                        return Some(val);
                    }
                }
                None
            }
            Self::OR(a, b) => {
                if let Some(a_val) = wires[a] {
                    if let Some(b_val) = wires[b] {
                        let val = a_val || b_val;
                        return Some(val);
                    }
                }
                None
            }
            Self::XOR(a, b) => {
                if let Some(a_val) = wires[a] {
                    if let Some(b_val) = wires[b] {
                        let val = a_val ^ b_val;
                        return Some(val);
                    }
                }
                None
            }
        }
    }
}

fn resolve(values: &mut HashMap<String, Option<bool>>, cnxs: &HashMap<String, Instruction>) {
    loop {
        let mut updates = HashMap::new();
        for wire in values.keys().by_ref() {
            let value = values[wire];
            if value.is_none() {
                if let Some(val) = cnxs[wire].try_resolve(&values) {
                    updates.insert(wire.to_owned(), val);
                }
            }
        }
        if updates.is_empty() {
            break;
        }
        for (wire, val) in updates {
            values.insert(wire, Some(val));
        }
    }
}

fn parse(input: String) -> (HashMap<String, Option<bool>>, HashMap<String, Instruction>) {
    // :wires:
    let mut wires = HashMap::new();
    let mut res = HashMap::new();
    let (wires_list, ops) = input.split("\n\n").collect_tuple().unwrap();
    for wire in wires_list.lines() {
        let (name, val) = wire.split(": ").collect_tuple().unwrap();
        wires.insert(name.to_owned(), Some(val == "1"));
    }

    for op_line in ops.lines() {
        let (op_str, to) = op_line.split(" -> ").collect_tuple().unwrap();
        let (a_name, op_name, b_name) = op_str.split(" ").collect_tuple().unwrap();
        wires.entry(a_name.to_owned()).or_insert(None);
        wires.entry(b_name.to_owned()).or_insert(None);
        let instr = match op_name {
            "AND" => Instruction::AND(a_name.to_owned(), b_name.to_owned()),
            "OR" => Instruction::OR(a_name.to_owned(), b_name.to_owned()),
            "XOR" => Instruction::XOR(a_name.to_owned(), b_name.to_owned()),
            _ => unimplemented!(),
        };
        wires.entry(to.to_owned()).or_insert(None);
        res.insert(to.to_owned(), instr);
    }

    (wires, res)
}

pub fn part1(input: String) -> u64 {
    let (mut wires, cnxs) = parse(input);
    resolve(&mut wires, &cnxs);
    // dbg!(&wires);
    // 0
    wires
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by_cached_key(|(k, _)| k.clone())
        .rev()
        .map(|(_, v)| v.unwrap())
        .fold(0, |a, v| (a << 1) + v.then_some(1).unwrap_or_default())
}

fn validate(k: &String, i: &Instruction, cnxs: &HashMap<String, Instruction>) -> bool {
    if k.starts_with("z") && k != "z45" {
        return i.is_xor();
    }

    let (a, b) = i.operands();
    if !k.starts_with("z")
        && !((a.starts_with("x") && b.starts_with("y"))
            || (a.starts_with("y") && b.starts_with("x")))
    {
        return i.is_and() || i.is_or();
    }

    if ((a.starts_with("x") && b.starts_with("y") && a != "x00" && b != "y00")
        || (a.starts_with("y") && b.starts_with("x") && a != "y00" && b != "x00"))
        && i.is_xor()
    {
        return cnxs.iter().any(|(_, i2)| {
            let (a2, b2) = i2.operands();
            (a2 == k || b2 == k) && i2.is_xor()
        });
    }

    if ((a.starts_with("x") && b.starts_with("y") && a != "x00" && b != "y00")
        || (a.starts_with("y") && b.starts_with("x") && a != "y00" && b != "x00"))
        && i.is_and()
    {
        return cnxs.iter().any(|(_, i2)| {
            let (a2, b2) = i2.operands();
            (a2 == k || b2 == k) && i2.is_or()
        });
    }

    true
}

pub fn part2(input: String) -> String {
    let (_, cnxs) = parse(input);

    cnxs.iter()
        .filter(|&(k, i)| {
            let invalid = !validate(k, i, &cnxs);
            // if invalid {
            //     dbg!(k, i);
            // }
            invalid
        })
        .map(|(k, _)| k.clone())
        .sorted()
        .join(",")
}
