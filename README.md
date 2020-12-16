## About
A visualization of the gradient descent method. It's first main focus were getting a fast working prototype
utilizing the mathbox² library combined with a Rust WebAssembly Model library.

This project is part of a series of projects trying to visualize some maths or computer science algorithms.
The series' main goal is trying out and learning to work with different libraries and (web-)technologies.

## Prerequisities
You will need the following tools installed on your System:

[rustup](https://www.rust-lang.org/tools/install)
  - a tool called cargo will be installed with rustup. It is Rust's dependency manager.
  - it will also install rustc, which is the rust compiler
[npm](https://www.npmjs.com/get-npm)
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer)

I trust you can figure out how to install those tools with their representive documentations.

## Quickstart Guide
(only tested on linux)

To build this projects there are actually a few steps to be taken. Obviously first Clone this repo. Install the Prerequisities.
Move your terminal to the projects folder. Then execute this command:

    wasm-pack build

This compiles automatically downloads all Rust dependencies and compiles your Rust code into WebAssembly, creates all the necessary
js bindings and puts it all together into the "/pkg" subdirectory.

Next is gonna be the JavaScript part. Go into the www folder and execute:

    npm install

npm now installs all the dependencies and dev-dependencies. We are almost ready now. But we'll have to do a manual tweak in the mathbox²
library ourselves. Go into your "www/node_modules/mathbox/build" folder and open the file called "mathbox-bundle.js". Here you'll want to
move the following code from the top of the file to the very bottom of it:

    // browserify support
    if ( typeof module === 'object' ) {
        module.exports = THREE;
    }

This will make sure that the "THREE" Object will be available correctly outside of the module's scope. This is an ES6 thing.
Now you can build the Javascript part of the project:

    npm run-script build
    npm start

and with that start a local dev server that'll listen on "https://localhost:8080"
If everything works: nice! Well done! :)

## Contributing
If you want to contribute just leave a message for me in Github or wait until I got the Pull Request configuration done :).
