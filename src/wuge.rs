use opencc_rs::{Config, OpenCC};

use crate::stroke_number::Stroke;

pub struct Wuge<'a> {
    pub stroke_goods: Vec<i64>,
    pub stroke_generals: Vec<i64>,
    pub stroke_bads: Vec<i64>,

    pub wuxing_goods: Vec<&'a str>,
    pub wuxing_generals: Vec<&'a str>,
    pub wuxing_bads: Vec<&'a str>,

    pub stroke_list: Vec<[i64; 2]>,
}

impl<'a> Wuge<'a> {
    pub fn new() -> Self {
        Wuge {
            stroke_goods: vec![
                1, 3, 5, 6, 7, 8, 11, 13, 15, 16, 17, 18, 21, 23, 24, 25, 29, 31, 32, 33, 35, 37,
                39, 41, 45, 47, 48, 52, 57, 61, 63, 65, 67, 68, 81,
            ],
            stroke_generals: vec![27, 38, 42, 55, 58, 71, 72, 73, 77, 78],
            stroke_bads: vec![
                2, 4, 9, 10, 12, 14, 19, 20, 22, 26, 28, 30, 34, 36, 40, 43, 44, 46, 49, 50, 51,
                53, 54, 56, 59, 60, 62, 64, 66, 69, 70, 74, 75, 76, 79, 80,
            ],
            wuxing_goods: vec![
                "木木木",
                "木木火",
                "木木土",
                "木火木",
                "木火土",
                "木水木",
                "木水金",
                "木水水",
                "火木木",
                "火木火",
                "火木土",
                "火火木",
                "火火土",
                "火土火",
                "火土土",
                "火土金",
                "土火木",
                "土火火",
                "土火土",
                "土土火",
                "土土土",
                "土土金",
                "土金土",
                "土金金",
                "土金水",
                "金土火",
                "金土土",
                "金土金",
                "金金土",
                "金水木",
                "金水金",
                "水木木",
                "水木火",
                "水木土",
                "水木水",
                "水金土",
                "水金水",
                "水水木",
                "水水金",
            ],
            wuxing_generals: vec![
                "木火火",
                "木土火",
                "火木水",
                "火火火",
                "土木木",
                "土木火",
                "土土木",
                "金土木",
                "金金金",
                "金金水",
                "金水水",
                "水火木",
                "水土火",
                "水土土",
                "水土金",
                "水金金",
                "水水水",
            ],
            wuxing_bads: vec![
                "木木金",
                "木火金",
                "木火水",
                "木土木",
                "木土水",
                "木金木",
                "木金火",
                "木金土",
                "木金金",
                "木金水",
                "木水火",
                "木水土",
                "火木金",
                "火火金",
                "火火水",
                "火金木",
                "火金火",
                "火金金",
                "火金水",
                "火水木",
                "火水火",
                "火水土",
                "火水金",
                "火水水",
                "土木土",
                "土木金",
                "土木水",
                "土火水",
                "土土水",
                "土金木",
                "土金火",
                "土水木",
                "土水火",
                "土水土",
                "土水水",
                "金木木",
                "金木火",
                "金木土",
                "金木金",
                "金木水",
                "金火木",
                "金火金",
                "金火水",
                "金金木",
                "金金木",
                "金水火",
                "水木金",
                "水火火",
                "水火土",
                "水火金",
                "水火水",
                "水土木",
                "水水土",
                "水金木",
                "水金火",
                "水水火",
                "水水土",
                "木木水",
                "木土金",
                "火土木",
                "火土水",
                "土火金",
                "金土水",
                "火金土",
                "土水金",
                "金火火",
                "金火土",
                "木土土",
                "金水土",
            ],
            stroke_list: vec![],
        }
    }

    fn get_wuxing(&self, count: i64) -> String {
        let rest = count % 10;
        if rest == 1 || rest == 2 {
            return String::from("木");
        } else if rest == 3 || rest == 4 {
            return String::from("火");
        } else if rest == 5 || rest == 6 {
            return String::from("土");
        } else if rest == 7 || rest == 8 {
            return String::from("金");
        } else {
            return String::from("水");
        }
    }

