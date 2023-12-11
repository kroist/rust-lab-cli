fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_math() {
        assert_eq!(1 + 1, 3);
    }
}
