# Password Generator

The password generator allows to generate pseudo-random passwords by combining upper, lowercase and special characters.

## Flags

*-v*. Verbose execution.

*-s*. The number of special characters that will be allowed in the password.

*-l*. The length of the password.

For example, this command will produce a password of 24 chars with a maximum of 2 special characters.

```ps1
.secure_password_generator.exe -s 2 -l 24
```

## Building the script

```bash
cargo build --release