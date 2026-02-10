/// `n` から始まるコラッツ数列の長さを決定。
fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    length
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
    assert_eq!(collatz_length(3), 8);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}