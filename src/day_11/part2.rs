use anyhow::Result;

fn parse(input: &str) -> Result<Vec<i64>> {
    Ok(Vec::from([0]))
}

pub fn main(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let input = "";
        let result = main(input);
        assert_eq!(result.unwrap(), 36);
    }
}
