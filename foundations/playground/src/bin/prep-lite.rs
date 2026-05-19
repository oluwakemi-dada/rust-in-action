fn str_simple_pattern() {
    let search_term = "picture";
    let quote = "\
      Every face, every shop, bedroom window, public-house, and
      dark square is a picture feverishly turned--in search of what?
      It is the same with books.
      What do we seek through millions of pages?
    ";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}

fn simple_with_linenums() {
    let search_term = "picture";
    let quote = "\
      Every face, every shop, bedroom window, public-house, and
      dark square is a picture feverishly turned--in search of what?
      It is the same with books.
      What do we seek through millions of pages?
    ";

    let mut line_num: usize = 1;

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
        line_num += 1;
    }
}

fn simple_with_enumerate() {
    let search_term = "picture";
    let quote = "\
      Every face, every shop, bedroom window, public-house, and
      dark square is a picture feverishly turned--in search of what?
      It is the same with books.
      What do we seek through millions of pages?
    ";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

fn main() {
    str_simple_pattern();
    simple_with_linenums();
    simple_with_enumerate();
}
