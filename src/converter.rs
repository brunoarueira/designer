use color_thief::Color;
use failure::Error;

fn map_rgb<'a>(match_number: &'a usize) -> &'a str {
    let match_str: &'a str = match *match_number {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "A",
        11 => "B",
        12 => "C",
        13 => "D",
        14 => "E",
        15 => "F",
        _ => panic!("[designer] map_rgb not match match_number value."),
    };

    match_str
}

fn round(value: f32, digit: u32) -> f32 {
    let a = 10usize.pow(digit) as f32;

    (value * a).round() / a
}

fn to_number(value: &str) -> f32 {
    match value.parse::<f32>() {
        Ok(value) => value / 255f32,
        Err(_error) => 0f32,
    }
}

fn rgb(color: &Color) -> Result<Vec<f32>, Error> {
    let r = to_number(&color.r.to_string());
    let g = to_number(&color.g.to_string());
    let b = to_number(&color.b.to_string());

    Ok(vec![round(r, 4), round(g, 4), round(b, 4)])
}

pub fn rgb2hex(color: &Color) -> Result<String, Error> {
    let rgb = rgb(&color)?;
    let mut value: Vec<f32> = vec![];
    let mut hex = String::from("#");

    for n in &rgb {
        value.push(n * 255f32);
    }

    for color_part in value.iter().take(3usize) {
        if *color_part == 0f32 {
            hex.push_str("00");

            continue;
        }

        let low = (color_part / 16f32) as usize;
        let high = (color_part % 16f32) as usize;
        hex.push_str(map_rgb(&low));
        hex.push_str(map_rgb(&high));
    }

    Ok(hex.to_uppercase())
}
