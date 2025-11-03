pub fn is_valid(code: &str) -> bool {
    // Step 1: Remove all spaces from the string
    let cleaned: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    // Step 2: Must have at least 2 characters
    if cleaned.len() <= 1 {
        return false;
    }

    // Step 3: If there’s any non-digit character, return false
    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Step 4: Convert to digits
    let digits: Vec<u32> = cleaned.chars().filter_map(|c| c.to_digit(10)).collect();

    // Step 5: Apply Luhn doubling rule
    let mut sum = 0;
    let mut double = false;

    for digit in digits.iter().rev() {
        let mut d = *digit;
        if double {
            d *= 2;
            if d > 9 {
                d -= 9;
            }
        }
        sum += d;
        double = !double; // flip true/false each time
    }

    // Step 6: Check if total divides by 10
    sum % 10 == 0
}
