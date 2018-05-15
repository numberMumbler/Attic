const COLLIDER_0: &[u8] = &[14, 48, 101, 97, 85, 154, 167, 135, 208, 11, 198, 247, 11, 189, 254, 52, 4, 207, 3, 101, 158, 112, 79, 133, 52, 192, 15, 251, 101, 156, 76, 135, 64, 204, 148, 47, 235, 45, 161, 21, 163, 244, 21, 92, 187, 134, 7, 73, 115, 134, 101, 109, 125, 31, 52, 164, 32, 89, 215, 143, 90, 141, 209, 239];
const COLLIDER_1: &[u8] = &[14, 48, 101, 97, 85, 154, 167, 135, 208, 11, 198, 247, 11, 189, 254, 52, 4, 207, 3, 101, 158, 116, 79, 133, 52, 192, 15, 251, 101, 156, 76, 135, 64, 204, 148, 47, 235, 45, 161, 21, 163, 244, 21, 220, 187, 134, 7, 73, 115, 134, 101, 109, 125, 31, 52, 164, 32, 89, 215, 143, 90, 141, 209, 239];


pub fn generate_colliders(suffix: &[u8]) -> (Vec<u8>, Vec<u8>) {
    ([COLLIDER_0, suffix].concat(), [COLLIDER_1, suffix].concat())
}

#[cfg(test)]
mod tests {
    use crypto::md5::Md5;
    use crypto::digest::Digest;
    use super::*;

    macro_rules! collision_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let input = $value;

                let (result_1, result_2) = generate_colliders(&input.as_bytes());

                let hash_1 = get_hash(result_1.as_slice());
                let hash_2 = get_hash(result_2.as_slice());
                assert_eq!(hash_1, hash_2);
              }
            )*
        }
    }

    collision_tests! {
        collision_0: "f4e97c930597f11cc9e5e22642ec16e5".to_string(),
    }

    macro_rules! result_contains_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let input_bytes = $value;

                let (result_1, result_2) = generate_colliders(&input_bytes);

                assert!(contains(&result_1, &input_bytes));
                assert!(contains(&result_2, &input_bytes));
              }
            )*
        }
    }

    result_contains_tests! {
        result_contains_0: "f4e97c930597f11cc9e5e22642ec16e5".as_bytes(),
    }

    fn get_hash(content: &[u8]) -> String {
        let mut hasher = Md5::new();
        hasher.input(&content);
        let result = hasher.result_str();

        return result;
    }

    #[test]
    fn get_hash_test() {
        let input = "d131dd02c5e6eec4693d9a0698aff95c2fcab58712467eab4004583eb8fb7f89 \n55ad340609f4b30283e488832571415a085125e8f7cdc99fd91dbdf280373c5b \nd8823e3156348f5bae6dacd436c919c6dd53e2b487da03fd02396306d248cda0 \ne99f33420f577ee8ce54b67080a80d1ec69821bcb6a8839396f9652b6ff72a70".to_string();
        let expected = "88da47c0d00f47f227e4ab2ad2bd8149".to_string();

        let result = get_hash(&input.as_bytes());

        assert_eq!(expected, result);
    }

    fn contains(haystack: &[u8], needle: &[u8]) -> bool {
        if haystack.len() == 0 {
            return false;
        }
        if needle.len() == 0 {
            return true;
        }
        let mut needle_index: usize = 0;
        for i in 0..haystack.len() {
            if haystack[i] == needle[needle_index] {
                needle_index += 1;
            } else if haystack[i] == needle[0] {
                needle_index = 1;
            } else {
                needle_index = 0;
            }

            if needle_index == needle.len() {
                return true;
            }
        }
        return false;
    }

    macro_rules! contains_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (haystack, needle, expected) = $value;
                let result = contains(&haystack.as_bytes(), &needle.as_bytes());
                assert_eq!(expected, result);
              }
            )*
        }
    }

    contains_tests! {
        contains_empty_haystack: ("", "asdf", false),
        contains_empty_needle: ("asdf", "", true),
        contains_0: ("aaaab", "aaaaa", false),
        contains_2: ("abcdefghijklmnop", "k", true),
        contains_1: ("abcdefghijklmnop", "fghi", true),
    }
}