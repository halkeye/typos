use clap::Parser;

const DICT: &[u8] = include_bytes!("../../assets/words.csv");

fn generate<W: std::io::Write>(file: &mut W) {
    writeln!(
        file,
        "// This file is code-genned by {}",
        env!("CARGO_PKG_NAME")
    )
    .unwrap();
    writeln!(file, "#![allow(clippy::unreadable_literal)]",).unwrap();
    writeln!(file).unwrap();

    let records: Vec<_> = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(DICT)
        .records()
        .map(|r| r.unwrap())
        .collect();
    dictgen::generate_trie(
        file,
        "WORD",
        "&'static [&'static str]",
        records.iter().map(|record| {
            let mut record_fields = record.iter();
            let key = record_fields.next().unwrap();
            let value = format!(
                "&[{}]",
                itertools::join(record_fields.map(|field| format!(r#""{}""#, field)), ", ")
            );
            (key, value)
        }),
        64,
    )
    .unwrap();
}

#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    codegen: codegenrs::CodeGenArgs,
    #[clap(flatten)]
    rustmft: codegenrs::RustfmtArgs,
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let options = Options::parse();

    let mut content = vec![];
    generate(&mut content);

    let content = String::from_utf8(content)?;
    let content = options.rustmft.reformat(&content)?;
    options.codegen.write_str(&content)?;

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
