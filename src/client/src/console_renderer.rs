use keyrock_challenge_proto::orderbook::Summary;
use std::io::{Write, StdoutLock};
use colored::Colorize;

fn clear_console(lock: &mut StdoutLock) {
    let _ = write!(lock, "{esc}c", esc = 27 as char);
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

fn render_spread(lock: &mut StdoutLock, spread: f64) {
    let _ = writeln!(lock, "{} {}", "Spread:".bold(), spread);
}

fn render_spread_padding(lock: &mut StdoutLock) {
    for _ in 0..40 {
        let _ = write!(lock, " ");
    }
}

fn render_table(lock: &mut StdoutLock, summary: &Summary) {
    let price_padding_size = 14;
    let amount_padding_size = 14;

    let mut price_header = String::new();
    let mut amount_header = String::new();
    pad_right("PRICE", &mut price_header, price_padding_size + 1, ' ');
    pad_right("AMOUNT", &mut amount_header, amount_padding_size, ' ');

    render_spread_padding(lock);
    let _ = writeln!(
        lock,
        "{}{}{}",
        price_header.bold(),
        amount_header.bold(),
        "EXCHANGE".bold()
    );

    let amount_asks = summary.asks.len();
    for index in 0..amount_asks {
        let ask = &summary.asks[amount_asks - index - 1];
        render_spread_padding(lock);
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
        let _ = write!(lock, "{} {} {}", price_padded.red(), amount_padded, ask.exchange);
        let _ = writeln!(lock);
    }

    render_spread(lock, summary.spread);

    for bid in &summary.bids {
        render_spread_padding(lock);
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
        let _ = write!(
            lock,
            "{} {} {}",
            price_padded.green(),
            amount_padded,
            bid.exchange
        );
        let _ = writeln!(lock);
    }
}

pub fn render(summary: Summary) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    clear_console(&mut lock);
    render_table(&mut lock, &summary);
}