    fn get_sancai_config(&self, counts: Vec<i64>) -> String {
        let mut c = String::from("");
        for count in counts {
            let wuxing = self.get_wuxing(count);
            c = format!("{}{}", c, wuxing);
        }
        c
    }

    fn get_stroke_type(&self, stroke: i64) -> String {
        if self.stroke_goods.contains(&stroke) {
            return String::from("大吉");
        } else if self.stroke_generals.contains(&stroke) {
            return String::from("中吉");
        } else if self.stroke_bads.contains(&stroke) {
            return String::from("凶");
        } else {
            return String::from("");
        }
    }

    fn get_sancai_type(&self, config: String) -> String {
        if self.wuxing_goods.contains(&config.as_str()) {
            return String::from("大吉");
        } else if self.wuxing_generals.contains(&config.as_str()) {
            return String::from("中吉");
        } else if self.wuxing_bads.contains(&config.as_str()) {
            return String::from("凶");
        } else {
            return String::from("");
        }
    }

    fn check_sancai_good(&self, counts: Vec<i64>, allow_general: bool) -> bool {
        let c = self.get_sancai_config(counts);
        if self.wuxing_goods.contains(&c.as_str()) {
            return true;
        } else if allow_general && self.wuxing_generals.contains(&c.as_str()) {
            return true;
        } else {
            return false;
        }
    }

    pub fn check_wuge_config (&self,name: String) {
        if name.len() != 3 {
            return;
        }
        let opencc = OpenCC::new([Config::S2T]).unwrap();
        let converter_name = opencc.convert(name).unwrap();
        let complex_name: Vec<char> = converter_name.chars().collect();
        let stroke_name = Stroke::new();
        let xing = stroke_name.get_stroke_number(complex_name.get(0).unwrap().to_string());
        let ming1 = stroke_name.get_stroke_number(complex_name.get(1).unwrap().to_string());
        let ming2 = stroke_name.get_stroke_number(complex_name.get(2).unwrap().to_string());

        let tian = xing + 1;
        let ren = xing + ming1;
        let di = ming1 + ming2;
        let zong = xing + ming1 + ming2;
        let wai = zong - ren + 1;
        
        let sancai_config = self.get_sancai_config(vec![tian,ren,di]);

        // println!("{}",name);
        println!("{} {} {} {}",converter_name,xing,ming1,ming2);
        println!("天格\t{}",tian.to_string());
        println!("人格\t{}\t{}",ren.to_string(),self.get_stroke_type(ren));
        println!("地格\t{}\t{}",di.to_string(),self.get_stroke_type(di));
        println!("总格\t{}\t{}",zong.to_string(),self.get_stroke_type(zong));
        println!("外格\t{}\t{}",wai.to_string(),self.get_stroke_type(wai));
        println!("三才\t{}\t{}",sancai_config,self.get_sancai_type(sancai_config.clone()));


    }

    pub fn get_stroke_list(&mut self, last_name: String, allow_general: bool) {
        let opencc = OpenCC::new([Config::S2T]).unwrap();
        let converter_last_name = opencc.convert(last_name).unwrap();
        let stroke_name = Stroke::new();
        let n = stroke_name.get_stroke_number(converter_last_name);
        for i in 1..81 {
            for j in 1..81 {
                let tian = n + 1;
                let ren = n + j;
                let di = i + j;
                let zong = n + j + i;
                let wai = zong - ren + 1;

                if self.stroke_goods.contains(&ren)
                    && self.stroke_goods.contains(&di)
                    && self.stroke_goods.contains(&zong)
                    && self.stroke_goods.contains(&wai)
                {
                    if self.check_sancai_good(vec![tian, ren, di], allow_general) {
                        self.stroke_list.push([i, j])
                    }
                } else if allow_general
                    && (self.stroke_goods.contains(&ren) || self.stroke_generals.contains(&ren))
                    && (self.stroke_goods.contains(&di) || self.stroke_generals.contains(&di))
                    && (self.stroke_goods.contains(&zong) || self.stroke_generals.contains(&zong))
                    && (self.stroke_goods.contains(&wai) || self.stroke_generals.contains(&wai))
                {
                    if self.check_sancai_good(vec![tian, ren, di], allow_general) {
                        self.stroke_list.push([i, j])
                    }
                }
            }
        }
    }
}
