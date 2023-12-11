use std::fs::read_to_string;
use std::path::Path;

pub fn head(path: &Path, n: usize) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .take(n)
        .collect()
}

pub fn tail(path: &Path, n: usize) -> Vec<String> {
    let all_strs: Vec<String> = read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    all_strs
        .iter()
        .rev()
        .take(n)
        .map(String::from)
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    #[test]
    fn test_head_3_from_5() {
        assert_eq!(
            crate::head(Path::new("./test/test1.txt"), 3 as usize),
            vec!["abacaba", "abac", "hubabuba"]
        )
    }
    #[test]
    fn test_head_6_from_5() {
        assert_eq!(
            crate::head(Path::new("./test/test1.txt"), 6 as usize),
            vec!["abacaba", "abac", "hubabuba", "bububa", "lulalula",]
        )
    }

    #[test]
    fn test_tail_3_from_5() {
        assert_eq!(
            crate::tail(Path::new("./test/test1.txt"), 3 as usize),
            vec!["hubabuba", "bububa", "lulalula",]
        )
    }
    #[test]
    fn test_tail_6_from_5() {
        assert_eq!(
            crate::tail(Path::new("./test/test1.txt"), 6 as usize),
            vec!["abacaba", "abac", "hubabuba", "bububa", "lulalula",]
        )
    }
}
