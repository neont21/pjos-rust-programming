use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("first: {}", pig_latin(String::from("first")));
    println!("apple: {}", pig_latin(String::from("apple")));
}

fn pig_latin(origin: String) -> String {
    if origin.len() == 0 {
        return origin;
    }

    let origin = UnicodeSegmentation::graphemes(&origin[..], true)
        .collect::<Vec<&str>>();

    let mut temp = "";
    let mut result = String::new();

    for (i, ch) in origin.iter().enumerate() {
        if i == 0 {
            temp = ch;
        } else {
            result = result + ch;
        }
    }

    if temp == "a" || temp == "e" || temp == "i" || temp == "o" || temp == "u" {
        result = format!("{}{}-hay", temp, result);
    } else {
        result = format!("{}-{}ay", result, temp);
    }

    result
}