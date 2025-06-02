use table::*;

fn main() {
    let data = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["c".to_string(), "d".to_string()],
    ];
    
    let result = table(&data, None).unwrap();
    println!("Result:");
    println!("{}", result);
    println!("Length: {}", result.len());
    println!("Contains 'a': {}", result.contains("a"));
}