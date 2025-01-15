# Template RS CLI

A command-line interface for managing and executing Rust code templates using the `rust_templates` library.

## Installation

```bash
cargo install template_rs_cli
```

Or build from source:

```bash
git clone https://github.com/tristanpoland/template_rs_cli
cd template-cli
cargo install --path .
```

## Requirements

- Rust 1.70 or higher
- rust-script (for template execution)

For template execution, ensure rust-script is installed:
```bash
cargo install rust-script
```

## Commands

### Creating Templates

Create a new template file with inline content:

```bash
template_rs_cli new -o hello.template_rs -c 'fn main() { println!("@[message]@"); }'
```

Create a template from an existing file:

```bash
template_rs_cli new -o output.template_rs -f input.rs
```

### Rendering Templates

Render a template with placeholder values:

```bash
template_rs_cli render \
    -t hello.template_rs \
    -v message="Hello, World!" \
    -o hello.rs
```

Multiple values can be provided:

```bash
template_rs_cli render \
    -t struct.template_rs \
    -v struct_name=User \
    -v fields="name: String, age: u32" \
    -o user.rs
```

### Executing Templates

Execute a template with rust-script:

```bash
template_rs_cli execute \
    -t math.template_rs \
    -v number_type=f64 \
    -v value=42.0 \
    -d "num=0.4"
```

Add multiple dependencies:

```bash
template_rs_cli execute \
    -t web.template_rs \
    -v url="https://api.example.com" \
    -d "reqwest=0.11" \
    -d "tokio=1.0" \
    -d "serde_json=1.0"
```

### Assembling Templates

Combine multiple templates with shared values:

```bash
template_rs_cli assemble \
    -t header.template_rs \
    -t implementation.template_rs \
    -t tests.template_rs \
    -v module_name=calculator \
    -o combined.rs
```

## Template Format

Templates use `@[placeholder_name]@` syntax for placeholders:

```rust
// Example template
fn @[function_name]@() -> @[return_type]@ {
    let value = @[value]@;
    println!("@[message]@: {}", value);
    value
}
```

## Value Formatting

Values are provided using key=value pairs:
- Basic values: `name=value`
- Strings with spaces: `message="Hello World"`
- Multiple lines: Use `\n` for newlines
- Rust code: Escape special characters as needed

## Dependencies

Dependencies for template execution use name=version format:
```bash
-d "serde=1.0"
-d "tokio=1.0"
```

## Error Handling

The CLI provides clear error messages for common issues:
- Missing placeholder values
- Invalid template syntax
- File system errors
- Execution errors
- Invalid dependency specifications

## Examples

### Basic Hello World

```bash
# Create template
template_rs_cli new -o hello.template_rs -c '
fn main() {
    println!("@[greeting]@ @[name]@!");
}
'

# Render template
template_rs_cli render \
    -t hello.template_rs \
    -v greeting=Hello \
    -v name=World \
    -o hello.rs

# Execute template
template_rs_cli execute \
    -t hello.template_rs \
    -v greeting=Hello \
    -v name=World
```

### Data Structure Template

```bash
# Create a struct template
template_rs_cli new -o struct.template_rs -c '
#[derive(Debug)]
struct @[struct_name]@ {
    @[fields]@
}

fn main() {
    let instance = @[struct_name]@ {
        @[field_values]@
    };
    println!("{:?}", instance);
}
'

# Render with values
template_rs_cli render \
    -t struct.template_rs \
    -v struct_name=User \
    -v fields="name: String,\n    age: u32" \
    -v field_values="name: String::from(\"Alice\"),\n    age: 30" \
    -o user.rs
```

### Web Request Template

```bash
# Create template with dependencies
template_rs_cli execute \
    -t api.template_rs \
    -v url="https://api.example.com" \
    -v method="get" \
    -d "reqwest=0.11" \
    -d "tokio=1.0"
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Safety Notes

- Validate template content before execution
- Be cautious with user-provided input
- Review dependencies before execution
- Consider using template sandboxing in production environments