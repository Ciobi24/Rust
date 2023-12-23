
/// Encodes a byte slice to base64 string.
///
/// # Arguments
///
/// * `input` - A byte slice to be encoded.
///
/// # Returns
///
/// Returns a base64-encoded string.
///
/// # Examples
///
/// ```
/// use base64::encode;
///
/// let input = b"Hello, World!";
/// let encoded = encode(input);
/// assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
/// ```
pub fn encode(input: &[u8]) -> String {
    const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut index = 0;

    while index < input.len() {
        let first = input[index];
        let second = if index + 1 < input.len() { input[index + 1] } else { 0 };
        let third = if index + 2 < input.len() { input[index + 2] } else { 0 };

        result.push(BASE64_CHARS[(first >> 2) as usize] as char);
        result.push(BASE64_CHARS[((first & 0b11) << 4 | (second >> 4)) as usize] as char);

        if index + 1 < input.len() {
            result.push(BASE64_CHARS[((second & 0b1111) << 2 | (third >> 6)) as usize] as char);
        } else {
            result.push('=');
        }

        if index + 2 < input.len() {
            result.push(BASE64_CHARS[(third & 0b111111) as usize] as char);
        } else {
            result.push('=');
        }

        index += 3;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(b""), "");
    }

    #[test]
    fn test_encode_padding() {
        assert_eq!(encode(b"abc"), "YWJj");
        assert_eq!(encode(b"abcd"), "YWJjZA==");
        assert_eq!(encode(b"abcde"), "YWJjZGU=");
        assert_eq!(encode(b"abcdef"), "YWJjZGVm");
    }

    #[test]
    fn test_encode_no_padding() {
        assert_eq!(encode(b"abcdefg"), "YWJjZGVmZw==");
        assert_eq!(encode(b"abcdefgh"), "YWJjZGVmZ2g=");
        assert_eq!(encode(b"abcdefghi"), "YWJjZGVmZ2hp");
    }
}
