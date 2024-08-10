mod fuzz_emoji;

fn main() {
    let fuzzer = fuzz_emoji::FuzzEmoji::new();
    let descriptions = vec!["flame", "flo"];
    let result = fuzzer.get_emojis(descriptions);
    for (k, v) in result {
        println!("{}: {}", k, v);
    }
    Ok(())
}
