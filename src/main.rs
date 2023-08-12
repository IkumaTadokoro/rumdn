use regex::Regex;
use std::env;
use std::fs;
use std::io::Write;

fn parse_line(line: &str) -> String {
    if line.trim().is_empty() {
        return line.to_string();
    }

    if line.starts_with("#") {
        let content = &line[1..].trim();
        return format!("<h1>{}</h1>", content);
    }

    let re = Regex::new(r"\*([^*]+)\*").unwrap();
    let emphasized = re.replace_all(line, "<em>$1</em>");
    format!("<p>{}</p>", emphasized)
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("# This is a header"),
        "<h1>This is a header</h1>"
    );
    assert_eq!(
        parse_line("This is *emphasized* text"),
        "<p>This is <em>emphasized</em> text</p>"
    );
    assert_eq!(
        parse_line("This is *emphasized* text with *more* emphasis"),
        "<p>This is <em>emphasized</em> text with <em>more</em> emphasis</p>"
    );
    assert_eq!(
        parse_line("This is normal text"),
        "<p>This is normal text</p>"
    );
    assert_eq!(parse_line(""), "")
}

fn parse_markdown(markdown: &str) -> String {
    markdown
        .lines() // https://doc.rust-lang.org/std/primitive.str.html#method.lines
        .map(|line| parse_line(line))
        .collect::<Vec<String>>()
        .join("\n")
}

#[test]
fn test_parse_markdown() {
    assert_eq!(
        parse_markdown("# This is a header\nThis is *emphasized* text"),
        "<h1>This is a header</h1>\n<p>This is <em>emphasized</em> text</p>"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("使用方法: {} [入力ファイル名] [出力ファイル名]", args[0]);
        return;
    }
    let input_filename = &args[1];
    let output_filename = &args[2];
    let content = fs::read_to_string(input_filename).expect("ファイルの読み込みに失敗しました");
    let converted = parse_markdown(&content);
    let mut file = fs::File::create(output_filename).expect("出力ファイルの作成に失敗しました");
    file.write_all(converted.as_bytes())
        .expect("ファイルへの書き込みに失敗しました");
}
