#![allow(unused)]
#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

fn main() {
    fn creator<'a>() -> WhatAboutThis<'a> {
        let mut tricky = WhatAboutThis {
            name: "Annabelle".to_string(),
            nickname: None,
        };
        tricky.nickname = Some(&tricky.name[..4]);

        tricky
    }
}
