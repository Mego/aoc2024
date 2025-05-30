use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    sync::LazyLock,
};

use reqwest::header::COOKIE;

const MY_COOKIE: &str = include_str!("../../cookie.txt");

pub async fn fetch_input(year: u16, day: u8) -> String {
    let fname = format!("inputs/{}/day{}.txt", year, day);
    if let Ok(contents) = read_to_string(&fname) {
        return contents;
    }
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let data = reqwest::Client::new()
        .get(url)
        .header(COOKIE, MY_COOKIE)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let f = File::options()
        .create(true)
        .write(true)
        .open(&fname)
        .unwrap();
    let mut buf = BufWriter::new(f);
    write!(&mut buf, "{}", data).unwrap();
    data
}

static CORRECT_ANSWERS: LazyLock<HashMap<(u8, u64), u64>> = LazyLock::new(|| {
    [
        ((1, 1), 1590491),
        ((1, 2), 22588371),
        ((2, 1), 516),
        ((2, 2), 561),
        ((3, 1), 174561379),
        ((3, 2), 106921067),
        ((4, 1), 2549),
        ((4, 2), 2003),
        ((5, 1), 4578),
        ((5, 2), 6179),
        ((6, 1), 5305),
        ((6, 2), 2143),
        ((7, 1), 1620690235709),
        ((7, 2), 145397611075341),
        ((8, 1), 413),
        ((8, 2), 1417),
        ((9, 1), 6607511583593),
        ((9, 2), 6636608781232),
        ((10, 1), 825),
        ((10, 2), 1805),
        ((11, 1), 218956),
        ((11, 2), 259593838049805),
        ((12, 1), 1396298),
        ((12, 2), 853588),
        ((13, 1), 29023),
        ((13, 2), 96787395375634),
        ((14, 1), 230686500),
        ((14, 2), 7672),
        ((15, 1), 1446158),
        ((15, 2), 1446175),
        ((16, 1), 107468),
        ((16, 2), 533),
        ((17, 2), 190593310997519),
        ((18, 1), 314),
        ((19, 1), 233),
        ((19, 2), 691316989225259),
        ((20, 1), 1367),
        ((20, 2), 1006850),
        ((21, 1), 123096),
        ((21, 2), 154517692795352),
        ((22, 1), 20411980517),
        ((22, 2), 2362),
        ((23, 1), 1046),
        ((24, 1), 69201640933606),
        ((25, 1), 3155),
        ((25, 2), 0), // compatibility
    ]
    .into_iter()
    .collect()
});

static CORRECT_STR_ANSWERS: LazyLock<HashMap<(u8, u64), &str>> = LazyLock::new(|| {
    [
        ((17, 1), "3,1,5,3,7,4,2,7,5"),
        ((18, 2), "15,20"),
        ((23, 2), "de,id,ke,ls,po,sn,tf,tl,tm,uj,un,xw,yz"),
        ((24, 2), "dhq,hbs,jcp,kfp,pdg,z18,z22,z27"),
    ]
    .into_iter()
    .collect()
});

#[cfg(test)]
pub fn submit_int(_year: u16, day: u8, level: u64, answer: u64) -> String {
    if let Some(&val) = CORRECT_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    return "unsolved".to_owned();
}

#[cfg(test)]
pub fn submit_str(_year: u16, day: u8, level: u64, answer: String) -> String {
    if let Some(&val) = CORRECT_STR_ANSWERS.get(&(day, level)) {
        if answer == val {
            return format!("right ({answer})");
        }
        return format!("wrong (expected {val}, received {answer})");
    }
    return "unsolved".to_owned();
}

#[cfg(test)]
pub fn submit(year: u16, day: u8, level: u64, answer: String) -> String {
    if CORRECT_STR_ANSWERS.contains_key(&(day, level)) {
        return submit_str(year, day, level, answer);
    }
    submit_int(year, day, level, answer.parse::<u64>().unwrap())
}
