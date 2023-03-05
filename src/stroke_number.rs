use regex::Regex;
use std::collections::HashMap;

use crate::util::read_lines;

#[derive(Debug, Default)]
pub struct Stroke {
    pub stroke_map: HashMap<String, i64>,
    pub split_map: HashMap<String, Vec<String>>,
}

impl Stroke {
    pub fn new() -> Self {
        let mut stroke_map: HashMap<String, i64> = HashMap::new();
        let mut split_map: HashMap<String, Vec<String>> = HashMap::new();
        if let Ok(lines) = read_lines("../data/stroke.dat") {
            for line in lines {
                if let Ok(data) = line {
                    let tmp: Vec<String> = data.split("|").map(|s| s.to_string()).collect();
                    let key = tmp.get(1).unwrap().to_owned();
                    let mut value = tmp.get(2).unwrap().to_owned();

                    value = value.replace("\n", "");
                    stroke_map.insert(key, value.parse::<i64>().unwrap_or(0));
                }
            }
        }
        if let Ok(lines) = read_lines("../data/chaizi-ft.dat") {
            for line in lines {
                if let Ok(data) = line {
                    let reg = Regex::new(r"\s").unwrap();
                    let tmp: Vec<String> =
                        reg.split(data.as_str()).map(|x| x.to_string()).collect();
                    if tmp.len() < 2 {
                        continue;
                    }
                    let key = tmp.get(1).unwrap().to_owned();
                    let mut split_list: Vec<String> = Vec::new();
                    for (index, _) in tmp[1..tmp.len() - 1].to_vec().iter().enumerate() {
                        let v = tmp.get(index).unwrap();
                        split_list.push(v.to_string());
                    }
                    split_map.insert(key, split_list);
                }
            }
        }
        Stroke {
            stroke_map,
            split_map,
        }
    }

    pub fn get_stroke_number(&self, word: String) -> i64 {
        let mut total: i64 = 0;
        for char in word.chars() {
            if char.to_string() == "一" {
                total += 1;
            } else if char.to_string() == "二" {
                total += 2;
            } else if char.to_string() == "三" {
                total += 3;
            } else if char.to_string() == "四" {
                total += 4;
            } else if char.to_string() == "五" {
                total += 5;
            } else if char.to_string() == "六" {
                total += 6;
            } else if char.to_string() == "七" {
                total += 7;
            } else if char.to_string() == "八" {
                total += 8;
            } else if char.to_string() == "九" {
                total += 9;
            } else if char.to_string() == "十" {
                total += 10;
            } else {
                total += self.stroke_map.get(&char.to_string()).unwrap_or(&0);
            }
        }
        self.get_final_number(word, total)
    }

    pub fn get_final_number(&self, word: String, number: i64) -> i64 {
        let mut total: i64 = number;
        for char in word.chars() {
            if self.split_map.contains_key(&char.to_string()) {
                let splits = self.split_map.get(&char.to_string()).unwrap();
                if splits.contains(&String::from("氵")) {
                    total += 1;
                }
                if splits.contains(&String::from("扌")) {
                    total += 1;
                }
                if splits[0] == "月" {
                    total += 2;
                }
                if splits.contains(&String::from("艹")) {
                    total += 3;
                }
                if splits.contains(&String::from("辶")) {
                    total += 4;
                }
                if splits[0] == "阜" {
                    total += 6;
                }
                if splits.contains(&String::from("邑")) && splits.contains(&String::from("阝")) {
                    total += 5;
                }
                if splits[0] == "玉" {
                    total += 1;
                }
                if splits[0] == "示" {
                    total += 1;
                }
                if splits[0] == "衣" {
                    total += 1;
                }
                if splits[0] == "心" {
                    total += 1;
                }
            }
        }
        total
    }
}
