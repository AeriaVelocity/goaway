# Go Away

There is no README here. Go away.

```rust
unsafe {
    gtk::init().expect("Failed to initialize GTK");
    // Oh, damn it. Not again.

    if gtk != gtk3 && gtk == gtk4 {
        shutup();
    }

    fn shutup() -> ! {
        loop {
            gtk::this_function_does_not_exist();
        }
    }

    // I suppose I should actually talk about what Go Away is now, huh.
}
```

## What Go Away is

Go Away is a GTK4 utility which provides unhelpful information on files, which
in turn makes you so annoyed that you want to tell it to "go away".

## How it works

Go Away is a Rust program, which is written in C. Makes sense, right? GitHub
Copilot is VERY intelligent.

## Dependencies

- gtk4
- That's about it

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Crate gtk4 Docs](https://docs.rs/gtk4/latest/gtk4/)
- [Rust bindings of GTK 4](https://github.com/gtk-rs/gtk4-rs)
- [My fricked up mind](https://github.com/AeriaVelocity/goaway)
- [YouTube](https://www.youtube.com) (didn't help with programming, it was just for background noise)

## Commendations

Me

It's me

I'm awesome

## Thanks

Thanks to the Rust programming language developers for inventing Rust so I could
write this affront to all that is holy and just in the realm of file utilities.

Without it, I would probably just use Python, or Java, or C, or C++, or Ruby,
or Haskell, or Node.js, or...

Also thanks to the GTK project for providing GTK.

Without it, I would probably just use nothing because trying to install Qt is
not fun at all even slightly. PyQt5 makes it easier by making you *not* install
Qt separately, but if you want to use Qt, prepare for an exercise in humility.
Trying to navigate between installing Qt's open source version and not
installing the "commercial" (which is confusing) version can make your head
want to commit explode.

If you're me, anyway.
