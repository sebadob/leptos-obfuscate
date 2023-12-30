# leptos-obfuscate

This is for easy email address obfuscation to prevents bots and spammers.

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

The component excepts an optional honeypot email address / link you can use, if you want to have a
sophisticated setup and blacklist any sender that sends an E-Mail to it.

The `delay_seconds` can be set as well. After this timeout, when mounted inside the browser,
the honeypot address will be exchanged with the real one. This means the link will not work with
HTML only, but there is no good way to prevent bots without Javascript / WASM.

# Panics
If the given String does not contain '@'
