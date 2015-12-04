A demo of using [nanny](http://github.com/dherman/nanny) and [rust-bindings](http://github.com/dherman/rust-bindings) to perform a non-trivial computation in Rust inside of a Node app.

## Setup

**Note: this is currently only working on OS X.**

### OS X

* [XCode](https://developer.apple.com/xcode/download/)
* Node: Node 4 or later. I recommend using [nvm](https://github.com/creationix/nvm#install-script):

```
% nvm install 4
```

* [multirust](https://github.com/brson/multirust#quick-installation)

*Right now multirust is a mandatory dependency because this demo relies on an unstable API and therefore requires Rust Nightly. This will change soon.*

## Running the Demo

Make sure you are using the right Node version:

```
% nvm use 4
```

Since [rust-bindings](http://github.com/dherman/rust-bindings) automates the build process, setting up and running this demo is just like any other Node package:

```
% npm install
% node -e 'require("./")'
```
