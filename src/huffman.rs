mod frequency;

use frequency::get_frequencies;
use std::io::{self, Read, Write};

pub fn compress(input: impl Read, outup: impl Write) -> io::Result<()> {
    let freq = get_frequencies(input)?;

    Ok(())
}
