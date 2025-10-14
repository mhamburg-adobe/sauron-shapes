This is a small shape drawing web application written using the Sauron Rust framework:

    https://github.com/ivanceras/sauron/

It has more structure than it needs for the task at hand and yet less than one would want in a more complex application.

Here is a brief summary of the files:

    index.html: The web page with loading instructions
    src/
        lib.rs             The top level logic for building the library
        shell.rs:          The outermost UX layer; relatively generic
        framework/         Standard files that should be reusable across projects
            tracker.rs     Support for mouse move and mouse up tracking
        shapes/            The app specific code
            core.rs        Definition of core shapes types
            doc.rs         Arranging shapes into a document
            app.rs         The TEA logic for building an application using core and doc

Pre-requisites:

    cargo install wasm-pack
    cargo install basic-http-server

Build using:

    wasm-pack build --target web --release

Serve using:

    basic-http-server -a 0.0.0.0:4000

Then load the web page:

    http://localhost:4000

