# leptos-obfuscate

This is for easy email address obfuscation to prevent bots and spammers.

Add the following to your CSS stylesheet:

```css
span.obfuscate {
    unicode-bidi: bidi-override;
    direction: rtl;
    cursor: pointer;
}

span.obfuscate > i {
    display: none;
}

span.obfuscate > span::after {
    content: '@';
}
```

The component accepts an optional honeypot email address / link you can use, if you want to have a
sophisticated setup and blacklist any sender that sends an E-Mail to it.

The `delay_seconds` can be set as well. After this timeout, when mounted inside the browser,
the honeypot address will be exchanged with the real one. This means the link will not work with
HTML only, but there is no good way to prevent bots without Javascript / WASM.

Then just use it like this:

```notest
let (email, _) = create_signal("mail@example.com".to_string());
view! { <ObfuscateEmail email /> }
```

# Panics

If the given String does not contain '@'

## Leptos Compatibility

### Leptos v0.7

leptos-obfuscate v0.3+

### Leptos v0.6

leptos-obfuscate v0.2

### Leptos v0.5

leptos-obfuscate v0.1
