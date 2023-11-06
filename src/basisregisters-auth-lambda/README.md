# Basisregisters Auth Lambda

## Goal

> custom aws api gateway authorizer lambda function written in Rust

## Motive for Rust

Before embracing Rust, we had been utilizing high-level languages like Python and JavaScript (Node.js) to handle our AWS authorizer. It soon became apparent that this approach introduced a significant amount of overhead, which in turn hampered the efficiency of our API calls. Several pain points emerged in this scenario, prominently the lambda cold starts and dynamodb cold starts, both of which proved to be substantial bottlenecks.

While quick improvements were achieved by adopting ARM functions over X86_64, a more profound enhancement, though somewhat costly, involved the integration of a Redis cache to curtail the frequency of dynamodb calls. This measure indeed ameliorated the situation, yet it still fell short of the desired level of performance.

And then came Rust. With Rust, we have managed to minimize the overhead to an impressive extent, with an overhead not exceeding 2-4 milliseconds. This transformation has revolutionized our operational efficiency and performance, providing us with a level of speed and responsiveness that was previously out of reach.

## Development

### Getting Started

#### Prerequisites

Before you begin, make sure to install the following:

- [Docker](https://docs.docker.com/engine/install/) (for the release script)
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo-lambda](https://www.cargo-lambda.info/guide/getting-started.html)

For development, we recommend using these IDEs:

- [Visual Studio Code (VSCode)](https://code.visualstudio.com/)
- [RustRover](https://www.jetbrains.com/rust/) (still in preview)

#### Build

To build the project, run the following command:

```bash
cargo lambda build
```
NOTE: this uses the [Cargo-lambda](https://www.cargo-lambda.info/guide/getting-started.html) binary

#### Clean

To clean the project, use the following command:
```bash
cargo clean
```

#### Release
To create a release for basisregisters-auth-lambda, run the release script:
```bash
./release-auth.sh
```
The output zip file is stored in the `./dist/basisregisters-auth-lambda` directory path. Please note that this won't be stored in git.

#### Publish

For publishing, manually upload the zip file to AWS. While Cargo-lambda does support CLI deployment, it is not set up yet.

## License

This project is licensed under the [European Union Public Licence (EUPL)](https://joinup.ec.europa.eu/news/understanding-eupl-v12)

The new version 1.2 of the European Union Public Licence (EUPL) is published in the 23 EU languages in the EU Official Journal: [Commission Implementing Decision (EU) 2017/863 of 18 May 2017 updating the open source software licence EUPL to further facilitate the sharing and reuse of software developed by public administrations](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=uriserv:OJ.L_.2017.128.01.0059.01.ENG&toc=OJ:L:2017:128:FULL) ([OJ 19/05/2017 L128 p. 59–64](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=uriserv:OJ.L_.2017.128.01.0059.01.ENG&toc=OJ:L:2017:128:FULL)).

## Credits

### Languages & Frameworks

* [rust](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) - [MIT](https://choosealicense.com/licenses/mit/)
* [cargo](https://github.com/rust-lang/cargo/blob/master/LICENSE-MIT) - _Cargo downloads your Rust project’s dependencies and compiles your project._ - [MIT](https://choosealicense.com/licenses/mit/)
* [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda/blob/main/LICENSE) - _cargo-lambda is a Cargo subcommand to help you work with AWS Lambda._ - [MIT](https://choosealicense.com/licenses/mit/)

### Libraries

* [aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust/blob/main/LICENSE) - _AWS SDK for Rust._ - [APACHE License 2.0](https://choosealicense.com/licenses/apache-2.0/)
* [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/LICENSE) - _This package makes it easy to run AWS Lambda Functions written in Rust._ - [APACHE License 2.0](https://choosealicense.com/licenses/apache-2.0/)
* [tokio](https://github.com/tokio-rs/tokio/blob/master/LICENSE) - _A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language._ - [MIT](https://choosealicense.com/licenses/mit/)
* [tracing](https://github.com/tokio-rs/tracing/blob/master/LICENSE) - _Application-level tracing for Rust._ - [MIT](https://choosealicense.com/licenses/mit/)
* [serde](https://github.com/serde-rs/serde/blob/master/LICENSE-MIT) - _Serde is a framework for serializing and deserializing Rust data structures efficiently and generically._ - [MIT](https://choosealicense.com/licenses/mit/)
* [serde_dynamo](https://github.com/zenlist/serde_dynamo/blob/main/LICENSE.md) - _serde_dynamo provides a way to serialize and deserialize between data stored in these items and strongly-typed Rust data structures._ - [MIT](https://choosealicense.com/licenses/mit/)
* [base64](https://github.com/marshallpierce/rust-base64/blob/master/LICENSE-MIT) - _It's base64. What more could anyone want?._ - [MIT](https://choosealicense.com/licenses/mit/)
* [redis](https://github.com/redis-rs/redis-rs/blob/master/LICENSE) - _Redis-rs is a high level redis library for Rust. It provides convenient access to all Redis functionality through a very flexible but low-level API._ - [redis-rs contributors](https://github.com/redis-rs/redis-rs/blob/main/LICENSE)
* [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs/blob/master/LICENSE-MIT) - _A macro for declaring lazily evaluated statics in Rust._ -[MIT](https://choosealicense.com/licenses/mit/)
