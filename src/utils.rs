use crate::api::{Quotes, QuotesInner};

use crossterm::{
    cursor::{RestorePosition, SavePosition},
    execute,
};
use viuer::{print as print_image, Config};
use crate::{MAX_LINES_FOR_QUOTE, WIDTH, HEIGHT};
use crate::images::Images;

pub fn print_image_and_quote(
    quote: &QuotesInner,
    width: usize,
    viuer_conf: &Config,
) {
    let mut stdout = std::io::stdout();
    let images = Images::new();
    let quotes = quote_to_vec(&quote.quote, width);

    println!(""); // First blank line

    //////////////////////////////////////////////////////////////////
    execute!(&mut stdout, SavePosition).unwrap();

    print!("\n\n");
    let mut lines_printed = 2;
    for line in quotes.iter() {
        println!("{}{}", " ".repeat(WIDTH as usize + 5), line);
        lines_printed += 1;
    }

    println!("{:^width$}", quote.character, width = quotes.first().unwrap().len() + 25);
    lines_printed += 1;

    for _ in 0..HEIGHT - lines_printed {
        println!("");
    }

    execute!(&mut stdout, RestorePosition).unwrap();
    //////////////////////////////////////////////////////////////////

    //////////////////////////////////////////////////////////////////
    let image = images.fetch(&quote.nation);
    print_image(&image, viuer_conf).expect("Image printing failed.");
    //////////////////////////////////////////////////////////////////

    println!(""); // Last blank line
}

pub fn get_quote_and_width(quotes: &Quotes) -> (&QuotesInner, usize) {
    let term_width = term_size::dimensions().map(|(w, _)| w);
    let default_width = Some(80);
    let width = term_width.or(default_width).unwrap();

    for quote in quotes.quotes.iter() {
        if (quote.quote.len() / width) <= MAX_LINES_FOR_QUOTE {
            return (quote, width);
        }
    }
    (quotes.quotes.first().unwrap(), width)
}

fn quote_to_vec(quote: &str, width: usize) -> Vec<String> {
    if width < 25 {
        return vec![];
    }

    textwrap::wrap(&quote, width - 25)
        .iter()
        .map(|line| line.to_string())
        .collect()
}
