use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "svg file")]
    svg: PathBuf,
    #[arg(value_name = "jsx location")]
    jsx: PathBuf,
    #[arg(short, value_name = "name of jsx file")]
    name: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let name = match cli.name {
        Some(val) => val,
        None => cli
            .jsx
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Icon")
            .to_string(),
    };
    //Read SVG from file
    let mut svg = File::open(cli.svg)?;

    //Read content of SVG into mutable string
    let mut contents = String::new();
    svg.read_to_string(&mut contents)?;

    //Parse SVG string to match jsx and wrap in function
    let parsed_svg = parse_svg(&contents);
    let function = wrap_svg(&parsed_svg, &name);

    //Write JSX back to target file
    let mut file = File::create(cli.jsx)?;
    file.write_all(function.as_bytes())?;
    Ok(())
}

fn wrap_svg(svg: &str, name: &str) -> String {
    format!(
        "export function {}() {{
return(
{});
}}
",
        name, svg
    )
}

fn parse_svg(svg: &str) -> String {
    let mut res = String::new();
    let mut was_dash = false;

    for c in svg.chars() {
        if was_dash {
            res += &c.to_uppercase().to_string();
            was_dash = false;
            continue;
        }
        if c == '-' {
            was_dash = true;
        } else {
            res += &c.to_string();
        }
    }
    res
}
