#![allow(dead_code)]
use std::collections::HashMap;
use std::process::Command;

type Bytes = [u8];
type Error = String;

pub trait ToVecString {
    fn to_vec_string(&self, _: bool) -> Vec<String>;
}

impl ToVecString for String {
    fn to_vec_string(&self, trim_line: bool) -> Vec<String> {
        //! Convert String with newlines into Vec\<String\> - split by '\n'
        self.lines()
            .filter(|l| !l.is_empty())
            .map(|s| if trim_line { s.trim() } else { s }.into())
            .collect()
    }
}

trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for Bytes {
    fn as_string(&self) -> String {
        //! Convert bytes into String
        String::from_utf8_lossy(self).parse().unwrap()
    }
}

pub fn value_for_substring<'a>(from_str: &'a str, substr: &str) -> Option<&'a str> {
    //! find a value of a substring in a given str
    //! eg: Some long string `key` `val` - find the val for the given key
    //!
    //! ## Example usage:
    //! ```ignore
    //! let s = "The bird Cuckoo is not a national bird!";
    //! println!("{:?}", value_for_substring(s, "bird"));   // will print Cuckoo
    //! assert!(value_for_substring(s, "bird").is_some());
    //! ```
    // If the substr is empty, lets assume that there is no search required,
    // so return the string back
    if substr.is_empty() {
        return Some(from_str.trim());
    }
    if let Some(substr_start) = from_str.find(substr) {
        // Ignore prefix and leading whitespace
        let string = &from_str[substr_start + substr.len()..].trim_start();

        // Find where the word boundary ends
        let word_end = string
            .find(|c: char| c.is_whitespace())
            .unwrap_or_else(|| string.len());
        let string = string[..word_end].trim_matches(&[' ', '=', '\"'] as &[_]);

        Some(string)
    } else {
        None
    }
}

#[inline]
pub fn exec_command(command: &str) -> Result<String, Error> {
    //! Executes a command with arguments and returns
    //! the output in Vector of line strings.
    //!
    //! ## Example usage:
    //! ```ignore
    //!
    //! fn main() {
    //!     let cmd = "ls";
    //!     let output = exec_command(cmd);
    //!     println!("{:?}", output);
    //! }
    //! ```
    exec_command_with_args(command, &[])
}

pub fn exec_command_with_args(command: &str, args: &[&str]) -> Result<String, Error> {
    //! Executes a command with arguments and returns
    //! the output in Vector of line strings.
    //!
    //! ## Example usage:
    //! ```ignore
    //!
    //! fn main() {
    //!     let cmd = "ls";
    //!     let args = ["-l", "-h"];
    //!     let output = exec_command_with_args(cmd, &args);
    //!     println!("{:?}", output);
    //! }
    //! ```
    Command::new(command)
        .args(args)
        .output()
        .map_err(|e| e.to_string())
        .and_then(|out| {
            if out.stdout.is_empty() {
                Ok(out.stderr[..].as_string())
            } else {
                Ok(out.stdout[..].as_string())
            }
        })
}

pub fn to_hashmap(string: String) -> HashMap<String, String> {
    //! Convert a String containing Key: Value into an HashMap.
    //! Default delim used here is `':'`.
    //!
    //! ## Example usage:
    //! ```ignore
    //! let string = "Key1: Value1\nKey2: Value2";
    //! println!("{:?}", to_hashmap(string));
    //! ```
    to_hashmap_with_delim(string, ':')
}

pub fn to_hashmap_with_delim(string: String, delim: char) -> HashMap<String, String> {
    //! Convert a String containing Key: Value into an HashMap.
    //! You can use a custom delimeter here.
    //! eg: ':', '=', '.', etc...
    //!
    //! ## Example usage:
    //! ```ignore
    //! let string = "Key1=Value1\nKey2=Value2";
    //! println!("{:?}", to_hashmap_with_delim(string, '='));
    //! ```
    string
        .lines()
        .filter_map(|l| l.split_once(delim))
        .map(|(key, val)| {
            (
                key.trim_matches(&[' ', '=', '\"', '\t', '\n'] as &[_])
                    .to_string(),
                val.trim_matches(&[' ', '=', '\"', '\t', '\n'] as &[_])
                    .to_string(),
            )
        })
        .collect()
}
