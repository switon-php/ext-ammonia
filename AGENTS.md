# AGENTS.md

`README.md` has the build steps. `stubs/README.md` has the method list.

## Language

PHPDoc, both READMEs, `phpinfo()` strings, and commit messages are English.

## API

Instance methods take `&mut self` and return `&mut Self`. A `ZendClassObject` receiver is registered as a static method and breaks the call.

## Tests

Every public `Builder` method has a PHP assertion that shows its effect, including a success path. An error-only call does not count. Rust tests in `src/policy.rs` cover option replay.
