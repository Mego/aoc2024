use std::sync::LazyLock;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    DO,
    DONT,
    MUL(u64, u64),
}

peg::parser! {
    grammar day3parser() for str {
        rule do_rule() -> Instruction
            = "do()" { Instruction::DO }

        rule dont() -> Instruction
            = "don't()" { Instruction::DONT }

        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("u64")) }

        rule mul() -> Instruction
            = "mul(" a:number() "," b:number() ")" { Instruction::MUL(a, b) }

        rule token() -> Instruction
            = do_rule() / dont() / mul()

        rule junk() -> &'input str
            = !token() s:$([c if c.is_ascii_whitespace() || c.is_ascii_graphic()]) { s }

        #[no_eof]
        pub rule day3() -> Vec<Instruction>
            = junk()* l:(token() ** (junk()*)) junk()* { l }
    }
}

pub fn part1(input: String) -> u64 {
    if let Ok(tokens) = day3parser::day3(&input) {
        return tokens.into_iter().fold(0, |acc, token| {
            acc + match token {
                Instruction::MUL(a, b) => a * b,
                _ => 0,
            }
        });
    }
    0
}

pub fn part2(input: String) -> u64 {
    if let Ok(tokens) = day3parser::day3(&input) {
        return tokens
            .into_iter()
            .fold((0, true), |(acc, enabled), token| match token {
                Instruction::DO => (acc, true),
                Instruction::DONT => (acc, false),
                Instruction::MUL(a, b) => {
                    (acc + enabled.then(|| a * b).unwrap_or_default(), enabled)
                }
            })
            .0;
    }
    0
}
