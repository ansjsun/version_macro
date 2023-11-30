extern crate proc_macro;
use std::{process::Command, str::FromStr};

use chrono::{Datelike, Local, Timelike};
use proc_macro::TokenStream;

#[proc_macro]
pub fn build_time(_item: TokenStream) -> TokenStream {
    let now = Local::now();

    let now = format!(
        "\"{}-{:02}-{:02} {:02}:{:02}:{:02}\"",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    );
    TokenStream::from_str(now.as_str()).unwrap()
}

#[proc_macro]
pub fn build_git_branch(_item: TokenStream) -> TokenStream {
    let cmd = Command::new("git")
        .args(&["describe", "--all", "--dirty"])
        .output()
        .unwrap();

    let result = if cmd.status.success() {
        match String::from_utf8(cmd.stdout) {
            Ok(v) => format!("\"{}\"", v.trim()),
            Err(e) => format!("\"{}\"", e),
        }
    } else {
        match String::from_utf8(cmd.stderr) {
            Ok(v) => format!("\"{}\"", v.trim()),
            Err(e) => format!("\"{}\"", e),
        }
    };

    TokenStream::from_str(result.as_str()).unwrap()
}

#[proc_macro]
pub fn build_git_version(_item: TokenStream) -> TokenStream {
    let cmd = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();

    let result = if cmd.status.success() {
        match String::from_utf8(cmd.stdout) {
            Ok(v) => format!("\"{}\"", v.trim()),
            Err(e) => format!("\"{}\"", e),
        }
    } else {
        match String::from_utf8(cmd.stderr) {
            Ok(v) => format!("\"{}\"", v.trim()),
            Err(e) => format!("\"{}\"", e),
        }
    };

    TokenStream::from_str(result.as_str()).unwrap()
}
