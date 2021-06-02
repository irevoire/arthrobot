use serenity::utils::Colour;

/// since a lot of discord userId are close we are going to sha1 their ID to generate really
/// different color
pub fn from_u64(input: &u64) -> Colour {
    let mut m = sha1::Sha1::new();
    m.update(&input.to_ne_bytes());
    let [r, g, b, ..] = m.digest().bytes();

    Colour::from_rgb(r, g, b)
}
