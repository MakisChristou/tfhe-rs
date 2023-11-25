use crate::ciphertext::fheasciichar::FheAsciiChar;
use crate::ciphertext::fhesplit::FheSplit;
use crate::ciphertext::fhestrip::FheStrip;
use crate::client_key::MyClientKey;
use crate::server_key::MyServerKey;
use crate::string_method::StringMethod;
use crate::{PublicParameters, MAX_FIND_LENGTH, STRING_PADDING};

pub fn abs_difference(a: usize, b: usize) -> usize {
    a.checked_sub(b).unwrap_or(b - a)
}

pub fn bubble_zeroes_left(
    mut result: Vec<FheAsciiChar>,
    server_key: &tfhe::integer::ServerKey,
    public_parameters: &PublicParameters,
) -> Vec<FheAsciiChar> {
    let zero = FheAsciiChar::encrypt_trivial(0u8, public_parameters);

    // Bring non \0 characters in front O(n^2), essentially bubble sort
    for _ in 0..result.len() {
        for i in 0..result.len() - 1 {
            let should_swap = result[i].eq(server_key, &zero);

            result[i] = should_swap.if_then_else(server_key, &result[i + 1], &result[i]);
            result[i + 1] = should_swap.if_then_else(server_key, &zero, &result[i + 1]);
        }
    }

    result
}

pub fn trim_vector(mut vec: Vec<String>) -> Vec<String> {
    while vec.first() == Some(&"".to_string()) {
        vec.remove(0);
    }

    while vec.last() == Some(&"".to_string()) {
        vec.pop();
    }

    vec
}

pub fn trim_str_vector(mut vec: Vec<&str>) -> Vec<String> {
    while vec.first() == Some(&"") {
        vec.remove(0);
    }

    while vec.last() == Some(&"") {
        vec.pop();
    }

    vec.into_iter().map(|s| s.to_string()).collect()
}

fn compare_and_print<T: PartialEq + std::fmt::Debug>(expected: T, actual: T) {
    if expected == actual {
        print!("Test Passed: OK, ");
    } else {
        print!("Test Failed: Expected: {:?}, Got: {:?}, ", expected, actual);
    }
}

