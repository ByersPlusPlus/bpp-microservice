# ByersPlusPlus Microservice Template

This is the scaffolding for every microservice used in ByersPlusPlus!

You can create your own microservice with this, thanks to `cargo-generate`.
Though you still have to implement the server yourself, it takes care of repeating work.

## How to use this?

First and foremost, install `cargo-generate` by running `cargo install cargo-generate` if you haven't already.
You only need to do this once on your machine.

To get started make sure that everything works, create your very own `.proto` file for the microservice you want to make.

Here is a template for such a proto file:

```proto
syntax = "proto3";
package myservice;

// Service implementation here
```

Please make sure to name your file and your package the same if you want to use this template!

Then fork the [proto](https://github.com/ByersPlusPlus/proto) repository, add your proto file, commit and push it to your fork.

This takes care of the prep work!

Next you want to run `cargo generate --git https://github.com/ByersPlusPlus/bpp-microservice.git`, which will ask you a few questions about your project, like the name of the project and the proto file.

**IMPORTANT!** It is currently not possible to generate the temple over SSH with any SSH keys other than RSA!

If everything worked out fine, you should now have a new Rust project, containing most of the files you need. There is just one more step before you can start to develop, which is adding the proto repository as a submodule. You can do that by running either `git submodule add git@github.com:yourName/proto.git` or `git submodule add https://github.com/yourName/proto.git` (please make sure to rename `yourName` to your GitHub username).

After that, you're done and you can start working on your service! The project will have a git initialized, but no remote configured, so you will need to do that too. A quick and easy way to do that is by using GitHub's CLI tool `hub`: `hub create yourName/yourservice`.

Once your microservice is done, create an issue on the compose repository for including your microservice.
