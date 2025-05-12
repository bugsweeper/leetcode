// Last updated: 12.05.2025, 16:15:53
use std::collections::HashSet;

fn clear_email(email: String) -> Vec<u8> {
    let email = email.as_bytes();
    let mut result = Vec::with_capacity(email.len());
    let mut stopped_at_index = 0;
    for (index, &byte) in email.iter().enumerate() {
        match byte {
            b'a'..b'z' => result.push(byte),
            b'+' | b'@' => {
                stopped_at_index = index;
                break;
            }
            _ => {}
        }
    }
    let at_index = email[stopped_at_index..].iter().position(|byte| *byte == b'@').unwrap() + stopped_at_index;
    result.extend_from_slice(&email[at_index..]);
    result
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails.into_iter().map(clear_email).collect::<HashSet<_>>().len() as i32
    }
}