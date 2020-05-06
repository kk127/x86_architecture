pub fn convert_u8_twocomplement(num: u8) -> i8 {
    let binary_num = format!("{:08x}", num);
    let str_binary = binary_num.chars().collect::<Vec<char>>();
    if str_binary[0] == '0' {
        num as i8
    } else {
        let inversed_bit = !num; 
        - ((inversed_bit + 1) as i8)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_convert_u8_twocomplement() {
        assert_eq!(convert_u8_twocomplement(  0),   0);
        assert_eq!(convert_u8_twocomplement(127),  127);
        assert_eq!(convert_u8_twocomplement(128), -128);
        assert_eq!(convert_u8_twocomplement(249),   -7);
    }
}