---
layout: post-math.liquid
title: Firefox Shortcuts for Rustaceans
published_date: 2019-03-24 00:00:00 -0800
description: A couple of productivity boosting tricks to speed up workflow.
data:
    link_type: internal
    route: feed
    type: blog
    published_date_friendly: March 24th, 2019
---
I recently found out about Firefox's advanced bookmarks feature, and quickly found a way to use them to speed up my workflow. I made bookmarks to search the [Rust standard library](https://doc.rust-lang.org/std/), the [crates.io](https://crates.io)  registry, and the crate documentation aggregate [docs.rs](https://docs.rs).

Here is a short demo:

<center>
    <img src="/assets/blog/firefox-shortcuts/demo.gif" style="width: 100%; max-width: 704px;" alt="GIF demoing the bookmark shortcuts.">
</center>

Here are the three bookmarks that I made in Firefox:

<center>
    <img src="/assets/blog/firefox-shortcuts/std.png" style="width: 100%; max-width: 440px;" alt="std lib search bookmark">
</center>

<center>
    <img src="/assets/blog/firefox-shortcuts/crates.png" style="width: 100%; max-width: 440px;" alt="crates search bookmark">
</center>

<center>
    <img src="/assets/blog/firefox-shortcuts/docs.png" style="width: 100%; max-width: 440px;" alt="docs search bookmark">
</center>

Here is the text for each bookmark for easy copy-paste:

STD
```
https://doc.rust-lang.org/std/?search=%s
```

crates.io
```
https://crates.io/search?q=%s
```

docs.rs
```
https://docs.rs/releases/search?query=%s&i-am-feeling-lucky=1
```
_N.B., remove the `&i-am-feeling-lucky=1` to get normal searching._

I hope this was helpful!
