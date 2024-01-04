fn get_first_num(s: &str) -> i32 {
    for i in 0..s.len() {
        let substr = &s[i..s.len()];
        let num = get_number(substr);
        if num != -1 {
            return num;
        }
    }

    0
}

fn get_first_num_rev(s: &str) -> i32 {
    for i in 0..s.len() {
        let substr = &s[s.len() - 1 - i..s.len()];
        let num = get_number(substr);
        
        if num != -1 {
            return num;
        }
    }

    0
}

fn get_parameter_from_line(s: &str) -> i32 {
    let first = get_first_num(s);
    let last = get_first_num_rev(s);

    
    let mut res = first.to_string();
    res.push_str(&last.to_string());
    
    res.parse().unwrap()
}

fn is_str_number(s: &str, number_str: &str) -> bool {

    if s.starts_with(number_str) {
        return true;
    }

    false
}

fn is_numeric_number(s: &str, number_char: &char) -> bool {
    if s.len() == 0 {
        return false;
    }
    let c = s.chars().next().unwrap();

    if c.eq(number_char) {
        return true;
    }

    false
}

fn get_number(s: &str) -> i32 {
    if is_str_number(s, &"one") || is_numeric_number(s, &'1') {
        return 1;
    } else if is_str_number(s, &"two") || is_numeric_number(s, &'2') {
        return 2;
    } else if is_str_number(s, &"three") || is_numeric_number(s, &'3') {
        return 3;
    } else if is_str_number(s, &"four") || is_numeric_number(s, &'4') {
        return 4;
    } else if is_str_number(s, &"five") || is_numeric_number(s, &'5') {
        return 5;
    } else if is_str_number(s, &"six") || is_numeric_number(s, &'6') {
        return 6;
    } else if is_str_number(s, &"seven") || is_numeric_number(s, &'7') {
        return 7;
    } else if is_str_number(s, &"eight") || is_numeric_number(s, &'8') {
        return 8;
    } else if is_str_number(s, &"nine") || is_numeric_number(s, &'9') {
        return 9;
    } else {
        return -1;
    }
}

pub fn get_configuration_value(input: &str) -> i32 {
    let mut calibration_value:i32 = 0;

    for line in input.lines()  {
        let param = get_parameter_from_line(&line);

        calibration_value += param;
    }
    
    calibration_value
}
