use std::str;

pub fn add(a: i16, b: i16) -> i16 {
    a + b
}

pub fn decipher(cipher: &str, shift: u8) -> String {
    let final_shift = shift % 26;
    let mut tmp_str = String::new();
    for chr in cipher.as_bytes() {
        if chr >= &65u8 && chr <= &90u8 {
            let final_char = chr - final_shift;
            if final_char < 65u8 {
                tmp_str.push_str(str::from_utf8(&[final_char + 90u8 - 65u8 + 1]).unwrap());
            }
            else {
                tmp_str.push_str(str::from_utf8(&[final_char]).unwrap());
            }
        }
        else {
            tmp_str.push_str(str::from_utf8(&[*chr]).unwrap());
        }
    }
    tmp_str.to_string()
}

pub fn cipher(cipher: &str, shift: u8) -> String {
    let final_shift = shift % 26;
    let mut tmp_str = String::new();
    for chr in cipher.as_bytes() {
        if chr >= &65u8 && chr <= &90u8 {
            let final_char = chr + final_shift;
            if final_char > 90u8 {
                tmp_str.push_str(str::from_utf8(&[final_char - 90u8 + 65u8 - 1]).unwrap());
            }
            else {
                tmp_str.push_str(str::from_utf8(&[final_char]).unwrap());
            }
        } else {
            tmp_str.push_str(str::from_utf8(&[*chr]).unwrap());
        }
    }
    tmp_str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_decipher() {
        let decipher_string="VWLFNV LQ D EXQGOH DUH XQEUHDNDEOH.";

        assert_eq!("STICKS IN A BUNDLE ARE UNBREAKABLE.", decipher(decipher_string, 3));
    }

    #[test]
    fn test_cipher() {
        let cipher_string = "STICKS IN A BUNDLE ARE UNBREAKABLE.";

        assert_eq!("VWLFNV LQ D EXQGOH DUH XQEUHDNDEOH.", cipher(cipher_string, 3));
    }

    #[test]
    fn test_cipher_quick_brown_fox_shift_26() {
        let cipher_string = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.";

        assert_eq!("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.", cipher(cipher_string, 26));
    }

    #[test]
    fn test_cipher_quick_brown_fox_shift_15() {
        let cipher_string = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.";

        assert_eq!("IWT FJXRZ QGDLC UDM YJBEH DKTG IWT APON SDV.", cipher(cipher_string, 15));
    }

    #[test]
    fn test_cipher_shift_3() {
        let cipher_string = "A.";

        assert_eq!("D.", cipher(cipher_string, 3));
    }
}

