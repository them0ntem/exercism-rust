pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let yelling: bool = message.chars().any(|x| x.is_ascii_uppercase())
        && !message.chars().any(|x| x.is_ascii_lowercase());
    let question: bool = message.ends_with("?");

    match (yelling, question) {
        (true, true) => { "Calm down, I know what I'm doing!" }
        (false, true) => { "Sure." }
        (true, false) => { "Whoa, chill out!" }
        _ => { "Whatever." }
    }

}