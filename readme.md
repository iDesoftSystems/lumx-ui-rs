# Lumx

Lumx is a rich component library for the [Leptos](https://github.com/leptos-rs/leptos) web framework.

It provides ready to be used components for capturing user input through cards, buttons, input fields, select inputs, sliders, date & time inputs or even a rich text editor.

## Installation

Start by adding `lumx` as a dependency of your app.

```
cargo add lumx-ui
```

Lumx comes with default styling in form of the `lumx-theme`. Themes, as well as other static files, are automatically copied to your project root directory when building your application. You have to tell `Lumx` where you want these files to be stored. We recommended not excluding them from your version control system(VCS).

To incorporate the Lumx themes in your app, add the following to your `style/main.scss` file.

```
@import './lumx/lumx-ui.scss';
```