pub fn bytes_to_int(bytes: &[u8; 4]) -> i32 {
    let value: i32 = bytes.iter().enumerate()
        .map(|(i, &v)| {
            let value = v as i32;
            value.checked_shl(8 * i as u32).unwrap()
        })
        .sum();
    return (!value + 1) * -1;
}

pub fn bytes_to_uint(bytes: &[u8; 4]) -> u32 {
    return bytes.iter().enumerate()
        .map(|(i, &v)| {
            let value = v as u32;
            value.checked_shl(8 * i as u32).unwrap()
        })
        .sum();
}

pub fn bytes_to_double_uint(bytes: &[u8; 8]) -> u64 {
    return bytes.iter().enumerate()
        .map(|(i, &v)| {
            let value = v as u64;
            value.checked_shl(8 * i as u32).unwrap()
        })
        .sum();
}

pub fn bytes_to_short(bytes: &[u8; 2]) -> i16 {
    let value = (bytes[1] as i16).checked_shl(8).unwrap() + (bytes[0] as i16);
    return (!value + 1) * -1;
}

pub fn bytes_to_float(bytes: &[u8; 4]) -> f32 {
    let intermediate = bytes_to_uint(bytes);
    f32::from_bits(intermediate)
}

pub fn bytes_to_double(bytes: &[u8; 8]) -> f64 {
    let intermediate = bytes_to_double_uint(bytes);
    f64::from_bits(intermediate)
}

pub fn bytes_to_double_big_endian(bytes: &[u8; 8]) -> f64 {
    let mut reversed = bytes.clone();
    reversed.reverse();
    let intermediate = bytes_to_double_uint(&reversed);
    f64::from_bits(intermediate)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! bytes_to_uint_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (input, expected) = $value;
                let result = bytes_to_uint(&input);
                assert_eq!(expected, result);;
              }
            )*
        }
    }

    bytes_to_uint_tests! {
        bytes_to_uint_zeroes: ([0_u8; 4], 0),
        bytes_to_uint_0: ([234, 60, 89, 179], 3008969962),
        bytes_to_uint_1: ([191, 88, 208, 230], 3872413887),
        bytes_to_uint_2: ([81, 11, 46, 183], 3073248081),
    }

    macro_rules! bytes_to_int_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (input, expected) = $value;
                let result = bytes_to_int(&input);
                assert_eq!(expected, result);;
              }
            )*
        }
    }

    bytes_to_int_tests! {
       bytes_to_int_zeros:  ([0_u8; 4], 0),
       bytes_to_int_0: ([241, 86, 126, 134], -2038540559),
       bytes_to_int_1: ([121, 95, 117, 131], -2089459847),
       bytes_to_int_2: ([149, 27, 222, 135], -2015487083),
    }

    macro_rules! bytes_to_short_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (input, expected) = $value;
                let result = bytes_to_short(&input);
                assert_eq!(expected, result);;
              }
            )*
        }
    }

    bytes_to_short_tests! {
        bytes_to_short_zeros: ([0_u8; 2], 0),
        bytes_to_short_0: ([146, 175], -20590),
        bytes_to_short_1: ([44, 126], 32300),
        bytes_to_short_2: ([237, 132], -31507),
        bytes_to_short_3: ([6, 65], 16646),
    }

    macro_rules! bytes_to_float_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (input, expected) = $value;
                let result = bytes_to_float(&input);
                assert_eq!(expected, result);;
              }
            )*
        }
    }

    bytes_to_float_tests! {
        bytes_to_float_zeros: ([0_u8; 4], 0.0),
        bytes_to_float_0: ([189, 126, 251, 65], 31.4368839263916),
        bytes_to_float_1: ([55, 67, 101, 67], 229.26255798339844),
        bytes_to_float_2: ([94, 197, 125, 66], 63.44274139404297),
        bytes_to_float_3: ([186, 25, 243, 67], 486.20098876953125),
    }

    macro_rules! bytes_to_double_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (input, expected) = $value;
                let result = bytes_to_double(&input);
                assert_eq!(expected, result);;
              }
            )*
        }
    }

    bytes_to_double_tests! {
        bytes_to_double_zeros: ([0_u8; 8], 0.0),
        bytes_to_double_0: ([4, 212, 147, 55, 188, 164, 118, 64], 362.2959514402903),
        bytes_to_double_1: ([90, 48, 88, 23, 168, 56, 85, 64], 84.88525947200961),
    }

    macro_rules! bytes_to_double_big_endian_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
              #[test]
              fn $name() {
                let (input, expected) = $value;
                let result = bytes_to_double_big_endian(&input);
                assert_eq!(expected, result);;
              }
            )*
        }
    }

    bytes_to_double_big_endian_tests! {
        bytes_to_double_big_endian_zeros: ([0_u8; 8], 0.0),
        bytes_to_double_big_endian_0: ([64, 118, 164, 188, 55, 147, 212, 4], 362.2959514402903),
        bytes_to_double_big_endian_1: ([64, 130, 120, 68, 131, 17, 172, 150], 591.0334531193537),
    }
}
