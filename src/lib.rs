// Copyright 2023 Sebastian Dobe <sebastiandobe@mailbox.org>

#![doc = include_str!("../README.md")]

use core::time::Duration;
use leptos::*;

/// The component accepts an optional honeypot email address / link you can use, if you want to have a
/// sophisticated setup and blacklist any sender that sends an E-Mail to it.
///
/// The `delay_seconds` can be set as well. After this timeout, when mounted inside the browser,
/// the honeypot address will be exchanged with the real one. This means the link will not work with
/// HTML only, but there is no good way to prevent bots without Javascript / WASM.
///
/// # Panics
/// If the given String does not contain '@'
#[component]
pub fn ObfuscateEmail(
    email: ReadSignal<String>,
    #[prop(default = "mailto:honeypot@example.com")] honeypot: &'static str,
    #[prop(default = 3)] delay_seconds: u64,
) -> impl IntoView {
    let mailto = create_rw_signal(honeypot.to_string());

    create_effect(move |_| {
        let mail = format!("mailto:{}", email.get());
        set_timeout(move || mailto.set(mail), Duration::from_secs(delay_seconds));
    });

    let one = move || {
        let plain = email.get();
        let (one, _) = plain.split_once('@').unwrap();
        one.chars().rev().collect::<String>()
    };
    let two = move || {
        let plain = email.get();
        let (_, two) = plain.split_once('@').unwrap();
        two.chars().rev().collect::<String>()
    };

    view! {
        <a href=move || mailto.get()>
            <span aria-label="E-Mail" class="obfuscate">
                {two}
                <i>"%/#"</i>
                <span></span>
                {one}
            </span>
        </a>
    }
}
