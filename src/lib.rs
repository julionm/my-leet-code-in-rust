#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_with_emptyspace() {
        let s = String::from(" ");

        assert_eq!(
            1,
            length_of_longest_substring(s)
        )
    }

    #[test]
    fn test_with_longest_in_the_middle() {
        let s = String::from("dvdf");
        assert_eq!(
            3,
            length_of_longest_substring(s)
        )
    }

    #[test]
    fn test_with_longest_in_the_final() {
        let s = String::from("pwwkew");
        assert_eq!(
            3,
            length_of_longest_substring(s)
        )
    }

    #[test]
    fn test_with_longest_in_the_start() {
        let s = String::from("abcabcbb");
        assert_eq!(
            3,
            length_of_longest_substring(s)
        )
    }

    #[test]
    fn test_with_same_letter() {
        let s = String::from("bbbbb");
        assert_eq!(
            1,
            length_of_longest_substring(s)
        )
    }

}



fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let mut longest_substring: usize = 0;
    let mut current_substring: String = String::new();
    let mut initial_index: Option<usize> = None;

    for (final_index, ch) in s.chars().enumerate() {

        let duplicated_index = current_substring.find(|a| a == ch);

        if let Some(dupli_idx) = duplicated_index {
            if let Some(init_index) = initial_index {
                if (final_index - init_index) > longest_substring {
                    longest_substring = current_substring.len();
                }
                let spliteds = current_substring.split(ch);

                initial_index = Some(initial_index.unwrap() + dupli_idx + 1);
                current_substring = String::from(spliteds.last().unwrap());
                current_substring.push(ch);
            }

        } else {
            if let None = initial_index {
                initial_index = Some(final_index);
            }
            current_substring.push(ch);
        }
    }

    if current_substring.len() > longest_substring {
        current_substring.len() as i32
    } else {
        longest_substring as i32
    }

}


fn reverse_integer(num: i32) -> i32 {

    let mut aux = num.to_be_bytes();
    aux.reverse();

    let bytes = Vec::from(aux);


    2
}
