# DXC Themes

**A theme collection for DXC, powered by [DXC](https://github.com/dioxuslabs/dioxus) and [ElementPlus](https://github.com/element-plus/element-plus).**

DXC Themes provides a curated set of professional themes to enhance your DXC applications with consistent, modern styling.

## Usage

Due to current limitations in `dioxus` regarding automatic asset loading,
you need to **manually import the theme stylesheet** in your app.

```rust
use dioxus::prelude::*;
use dxc::prelude::*;

fn app() -> Element {
    rsx! {
        document::Link{ 
          rel: "stylesheet", 
          href: DXC_THEMES // Applies the default theme
        }

        div {
          // Your app content here
        }
    }
}
```

## Build

First, make sure you have installed [Node](https://nodejs.com) or [Bun](https://bun.sh).

Then, run the following command to build the themes:
```bash
# Using Bun
bun run build

# Using npm
npm run build

# Using pnpm
pnpm run build
```

The compiled CSS files will be generated in the `assets/css` directory.