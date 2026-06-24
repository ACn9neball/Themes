use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub fn main() -> Vec<String> {
    let mut color_vec: Vec<String> = vec![];
    for _ in 0..21 {
        color_vec.push("*".to_string());
    }
    let name = "theme";
    let config = "colors.conf";
    let mut config_path = dirs::config_dir().ok_or("No system config file").unwrap();
    config_path.push(name);
    fs::create_dir_all(&config_path).unwrap();
    config_path.push(config);

    let file = File::open(&config_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let values = seperator(line.unwrap());
        match values[0].as_str() {
            "foreground" => color_vec[0] = values[1].clone(),
            "background" => color_vec[1] = values[1].clone(),
            "cursor" => color_vec[2] = values[1].clone(),
            "selection_foreground" => color_vec[3] = values[1].clone(),
            "selection_background" => color_vec[4] = values[1].clone(),
            "color0" => color_vec[5] = values[1].clone(),
            "color1" => color_vec[6] = values[1].clone(),
            "color2" => color_vec[7] = values[1].clone(),
            "color3" => color_vec[8] = values[1].clone(),
            "color4" => color_vec[9] = values[1].clone(),
            "color5" => color_vec[10] = values[1].clone(),
            "color6" => color_vec[11] = values[1].clone(),
            "color7" => color_vec[12] = values[1].clone(),
            "color8" => color_vec[13] = values[1].clone(),
            "color9" => color_vec[14] = values[1].clone(),
            "color10" => color_vec[15] = values[1].clone(),
            "color11" => color_vec[16] = values[1].clone(),
            "color12" => color_vec[17] = values[1].clone(),
            "color13" => color_vec[18] = values[1].clone(),
            "color14" => color_vec[19] = values[1].clone(),
            "color15" => color_vec[20] = values[1].clone(),
            _ => {}
        }
    }

    color_vec
}

fn seperator(line: String) -> Vec<String> {
    let values: Vec<&str> = line.split_whitespace().collect();
    let mut s: Vec<String> = vec![];
    for value in values {
        s.push(value.trim().to_string());
    }
    s
}
