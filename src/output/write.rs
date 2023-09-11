use std::io::Write;

use anyhow::{Context, Result};

use crate::fetch_availability::Slot;

pub fn write_slots<W>(slots: &[&Slot], writer: &mut W) -> Result<()>
where
    W: Write,
{
    for (index, slot) in slots.iter().enumerate() {
        writer
            .write_all(slot.to_string().as_bytes())
            .context("Failed to write to output")?;
        if index < slots.len() - 1 {
            writer
                .write_all(b"\n")
                .context("Failed to write to output")?;
        }
    }

    Ok(())
}