pub fn run_fhe_str_method(
    my_server_key: &MyServerKey,
    my_client_key: &MyClientKey,
    public_parameters: &PublicParameters,
    my_string_plain: &String,
    pattern_plain: &String,
    n_plain: usize,
    from_plain: &String,
    to_plain: &String,
    method: &StringMethod,
) {
    let my_string = my_client_key.encrypt(my_string_plain, STRING_PADDING, &public_parameters);
    let pattern = my_client_key.encrypt_no_padding(pattern_plain);
    let from = my_client_key.encrypt_no_padding(from_plain);
    let to = my_client_key.encrypt_no_padding(to_plain);
    let n = my_client_key.encrypt_char(n_plain as u8);

    match method {
        StringMethod::ToUpper => {
            let my_string_upper = my_server_key.to_upper(&my_string, &public_parameters);
            let actual = my_client_key.decrypt(my_string_upper, STRING_PADDING);
            let expected = my_string_plain.to_uppercase();

            compare_and_print(expected, actual);
        }
        StringMethod::ToLower => {
            let my_string_upper = my_server_key.to_lower(&my_string, &public_parameters);
            let actual = my_client_key.decrypt(my_string_upper, STRING_PADDING);
            let expected = my_string_plain.to_lowercase();

            compare_and_print(expected, actual);
        }
        StringMethod::Contains => {
            let res = my_server_key.contains(&my_string, &pattern, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.contains(pattern_plain);

            compare_and_print(expected as u8, actual);
        }
        StringMethod::ContainsClear => {
            let res = my_server_key.contains_clear(&my_string, &pattern_plain, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.contains(pattern_plain);

            compare_and_print(expected as u8, actual);
        }
        StringMethod::EndsWith => {
            let res =
                my_server_key.ends_with(&my_string, &pattern, STRING_PADDING, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.ends_with(pattern_plain);

            compare_and_print(expected as u8, actual);
        }
        StringMethod::EndsWithClear => {
            let res = my_server_key.ends_with_clear(
                &my_string,
                &pattern_plain,
                STRING_PADDING,
                &public_parameters,
            );
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.ends_with(pattern_plain);

            compare_and_print(expected as u8, actual);
        }
        StringMethod::EqIgnoreCase => {
            let heistack1 =
                my_client_key.encrypt(&my_string_plain, STRING_PADDING, &public_parameters);
            let heistack2 =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING + 20, &public_parameters);
            let res = my_server_key.eq_ignore_case(&heistack1, &heistack2, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.eq_ignore_ascii_case(&pattern_plain);

            compare_and_print(expected as u8, actual);
        }
        StringMethod::Find => {
            let res = my_server_key.find(&my_string, &pattern, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.find(pattern_plain);
            let expected = if let Some(position) = expected {
                position
            } else {
                MAX_FIND_LENGTH
            };

            compare_and_print(expected as u8, actual);
        }
        StringMethod::FindClear => {
            let res = my_server_key.find_clear(&my_string, &pattern_plain, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.find(pattern_plain);
            let expected = if let Some(position) = expected {
                position
            } else {
                MAX_FIND_LENGTH
            };

            compare_and_print(expected as u8, actual);
        }
        StringMethod::IsEmpty => {
            let res = my_server_key.is_empty(&my_string, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.is_empty();

            compare_and_print(expected as u8, actual);
        }
        StringMethod::Len => {
            let res = my_server_key.len(&my_string, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.len();

            compare_and_print(expected as u8, actual);
        }
        StringMethod::Repeat => {
            let n = my_client_key.encrypt_char(n_plain as u8);
            let my_string_upper = my_server_key.repeat(&my_string, n, &public_parameters);
            let actual = my_client_key.decrypt(my_string_upper, STRING_PADDING);
            let expected = my_string_plain.repeat(n_plain.into());

            compare_and_print(expected, actual);
        }
        StringMethod::RepeatClear => {
            let my_string_upper =
                my_server_key.repeat_clear(&my_string, n_plain, &public_parameters);
            let actual = my_client_key.decrypt(my_string_upper, STRING_PADDING);
            let expected = my_string_plain.repeat(n_plain.into());

            compare_and_print(expected, actual);
        }
        StringMethod::Replace => {
            let my_new_string = my_server_key.replace(&my_string, &from, &to, &public_parameters);
            let actual = my_client_key.decrypt(my_new_string, STRING_PADDING);
            let expected = my_string_plain.replace(from_plain, to_plain);

            compare_and_print(expected, actual);
        }
        StringMethod::ReplaceClear => {
            let my_new_string =
                my_server_key.replace_clear(&my_string, &from_plain, &to_plain, &public_parameters);
            let actual = my_client_key.decrypt(my_new_string, STRING_PADDING);
            let expected = my_string_plain.replace(from_plain, to_plain);

            compare_and_print(expected, actual);
        }
        StringMethod::ReplaceN => {
            let my_new_string =
                my_server_key.replacen(&my_string, &from, &to, n, &public_parameters);
            let actual = my_client_key.decrypt(my_new_string, STRING_PADDING);
            let expected = my_string_plain.replacen(from_plain, to_plain, n_plain.into());

            compare_and_print(expected, actual);
        }
        StringMethod::ReplaceNClear => {
            let my_new_string = my_server_key.replacen_clear(
                &my_string,
                &from_plain,
                &to_plain,
                n_plain as u8,
                &public_parameters,
            );
            let actual = my_client_key.decrypt(my_new_string, STRING_PADDING);
            let expected = my_string_plain.replacen(from_plain, to_plain, n_plain.into());

            compare_and_print(expected, actual);
        }
        StringMethod::Rfind => {
            let needle = my_client_key.encrypt_no_padding(&pattern_plain);
            let res = my_server_key.rfind(&my_string, &needle, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.rfind(pattern_plain);
            let expected = if let Some(position) = expected {
                position
            } else {
                MAX_FIND_LENGTH
            };

            compare_and_print(expected as u8, actual);
        }
        StringMethod::RfindClear => {
            let res = my_server_key.rfind_clear(&my_string, &pattern_plain, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.rfind(pattern_plain);
            let expected = if let Some(position) = expected {
                position
            } else {
                MAX_FIND_LENGTH
            };

            compare_and_print(expected as u8, actual);
        }
        StringMethod::Rsplit => {
            let fhe_split = my_server_key.rsplit(&my_string, &pattern, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.rsplit(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::RsplitClear => {
            let fhe_split =
                my_server_key.rsplit_clear(&my_string, &pattern_plain, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.rsplit(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::RsplitOnce => {
            let fhe_split = my_server_key.rsplit_once(&my_string, &pattern, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected = my_string_plain.rsplit_once(pattern_plain);

            match expected {
                Some(expected_tuple) => {
                    let expected = vec![expected_tuple.1, expected_tuple.0];
                    let actual = trim_vector(plain_split.0);
                    let expected = trim_str_vector(expected);

                    compare_and_print(expected, actual);
                }
                // Delimiter not found
                None => {
                    let actual = plain_split.1;
                    compare_and_print(0u8, actual);
                }
            }
        }
        StringMethod::RsplitOnceClear => {
            let fhe_split =
                my_server_key.rsplit_once_clear(&my_string, &pattern_plain, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);

            let expected_tuple = my_string_plain.rsplit_once(pattern_plain).unwrap();
            let expected = vec![expected_tuple.1, expected_tuple.0];

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::RsplitN => {
            let fhe_split = my_server_key.rsplitn(&my_string, &pattern, n, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.rsplitn(n_plain, pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::RsplitNClear => {
            let fhe_split = my_server_key.rsplitn_clear(
                &my_string,
                &pattern_plain,
                n_plain,
                &public_parameters,
            );
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.rsplitn(n_plain, pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::RsplitTerminator => {
            let fhe_split =
                my_server_key.rsplit_terminator(&my_string, &pattern, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.rsplit_terminator(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::RsplitTerminatorClear => {
            let fhe_split = my_server_key.rsplit_terminator_clear(
                &my_string,
                &pattern_plain,
                &public_parameters,
            );
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.rsplit_terminator(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::Split => {
            let fhe_split = my_server_key.split(&my_string, &pattern, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.split(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitClear => {
            let fhe_split =
                my_server_key.split_clear(&my_string, &pattern_plain, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.split(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitAsciiWhitespace => {
            let fhe_split = my_server_key.split_ascii_whitespace(&my_string, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.split_ascii_whitespace().collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitInclusive => {
            let fhe_split = my_server_key.split_inclusive(&my_string, &pattern, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.split_inclusive(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitInclusiveClear => {
            let fhe_split =
                my_server_key.split_inclusive_clear(&my_string, &pattern_plain, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.split_inclusive(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitTerminator => {
            let fhe_split =
                my_server_key.split_terminator(&my_string, &pattern, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.split_terminator(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitTerminatorClear => {
            let fhe_split = my_server_key.split_terminator_clear(
                &my_string,
                &pattern_plain,
                &public_parameters,
            );
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.split_terminator(pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitN => {
            let fhe_split = my_server_key.splitn(&my_string, &pattern, n, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.splitn(n_plain, pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::SplitNClear => {
            let fhe_split =
                my_server_key.splitn_clear(&my_string, &pattern_plain, n_plain, &public_parameters);
            let plain_split = FheSplit::decrypt(fhe_split, &my_client_key, STRING_PADDING);
            let expected: Vec<&str> = my_string_plain.splitn(n_plain, pattern_plain).collect();

            let actual = trim_vector(plain_split.0);
            let expected = trim_str_vector(expected);

            compare_and_print(expected, actual);
        }
        StringMethod::StartsWith => {
            let res = my_server_key.starts_with(&my_string, &pattern, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.starts_with(pattern_plain);

            compare_and_print(expected as u8, actual);
        }
        StringMethod::StartsWithClear => {
            let res =
                my_server_key.starts_with_clear(&my_string, &pattern_plain, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&res);
            let expected = my_string_plain.starts_with(pattern_plain);

            compare_and_print(expected as u8, actual);
        }
        StringMethod::StripPrefix => {
            let fhe_strip = my_server_key.strip_prefix(&my_string, &pattern, &public_parameters);
            let (actual, actual_pattern_found) =
                FheStrip::decrypt(fhe_strip, &my_client_key, STRING_PADDING);
            let expected = my_string_plain.strip_prefix(pattern_plain);
            let expected_pattern_found = if let Some(_) = expected { true } else { false };

            match expected {
                Some(expected) => {
                    compare_and_print(expected, &actual);
                }
                None => {
                    compare_and_print(expected_pattern_found as u8, actual_pattern_found);
                }
            }
        }
        StringMethod::StripPrefixClear => {
            let fhe_strip =
                my_server_key.strip_prefix_clear(&my_string, &pattern_plain, &public_parameters);
            let (actual, actual_pattern_found) =
                FheStrip::decrypt(fhe_strip, &my_client_key, STRING_PADDING);
            let expected = my_string_plain.strip_prefix(pattern_plain);
            let expected_pattern_found = if let Some(_) = expected { true } else { false };

            match expected {
                Some(expected) => {
                    compare_and_print(expected, &actual);
                }
                None => {
                    compare_and_print(expected_pattern_found as u8, actual_pattern_found);
                }
            }
        }
        StringMethod::StripSuffix => {
            // Limitation: Client needs to know the string padding
            let pattern_string =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING, &public_parameters);
            let fhe_strip =
                my_server_key.strip_suffix(&my_string, &pattern_string.bytes, &public_parameters);
            let (actual, actual_pattern_found) =
                FheStrip::decrypt(fhe_strip, &my_client_key, STRING_PADDING);
            let expected = my_string_plain.strip_suffix(pattern_plain);
            let expected_pattern_found = if let Some(_) = expected { true } else { false };

            match expected {
                // Pattern was found and stripped from original string
                Some(expected) => {
                    compare_and_print(expected, &actual);
                }
                // Pattern not found
                None => {
                    compare_and_print(expected_pattern_found as u8, actual_pattern_found);
                }
            }
        }
        StringMethod::StripSuffixClear => {
            // Limitation: Client needs to know the string padding
            let null_bytes = "\0".repeat(STRING_PADDING);
            let padded_pattern_plain = format!("{}{}", pattern_plain, null_bytes);

            let fhe_strip = my_server_key.strip_suffix_clear(
                &my_string,
                &padded_pattern_plain,
                &public_parameters,
            );
            let (actual, actual_pattern_found) =
                FheStrip::decrypt(fhe_strip, &my_client_key, STRING_PADDING);
            let expected = my_string_plain.strip_suffix(pattern_plain);
            let expected_pattern_found = if let Some(_) = expected { true } else { false };

            match expected {
                Some(expected) => {
                    compare_and_print(expected, &actual);
                }
                None => {
                    compare_and_print(expected_pattern_found as u8, actual_pattern_found);
                }
            }
        }
        StringMethod::Trim => {
            let my_trimmed_string = my_server_key.trim(&my_string, &public_parameters);
            let actual = my_client_key.decrypt(my_trimmed_string, STRING_PADDING);
            let expected = my_string_plain.trim();

            compare_and_print(expected, &actual);
        }
        StringMethod::TrimEnd => {
            let my_trimmed_string = my_server_key.trim_end(&my_string, &public_parameters);
            let actual = my_client_key.decrypt(my_trimmed_string, STRING_PADDING);
            let expected = my_string_plain.trim_end();

            compare_and_print(expected, &actual);
        }
        StringMethod::TrimStart => {
            let my_trimmed_string = my_server_key.trim_start(&my_string, &public_parameters);
            let actual = my_client_key.decrypt(my_trimmed_string, STRING_PADDING);
            let expected = my_string_plain.trim_start();

            compare_and_print(expected, &actual);
        }
        StringMethod::Concatenate => {
            let pattern_string =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING, &public_parameters);
            let my_string_concatenated =
                my_server_key.concatenate(&my_string, &pattern_string, &public_parameters);
            let actual = my_client_key.decrypt(my_string_concatenated, STRING_PADDING);
            let expected = format!("{}{}", my_string_plain, pattern_plain);

            compare_and_print(expected, actual.into());
        }
        StringMethod::Lt => {
            let pattern_string =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING, &public_parameters);
            let actual = my_server_key.lt(&my_string, &pattern_string, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&actual);
            let expected = (my_string_plain < pattern_plain) as u8;

            compare_and_print(expected, actual);
        }
        StringMethod::Le => {
            let pattern_string =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING, &public_parameters);
            let actual = my_server_key.le(&my_string, &pattern_string, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&actual);
            let expected = (my_string_plain <= pattern_plain) as u8;

            compare_and_print(expected, actual);
        }
        StringMethod::Gt => {
            let pattern_string =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING, &public_parameters);
            let actual = my_server_key.gt(&my_string, &pattern_string, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&actual);
            let expected = (my_string_plain > pattern_plain) as u8;

            compare_and_print(expected, actual);
        }
        StringMethod::Ge => {
            let pattern_string =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING, &public_parameters);
            let actual = my_server_key.ge(&my_string, &pattern_string, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&actual);
            let expected = (my_string_plain >= pattern_plain) as u8;

            compare_and_print(expected, actual);
        }
        StringMethod::Eq => {
            let pattern_string =
                my_client_key.encrypt(&pattern_plain, STRING_PADDING, &public_parameters);
            let actual = my_server_key.eq(&my_string, &pattern_string, &public_parameters);
            let actual: u8 = my_client_key.decrypt_char(&actual);
            let expected = (my_string_plain == pattern_plain) as u8;

            compare_and_print(expected, actual);
        }
    }
}
