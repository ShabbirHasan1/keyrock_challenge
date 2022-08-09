use keyrock_challenge_proto::orderbook::Summary;

use colored::Colorize;

fn clear_console() {
    print!("{esc}c", esc = 27 as char);
}

fn pad_right(original: &str, padded: &mut String, length: u8, character: char) {
    let mut index: u8 = 0;
    let chars = original.chars();

    let difference = length - chars.count() as u8;

    padded.push_str(original);

    while index < difference {
        padded.push(character);
        index += 1;
    }
}

fn render_spread(spread: f64) {
    println!("{} {}", "Spread:".bold(), spread);
}

fn render_spread_padding() {
    for _ in 0..40 {
        print!(" ");
    }
}

fn render_table(summary: &Summary) {
    let price_padding_size = 14;
    let amount_padding_size = 14;

    let mut price_header = String::new();
    let mut amount_header = String::new();
    pad_right("PRICE", &mut price_header, price_padding_size + 1, ' ');
    pad_right("AMOUNT", &mut amount_header, amount_padding_size, ' ');

    render_spread_padding();
    println!(
        "{}{}{}",
        price_header.bold(),
        amount_header.bold(),
        "EXCHANGE".bold()
    );

    for i in 0..9 {
        render_spread_padding();
        let ask = &summary.asks[9 - i];
        let mut price_padded = String::new();
        let mut amount_padded = String::new();
        pad_right(
            &ask.price.to_string(),
            &mut price_padded,
            price_padding_size,
            ' ',
        );
        pad_right(
            &ask.amount.to_string(),
            &mut amount_padded,
            amount_padding_size,
            ' ',
        );
        print!("{} {} {}", price_padded.red(), amount_padded, ask.exchange);
        println!();
    }

    render_spread(summary.spread);

    for i in 0..9 {
        render_spread_padding();
        let bid = &summary.bids[i];
        let mut price_padded = String::new();
        let mut amount_padded = String::new();
        pad_right(
            &bid.price.to_string(),
            &mut price_padded,
            price_padding_size,
            ' ',
        );
        pad_right(
            &bid.amount.to_string(),
            &mut amount_padded,
            amount_padding_size,
            ' ',
        );
        print!(
            "{} {} {}",
            price_padded.green(),
            amount_padded,
            bid.exchange
        );
        println!();
    }
}

pub fn render(summary: Summary) {
    clear_console();
    render_table(&summary);
}
