# Go Style Guide Summary

Practical summary of the style rules applied in this project. Authoritative sources:

- [Effective Go](https://go.dev/doc/effective_go)
- [Google Go Style Guide](https://google.github.io/styleguide/go/guide)
- [Google Go Best Practices](https://google.github.io/styleguide/go/best-practices)
- [Google Go Decisions](https://google.github.io/styleguide/go/decisions)

Compliance is enforced mechanically by `make lint` (`golangci-lint`, config in `.golangci.yml`).

---

## Formatting

- **`gofmt` is mandatory.** All Go code must be formatted with `gofmt`. Never work around its output — rearrange the code instead.
- **Tabs for indentation**, not spaces.
- **No line length limit**, but wrap lines that are unreasonably long, indenting wrapped lines with an extra tab.
- **Opening braces on the same line** as the control structure — Go's semicolon insertion rules require this.
- **Comment line width**: aim for ~80 columns, but readability takes priority.

---

## Naming

### General

- **MixedCaps** for all identifiers — no underscores in names (`MaxRetries`, not `max_retries`).
- Names should be **short, concise, and evocative**. Err on the side of brevity.

### Package names

- **Lowercase, single word**, no underscores or MixedCaps: `cmdutil`, not `cmd_util`.
- The package name is the base name of its source directory.
- Avoid generic, uninformative names: `util`, `helper`, `common`, `model`.
- Avoid shadowing commonly used variable names.

### Initialisms

- Keep the same case throughout: `URL` or `url`, never `Url`; `HTTP` or `http`, never `Http`; `ID` or `id`, never `Id`.
- In multi-initialism names, each initialism is consistently cased: `XMLAPI` not `XmlAPI`.

### Variable names

- **Length proportional to scope** and inversely proportional to usage frequency:
  - Small scope (1–7 lines): single letters acceptable.
  - Medium scope (8–15 lines): one or two short words.
  - Large scope (15–25 lines): longer, descriptive names.
- Single-letter variables are idiomatic for: receiver names, loop indices (`i`, `j`), coordinates (`x`, `y`), familiar reader/writer types (`r`, `w`).
- Don't drop letters just to save typing: `Sandbox`, not `Sbx`.
- Don't embed the type in the name: `userCount`, not `numUsers` or `usersInt`.
- Add words to disambiguate similar names: `userCount` vs. `projectCount`.

### Receiver names

- Short, one or two characters, abbreviating the type: `func (c *Config)`, not `func (config *Config)`.
- **Consistent** across all methods for a given type.

### Constant names

- MixedCaps like all other identifiers — not `ALL_CAPS`.
- Name constants based on their **role**, not their literal value.

### Getters

- Don't use a `Get` or `get` prefix for simple accessor functions. Start directly with the noun: `Counts()`, not `GetCounts()`.
- Use `Compute` or `Fetch` for expensive or remote operations.

### Avoiding repetition (stutter)

- Don't repeat the package name in exported symbol names: `widget.New`, not `widget.NewWidget`.
- Don't repeat the receiver type in method names: `WriteTo`, not `WriteConfigTo`.
- Don't restate what the return type already makes obvious.

### Context key types

- Use an **unexported empty struct** as a context key type to prevent collisions:
  ```go
  type contextKey struct{}
  ```
  Never use a string or exported type as a context key.

---

## Comments and documentation

### Doc comments

- **All exported symbols and packages must have godoc comments.**
- Package comment: `// Package foo does X.`
- Function/type/method comment: `// FunctionName does X.` — starts with the symbol name, no blank line between comment and declaration.
- Comments are **complete sentences**, capitalised, and end with a **period**.
- End-of-line field comments may be shorter phrases; struct field groups may share one comment.

### Parameter documentation

- Document non-obvious or error-prone parameters — explain *why* they matter, not what the type is.
- Context cancellation behaviour is implied; only document when the function behaves differently from `ctx.Err()`.

### Concurrency documentation

- Assume read-only operations are safe for concurrent use without extra synchronisation.
- Document concurrency safety (or lack of it) for mutating operations.

### Cleanup documentation

- Document any explicit cleanup that callers are responsible for (e.g., `Close`, `Cancel`).

### Error documentation

- Document significant sentinel errors and error types your functions return.
- Document package-wide error conventions in the package doc comment where widely applicable.

### Signal-boosting comments

- Add a comment when code is non-obvious or inverts a common pattern:
  ```go
  if err := doSomething(); err == nil { // if NO error
  ```

---

## Errors

### Error strings

- **Lowercase, no trailing punctuation**: `"no token found"`, not `"No token found."`.
- Exception: proper nouns, acronyms, and exported names may be capitalised.

### `errors.New` vs `fmt.Errorf`

- Use **`errors.New`** for static strings with no format verbs.
- Use **`fmt.Errorf`** only when formatting or wrapping is needed.

### Error wrapping

- Use **`%w`** (not `%v`) to wrap errors so callers can use `errors.Is` / `errors.As`.
- Place `%w` at the **end** of format strings: `"reading file: %w"`.
- Use `%v` only when you deliberately want to break the error chain (e.g., at a system boundary like an RPC layer).

### Error comparison

- Use **`errors.Is`** for comparisons, not `==` or deprecated helpers like `os.IsNotExist`.
  ```go
  // correct
  if errors.Is(err, os.ErrNotExist) { ... }
  // wrong
  if os.IsNotExist(err) { ... }
  ```

### Handling errors

- Make a **deliberate choice**: handle immediately, return to caller, or log and exit.
- **Never discard errors silently** with `_`. If discarding is genuinely safe, add an explanatory comment:
  ```go
  // MarkFlagRequired only errors if the flag name is unknown, which cannot
  // happen here since we just registered "id" on this command.
  _ = cmd.MarkFlagRequired("id")
  ```
- Don't add error annotations that merely restate failure without adding context.

### In-band errors

- Don't signal errors via special return values (`-1`, `nil`, `""`). Use a separate `error` return value.

### Sentinel errors

- Use unparameterised global `var` values for simple structured errors that callers compare directly.
- Use `errors.Is` when the error may be wrapped.

### Panics

- Use `error` returns for normal error handling — **don't panic**.
- Panics are acceptable only when an unrecoverable internal invariant has been violated (a bug, not a user error).
- In `package main` or `init`, consider `log.Fatal` or `log.Exit` for fatal startup errors.
- If using panic internally (e.g., in deeply recursive parsers), always recover before crossing a package boundary.

---

## Control flow — indent error flow

Handle errors and special cases first; keep the happy path at the lowest indent level.

```go
// correct — flat, errors first
data, err := os.ReadFile(path)
if errors.Is(err, os.ErrNotExist) {
    return &Config{}, nil
}
if err != nil {
    return nil, fmt.Errorf("reading %s: %w", path, err)
}
// happy path continues unindented

// wrong — nested if pushes happy path right
if err != nil {
    if os.IsNotExist(err) {
        return &Config{}, nil
    }
    return nil, err
}
```

- Prefer early return over `else` after an `if` block that returns.
- Don't use `if`-with-initialiser for variables needed many lines ahead.

---

## Imports

Three groups, separated by blank lines: **stdlib**, **third-party**, **internal (project)**.

```go
import (
    "context"
    "fmt"

    netbox "github.com/netbox-community/go-netbox/v4"
    "github.com/modelcontextprotocol/go-sdk/mcp"

    "github.com/kirtis/netbox-mcp/internal/config"
)
```

- **Don't rename imports** unless avoiding a genuine name collision. If you must, the local name must follow package naming rules (lowercase).
- **Don't use `import .`** — it obscures where identifiers come from.
- **`import _` only in `main` packages or tests**, not in library packages.
- Protocol buffer packages: rename with a `pb` suffix (`foopb "path/to/foo_go_proto"`).

---

## Context

- `context.Context` is always the **first parameter**, conventionally named `ctx`.
- **Never store a context in a struct field.** Pass it as a parameter.
- **Never create custom context types.** Use `context.WithValue` with an unexported key type.
- Pass the context through; don't use `context.Background()` inside library or command code.
- Context cancellation interrupting a function is implied; only document divergence from `ctx.Err()`.

---

## Variable declarations

- **Prefer `:=`** over `var` when initialising a new variable with a non-zero value.
- Use **`var`** declarations to convey an intentional zero value ready for later use:
  ```go
  var coords Point
  json.Unmarshal(data, &coords)
  ```
- **Maps must be explicitly initialised** before modification; reading from a nil map is safe.
- Use composite literals when you know the initial elements; avoid them for empty values.
- Preallocate slices/maps only when the final size is known and performance analysis justifies it.

---

## Literal formatting

- **Field names are required** in struct literals for types defined outside the current package.
- **Omit zero-value fields** from struct literals when clarity is not lost.
- In multi-line literals, the **closing brace goes on its own line** with a trailing comma after the last element:
  ```go
  // correct
  s := MyStruct{
      Field: value,
  }
  ```
- **Omit repeated type names** in slice/map literals (`gofmt -s` does this automatically).

---

## Interfaces

- Interfaces belong in the **consuming package**, not the implementing package.
- Implementing packages return **concrete types**, not interfaces.
- **Don't define an interface before you have a realistic use case.** Add it when needed.
- Don't use interface parameters unless callers genuinely pass different implementations.
- Don't export unnecessary interfaces.

---

## Receiver type (value vs. pointer)

Use a **pointer receiver** when:
- The method mutates the receiver.
- The receiver contains fields that must not be copied (`sync.Mutex`, etc.).
- The struct is large enough that copying is expensive.
- The struct contains pointer fields that the method needs to mutate.

Use a **value receiver** when:
- The receiver is a small, naturally-valued, immutable struct.
- The receiver is a map, function, or channel.
- You don't want concurrent modifications to be visible.

**When in doubt, use a pointer receiver.** Keep all methods for a type consistently pointer or consistently value.

---

## Function argument lists

- Keep function signatures short. Many parameters make individual roles unclear and arguments easier to confuse.
- **Strategies for complex functions**: split into simpler functions, extract local variables, or use an option struct.
- Use an **option struct** when many callers need several options:
  ```go
  type Options struct {
      Timeout  time.Duration
      Retries  int
  }
  ```
  Option structs are self-documenting at the call site and can grow without breaking callers.
- Don't put contexts in option structs.

---

## Goroutine lifetimes

- Make clear **when and whether goroutines exit**. Leaking goroutines causes unbounded memory growth and data races.
- Use `context.Context` to manage goroutine lifetime.
- Constrain synchronisation within function scope where possible.
- Prefer **synchronous functions** that return results directly; callers can add concurrency by calling in a goroutine if needed.

---

## Copying

- **Don't copy synchronisation types** (`sync.Mutex`, `sync.WaitGroup`, etc.). Pass by pointer.
- Don't copy types with pointer-method receivers.
- Structs containing uncopyable fields must be passed and returned as pointers.

---

## Pass values vs. pointers

- **Don't pass a pointer just to avoid copying** — copy semantics are often clearer and cheaper than pointer indirection for small types.
- Do use pointers for large structs, protocol buffers, or any type containing uncopyable fields.

---

## Switch statements

- **Don't add `break` at the end of switch cases** — it is the default behaviour and is redundant.
- Use a comment for an intentionally empty case.
- To break from an enclosing `for` loop, use a labelled `break`.

---

## `Must` functions

- Name them `MustXxx` (exported) or `mustXxx` (unexported).
- Only call in package `init`, startup, or test helpers — **never on user input**.
- In test helpers: mark with `t.Helper()` and call `t.Fatal` on failure.

---

## Generics

- Use generics where they genuinely reduce duplication for multiple concrete types.
- **Don't use generics for a single type** — add them later when the need arises.
- Avoid using generics to build domain-specific languages or error-handling frameworks.

---

## Use `%q`

- Use **`%q`** to print strings with quotation marks and escape control characters — preferred over manual quoting with `%s`.

---

## Use `any`

- Prefer **`any`** over `interface{}` in new code — `any` is the standard alias.

---

## Cryptography

- **Never use `math/rand` for secrets or keys.** Use `crypto/rand.Reader`.
- Encode cryptographic bytes as hex or base64.

---

## Testing

### Table-driven tests

- Use **table-driven tests** when multiple cases share the same structure. Define a slice of structs with `name`, inputs, and expected outputs, then range over them with `t.Run`.
- Use **field names in struct literals** for test cases (self-documenting, prevents field-swap bugs, enables omitting zero values).

### Subtests and helpers

- Use `t.Run` for subtests.
- Test helpers must call **`t.Helper()`** as their first statement so failures are attributed to the call site.
- Use **`t.Setenv`** (not `os.Setenv`) for environment variable isolation.
- Use **`t.TempDir()`** for temporary files and directories.
- Use **`t.Cleanup`** to register cleanup functions.

### Failure messages

- Follow **`"got X, want Y"`** convention for failure messages.
- Always include the function name and inputs in failure messages: `YourFunc(%v) = %v, want %v`.
- Print actual value before expected; include a clear diff direction label.

### `t.Error` vs. `t.Fatal`

- Prefer **`t.Error`** — tests should generally continue after a failure to surface all problems at once.
- Use **`t.Fatal`** only when the test truly cannot continue (e.g., required setup failed).
- In table-driven test loops: use `t.Error` and `continue`, not `t.Fatal`.
- Inside `t.Run` subtests: `t.Fatal` ends only the current subtest, which is fine.

### Don't call `t.Fatal` from goroutines

- **Never call `t.Fatal`, `t.FailNow`, etc. from a goroutine other than the test function.** Use `t.Error` and return instead. The main test goroutine handles fatal failures.

### Black-box test packages

- Use the **`package foo_test`** naming convention to test only the public API, preventing reliance on unexported internals.

### Assertion libraries

- **Don't create or use custom assertion libraries.** Use standard `cmp.Equal` / `cmp.Diff` for structural comparisons.
- Use `t.Error` / `t.Fatal` directly with descriptive messages.

### Test helpers vs. assertion helpers

- **Test helpers** perform setup/teardown — failures indicate environment problems, not code bugs.
- **Assertion helpers** check correctness — not idiomatic in Go; inline the validation logic or return an `error` instead.

---

## Linting

`make lint` runs `golangci-lint` with the following linters enabled beyond the standard set:

| Linter | Purpose |
|---|---|
| `bodyclose` | HTTP response body must be closed. |
| `dupword` | Duplicate words in comments or strings. |
| `errorlint` | Correct use of Go 1.13 error wrapping (`errors.Is`, `%w`). |
| `fatcontext` | Context not stored in loops or closures. |
| `gocritic` | Broad set of style and correctness checks. |
| `godot` | Comments must end with a period. |
| `gosec` | Common security issues. |
| `misspell` | Spelling in comments and strings. |
| `revive` | Idiomatic Go; replaces `golint`. |

Use `//nolint:<linter> // reason` to suppress a specific false positive with a mandatory explanation.
