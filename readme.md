# Lumx UI Kit

Lumx is a rich component library for the [Leptos](https://github.com/leptos-rs/leptos) web framework.

It provides ready to be used components for capturing user input through cards, buttons, input fields, select inputs, sliders, date & time inputs or even a rich text editor.

## Installation

Add to your `Cargo.toml` dependencies:

```toml
[dependencies]
lumx-ui = { git = "https://github.com/idesoftSystems/lumx-ui-rs.git", branch = "main" }
```

Lumx comes with default styling in form of the `lumx-theme`. Themes, as well as other static files, are automatically copied to your project root directory when building your application. You have to tell `Lumx` where you want these files to be stored. We recommended not excluding them from your version control system(VCS).

To incorporate the Lumx themes in your app, add the following to your `style/main.scss` file.

```scss
@import './lumx/lumx-ui.scss';
```

Or link from your `index.html`

```html
<link data-trunk rel="scss" href="./style/lumx/lumx-ui.scss">
```
