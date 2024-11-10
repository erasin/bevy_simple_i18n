# bevy_i18n

An opinionated but dead simple internationalization library for the Bevy game engine.

## Project Status

This project is in the early stages of development and currently acts as a wrapper for the [rust-i18n](https://github.com/longbridgeapp/rust-i18n) library. The long term goal is to implement a more Bevy-native solution. Expect breaking changes.

## [Demo](https://turtiesocks.github.io/bevy_i18n/)

## Usage

### Cargo.toml

Add the following to your `Cargo.toml`:

```toml
bevy_i18n = "0.1"
```

### main.rs

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(I18nPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((I18nText::new("hello"), I18nFont::new("NotoSans")));
}
```

## File Structure

In order to use this plugin you'll need to set up your asset folder in the following way:

```ts
.
├── assets
│   ├── locales
│   │   ├── {locale_file}.yml
│   │   ├── {locale_file}.json
│   │   └── {locale_file}.toml
│   └── fonts
│       └── {font_name}
│           ├── fallback.ttf
│           ├── {locale}.ttf
│           └── {locale}.otf
└── Cargo.toml
```

## Locale Files

Locale files are stored in the `assets/locales` directory. Since we're just using the `rust-i18n` library, the format is the same. You can find more information on the supported formats [here](https://github.com/longbridgeapp/rust-i18n?tab=readme-ov-file#locale-file).

## Features

### Translations

To translate text, you can use the `I18nText` component. This component takes a string as an argument and will automatically translate it based on the current locale.

Translation File:

```yml
_version: 2
hello:
  en: Hello world
  zh-TW: 你好世界
  ja: こんにちは世界
```

Bevy code:

```rust
commands.spawn(I18nText::new("hello"));
```

### Interpolation

Interpolation is supported using the `I18nText` component. You can interpolate variables by adding tuple (key, value) arguments to the `I18nText` component.

Translation File:

```yml
_version: 2
messages.hello:
  en: Hello, %{name}
  zh-TW: 你好，%{name}
  ja: こんにちは、%{name}
```

Bevy code:

```rust
commands.spawn(I18nText::new("messages.hello").with_arg("name", "world"));
```

### Dynamic Fonts

Dynamic fonts enable this plugin to automatically switch between different fonts based on the current locale. For example, since Japanese and English languages have different character sets, you may want to use different fonts for each language. In order to make use of dynamic font, you must follow the file structure mentioned above.

Folder setup:

```ts
.
├── assets
│   └── fonts
│       └── NotoSans
│           ├── fallback.ttf
│           ├── ja.ttf
│           └── zh.ttf
└── Cargo.toml
```

We would then spawn the dynamic font using:

```rust
commands.spawn((I18nText::new("hello"), I18nFont::new("NotoSans")))
```

When the locale is set to `ja`, the font will be set to `ja.ttf`. If the locale is set to `zh-TW`, the font automatically load `zh.ttf`, since `zh-TW` does not have a font file. If the locale is set to any other locale, Bevy will load `fallback.ttf`.

### Automatic Text Re-Rendering

When the locale is changed, the plugin will automatically update all `I18nText` components to reflect the new locale. No boilerplate code is required, other than changing the locale using the `I18n` resource.

```rust
fn change_locale(mut i18n: ResMut<I18n>) {
    i18n.set_locale("zh-TW");
}
```

## Bevy support table

| bevy | bevy_i18n |
| ---- | --------- |
| main | 0.1       |

## Credits

- [Fonts](https://fonts.google.com/noto/fonts)
