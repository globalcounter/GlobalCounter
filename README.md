# Android with Rust

An `Android` fun app which allows users to `increase/decrease` a global counter value simultaneously.

## Requirements

It's using [tusk](https://github.com/rliebz/tusk) to run the automated scripts. Install it from the following:

```sh
brew install rliebz/tusk/tusk
```

## Development

Setup the project by installing all the required dev tools:

```sh
tusk setup
```

Start the dev hot rebuild process:

```sh
tusk dev
```

Build release library files:

```sh
tusk build:prod
```

Generate jni files:

```sh
tusk jni
```

See logcat via `ADB`:

```sh
tusk log
```

Please refer to `tusk.yml` file for more commands.