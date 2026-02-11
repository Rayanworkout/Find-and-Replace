use anyhow::{Context, Result};

pub fn parse_select(select: Option<Vec<String>>) -> Result<Option<Vec<usize>>> {
    let Some(tokens) = select else {
        return Ok(None);
    };

    let mut indices = Vec::new();

    for raw in tokens {
        let token = raw.trim();
        if token.is_empty() {
            continue;
        }

        if let Some((start_raw, end_raw)) = token.split_once('-') {
            let start: usize = start_raw
                .trim()
                .parse()
                .with_context(|| format!("Invalid --select range start: '{start_raw}'"))?;
            let end: usize = end_raw
                .trim()
                .parse()
                .with_context(|| format!("Invalid --select range end: '{end_raw}'"))?;

            if start == 0 || end == 0 {
                anyhow::bail!("--select indices are 1-based, got '{token}'");
            }
            if start > end {
                anyhow::bail!("Invalid --select range '{token}': start must be <= end");
            }

            indices.extend(start..=end);
            continue;
        }

        let index: usize = token
            .parse()
            .with_context(|| format!("Invalid --select value: '{token}'"))?;
        if index == 0 {
            anyhow::bail!("--select indices are 1-based, got '{token}'");
        }
        indices.push(index);
    }

    if indices.is_empty() {
        return Ok(None);
    }

    Ok(Some(indices))
}
