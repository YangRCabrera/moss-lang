# Semantic Constitution

This document defines the core semantic rules of the language. Surface syntax may change freely so long as these rules remain true.

The language is a statically typed, expression-oriented, functional language with explicit mutable references, algebraic data types, pattern matching, and no null value.

Its guiding principle is:

> Ordinary values are immutable. Recoverable uncertainty is represented in types. Mutation is explicit. Programmer assertions may fail catastrophically.

---

## 1. Everything Produces a Value

Every executable construct is an expression and therefore has a type and produces a value.

Blocks evaluate expressions sequentially and evaluate to the value of their final expression.

```
result =
    x = 10
    y = 20
    x + y
```

The block evaluates to `30`.

A binding is not itself an expression. It is a block item that introduces a name scoped over the remainder of the block. Conceptually:

```
x = 10
y = 20
x + y
```

is:

```text
let x = 10 in
let y = 20 in
x + y
```

A block must therefore end in an expression, never a binding.

`Unit` has exactly one value, written `()`.

Every expression in a block other than the final one must have type `Unit`. A value that is not `Unit` may not be silently discarded:

```
parse_config input    # error: Result discarded
_ = parse_config input    # ok: discard is explicit
```

This prevents expected failure (see §10) from disappearing unnoticed.

Apart from bindings, there is no distinction between "statement context" and "expression context" at the semantic level.

---

## 2. Values Are Immutable

Ordinary values do not change after creation.

A binding associates a name with a value. It does not create a mutable storage location.

```
x = 10
```

The meaning of `x` is permanently the value established by that binding.

Reassignment of an ordinary binding is not permitted.

```
x = 10
x = 20    # error
```

Nested scopes may shadow outer bindings:

```
x = 10

result =
    x = 20
    x

# result == 20
# outer x == 10
```

A name may not be defined more than once in the same lexical scope.

---

## 3. Scope Is Lexical

Names are resolved according to lexical program structure.

Functions capture the lexical environment in which they are created.

```
x = 10

add_x =
    fn y => x + y
```

`add_x` continues to refer to the `x` visible at its definition site.

There is no dynamic scoping.

---

## 4. Evaluation Is Strict

Function arguments are evaluated before function application.

Evaluation order is deterministic and proceeds left-to-right unless a construct explicitly specifies otherwise.

For:

```
f (a()) (b())
```

the conceptual order is:

```
evaluate f
evaluate a()
apply f
evaluate b()
apply resulting function
```

This rule becomes observable when effects or mutable references are involved and therefore must not be left unspecified.

There is no implicit lazy evaluation in the core language.

Lazy values may later exist as an explicit library or language abstraction.

### Tail Calls Are Guaranteed

The core language has no loop construct; iteration is expressed through recursion and higher-order functions.

A call in tail position therefore does not consume additional stack space. A tail-recursive function runs in constant stack space regardless of how many times it recurses.

```
fn count_down (n: UInt) -> Unit =
    match n
        0 => ()
        _ => count_down (n - 1)
```

runs in constant stack space for any `n`.

This is a semantic guarantee, not an optional optimization.

---

## 5. Functions Are Unary

Every function accepts exactly one argument and returns exactly one value.

A function type:

```
A -> B
```

means a function from one value of type `A` to one value of type `B`.

A type such as:

```
A -> B -> C
```

associates to the right:

```text
A -> (B -> C)
```

Therefore:

```
add 1 2
```

means:

```
(add 1) 2
```

Partial application requires no special semantics.

### Declarations and Lambdas

A function declaration lists one or more parameter groups. Each parenthesized group is one irrefutable pattern and contributes exactly one unary layer:

```
fn add (a: Int) (b: Int) -> Int =
    a + b
```

declares:

```
add: Int -> Int -> Int
```

A function declaration binds its own name within its body and is therefore recursive (see §30).

An anonymous function always begins with `fn`:

```
fn x => x + 1
```

A declaration introduces its body with `=`. A lambda introduces its body with `=>`.

---

## 6. Multiple Pieces of Input Are Data

When several values conceptually form one argument, they should be represented as one data structure.

For example:

```
distance: (Point, Point) -> Float
```

accepts one tuple containing two points.

This is semantically distinct from:

```
distance: Point -> Point -> Float
```

The second function supports meaningful partial application; the first models its inputs as a single unit.

Because each parameter group is one pattern (§5), the two forms are declared as:

```
fn distance (p: Point, q: Point) -> Float = ...    # (Point, Point) -> Float
fn distance (p: Point) (q: Point) -> Float = ...   # Point -> Point -> Float
```

A comma-separated parameter group is a tuple pattern, not a list of separate arguments.

Tuples, records, lists, and user-defined types are therefore the ordinary mechanism for passing structured input.

---

## 7. Types Are Static

Every expression has a type known before execution.

A successfully compiled program may not encounter a runtime "type error."

There are no implicit conversions between unrelated types.

For example, an `Int` does not automatically become a `Float`, a `String`, or a `Bool`.

Conversions requiring a semantic change must be explicit.

Integer literals take their type from context:

```
a: Ref<UInt> = Ref.new 0    # 0 is a UInt
```

When context does not determine a type, an integer literal defaults to `Int`. This is literal typing, not an implicit conversion: an already-typed `Int` value never becomes a `UInt`.

Type inference may eliminate the need to write types, but it does not weaken static typing.

---

## 8. Generic Functions Are Parametric

Generic type parameters represent arbitrary types.

Given:

```
identity: T -> T
```

the implementation of `identity` may not behave differently merely because `T` happens to be `Int`, `String`, or another type, unless some explicit constraint or capability gives it that information.

The language should prefer parametric polymorphism over implicit runtime type inspection.

Generic type parameters may usually be inferred.

The exact syntax for explicit generic parameters is a surface-language concern.

---

## 9. There Is No `Null`

There is no universal null value.

A type `T` means that a value of type `T` exists.

Expected absence is represented explicitly:

```
Maybe<T>
```

with conceptual constructors:

```
Some T
None
```

Therefore:

```
Person
```

and:

```
Maybe<Person>
```

are meaningfully different types.

Code accepting a `Person` never needs to check whether it secretly contains no person.

---

## 10. Expected Failure Is Data

Operations with ordinary, anticipated failure modes represent failure in their return type.

The conventional type is:

```
Result<T, E>
```

with conceptual constructors:

```
Ok T
Err E
```

Examples include parsing user input, opening a file, performing a network operation, or validating external data.

Failures represented by `Result` are values, not exceptional control flow.

They may be inspected and transformed using ordinary pattern matching and functions.

---

## 11. Programmer Assertions May Trap

Not every operation that can fail must return `Maybe` or `Result`.

Some operations express a programmer assertion that a required condition is true.

Examples include:

```
items[index]
numerator / denominator
```

Indexing asserts that the index is valid.

Integer division and modulo assert that the denominator is nonzero.

Integer arithmetic asserts that the result is representable in its type. Overflow and underflow trap; they never wrap silently.

```
x: UInt = 0
x - 1    # traps
```

Wrapping or saturating arithmetic, if provided, must be requested explicitly through distinct operations.

`%` is modulo, not remainder: for a nonzero divisor, the result takes the sign of the divisor.

```
 7 %  3    #  1
-7 %  3    #  2
 7 % -3    # -2
```

If such an assertion is false, execution `traps`.

A `trap` represents violation of an invariant or programmer expectation, not an ordinary domain-level failure.

The standard library should normally provide recoverable alternatives where useful:

```
List.get: Int -> List<T> -> Maybe<T>
```

alongside trapping convenience operations:

```
items[index]
```

The distinction is intentional:

> Use a recoverable operation when failure is expected. Use a trapping operation when failure would indicate that the program's assumptions were wrong.

In the initial language, `traps` are not catchable.

They terminate program execution.

This may be revisited later, but `Result` must never depend on catchable exceptions for its meaning.

---

## 12. Pattern Matching Is the Fundamental Branching Mechanism

Pattern matching is the core mechanism for inspecting structured values and choosing among alternatives.

Conceptually:

```
match value
    PatternA => expression_a
    PatternB => expression_b
```

The scrutinee is evaluated exactly once.

Patterns are tested from top to bottom.

The expression belonging to the first matching pattern is evaluated.

Patterns themselves do not perform arbitrary computation.

Alternatives may be combined into one or-pattern:

```
match n
    0 | 1 => n
    _     => fib (n - 1) + fib (n - 2)
```

Every alternative of an or-pattern must bind the same names with the same types.

---

## 13. Matches Must Be Exhaustive

A match must cover every possible value of its input type.

For:

```
Maybe<T>
```

a match must account for both:

```
Some value
None
```

unless another pattern subsumes them.

Non-exhaustive matching is a compile-time error.

Patterns that can never match because an earlier pattern already covers them should produce a compiler diagnostic and may eventually be treated as errors.

Exhaustiveness checking is therefore part of the type-checking phase rather than runtime behavior.

---

## 14. Binding Patterns Must Be Irrefutable

Patterns may be used to destructure values while creating bindings.

For example:

```
(x, y) = point
```

is legal because every value of a two-element tuple necessarily has that shape.

Likewise:

```
{ name, age } = person
```

may be legal for an appropriate record type.

A pattern that might fail is not legal as an ordinary binding.

Therefore:

```
Some x = maybe_x
```

is invalid because `maybe_x` might contain `None`.

The programmer must use explicit matching:

```
match maybe_x
    Some x => ...
    None   => ...
```

Ordinary binding never secretly performs a runtime pattern-match failure.

---

## 15. Algebraic Data Types Are Nominal

Variant types create distinct named types.

For example:

```
type Shape =
    | Circle Float
    | Rectangle { width: Float, height: Float }
```

defines a type `Shape`.

Its constructors conceptually have ordinary function types:

```
Circle:
    Float -> Shape

Rectangle:
    { width: Float, height: Float } -> Shape
```

Constructors are values and participate naturally in function application.

```
Circle 10.0
```

does not require a special object-construction semantic model.

---

## 16. Records Are Structural

For the initial language, record types are structural.

A record type:

```
{
    name: String,
    age: Int
}
```

describes any record containing exactly those fields with those types.

A declaration such as:

```
type Person =
    {
        name: String,
        age: Int
    }
```

is therefore a type alias rather than the creation of a new nominal identity.

Two aliases describing the same record structure are type-compatible.

This keeps records simple and makes them useful as lightweight data structures.

Nominal record/newtype semantics may later be added as a separate feature if identity between otherwise identical representations becomes useful.

Records are closed in the initial language: unspecified extra fields are not implicitly accepted.

Row polymorphism is deferred.

---

## 17. Record Values Are Immutable

Creating a modified record produces a new record.

Conceptually:

```
older =
    { person with age = person.age + 1 }
```

does not alter `person`.

---

## 18. Lists Are Immutable and Homogeneous

A `List<T>` contains zero or more values of a single element type `T`.

Lists are immutable values.

Operations that appear to add, remove, or change elements produce new lists.

Implementations are free to use persistent structural sharing.

Mutation of list elements is not a primitive operation.

A mutable collection, if later desired, must be represented explicitly through a mutable abstraction.

---

## 19. Tuples Are Fixed Product Types

A tuple represents a fixed number of ordered values whose types may differ.

```
(Int, String)
```

is distinct from:

```
(String, Int)
```

and from:

```
(Int, String, Bool)
```

A one-element tuple does not exist.

Parentheses around a single expression are grouping:

```
(x)
```

and not tuple construction.

---

## 20. Mutation Exists Only Through References

Mutation is represented explicitly by:

```
Ref<T>
```

A `Ref<T>` is an identity-bearing mutable storage cell containing a value of type `T`.

Creating:

```
counter = Ref.new 0
```

allocates a fresh cell containing `0`.

`Ref.new` is an ordinary function, not a data constructor. Because every call produces a new identity, creation is an effect (§33). `Ref` therefore cannot appear in a pattern.

The binding `counter` itself remains immutable.

It permanently refers to the same cell.

Reading a reference produces its current contained value:

```
counter.*
```

has type:

```
Int
```

if:

```
counter
```

has type:

```
Ref<Int>
```

Writing to a reference changes the contents of that cell.

Conceptually:

```
counter := 10
```

has type:

```
Unit
```

and afterwards:

```
counter.*
```

evaluates to `10`.

Assignment uses `:=`, never `=`. `=` always introduces a binding; `:=` always writes to a cell.

Both forms are syntactic sugar over ordinary functions:

```text
counter.*        ≡  Ref.get counter
counter := 10    ≡  Ref.set counter 10
```

---

## 21. References Have Identity

Copying a reference copies access to the same cell.

```
a = Ref.new 0
b = a

b := 5
```

Afterward:

```
a.*
```

evaluates to:

```
5
```

Creating another reference creates another identity:

```
a = Ref.new 0
b = Ref.new 0
```

`a` and `b` contain equivalent values but denote different cells.

This identity distinction is fundamental to `Ref<T>`.

---

## 22. Mutation Never Propagates Through Ordinary Values

References do not make surrounding immutable values magically mutable.

If a record contains a reference:

```
{
    name = "Ada",
    score = Ref.new 0
}
```

the record itself remains immutable.

The cell identified by its `score` field may change.

This distinction must remain observable and teachable:

> A reference is mutable. A value containing a reference is not thereby mutable as a whole.

---

## 23. Closures May Capture References

Functions may capture `Ref<T>` values through normal lexical closure semantics.

```
counter = Ref.new 0

increment =
    fn _ =>
        counter := counter.* + 1
```

Calling `increment` modifies the captured cell.

No special closure mutation semantics are required.

The function captures an immutable reference value whose referenced cell happens to be mutable.

---

## 24. Namespaces Do Not Create Objects

Namespaces organize names.

Given:

```
namespace Person
    fn age_up (p: Person) -> Person = ...
```

the expression:

```text
Person.age_up
```

performs qualified name lookup.

It does not perform dynamic dispatch.

A namespace associated by convention with a type does not become part of values of that type.

Therefore:

```
Person.age_up person
```

is ordinary function application.

Any future method-call syntax must desugar into ordinary qualified functions rather than introducing hidden per-object method storage.

---

## 25. Field Access and Namespace Access Are Static

Record field access:

```
person.name
```

selects a statically known field.

Namespace access:

```
Person.age_up
```

selects a statically known declaration.

Neither operation performs runtime string-based property lookup.

Dynamic dictionaries or maps are a separate data structure.

---

## 26. Equality Is Type-Directed

The language should not assume that every value is meaningfully comparable for equality.

Equality is available only for types for which equality is defined.

Primitive immutable values such as integers, booleans, and strings may support value equality.

Tuples, records, and algebraic values may support equality when all contained values support equality.

Functions do not support equality.

Reference equality, if exposed, compares reference identity rather than the current contents of the cells.

Automatic equality derivation and the exact mechanism for equality constraints are deferred, but universal "compare any two values" behavior is forbidden.

---

## 27. No Implicit Truthiness

Conditions require `Bool`.

Values such as:

```
0
""
[]
None
```

do not automatically become false.

Within an `if` construct:

```
if condition
    ...
```

`condition` must have type `Bool`.

This keeps control flow type-safe and avoids hidden conversion rules.

---

## 28. `if` Is Convenience Over Boolean Matching

Boolean conditional syntax may exist because it expresses programmer intent clearly.

Semantically:

```text
if condition
    yes
else
    no
```

is equivalent to:

```
match condition
    true  => yes
    false => no
```

Both branches must produce compatible types.

`if` therefore introduces no independent control-flow model.

---

## 29. Pipes Are Ordinary Application

Pipeline syntax introduces no special call semantics.

```
value |> f
```

means exactly:

```
f value
```

Therefore:

```
value
|> normalize
|> validate
|> save
```

is ordinary nested unary function application.

Pipes do not search for argument positions, invoke methods, or introduce implicit placeholders.

Such behavior, if ever wanted, must use a different construct.

---

## 30. Declarations Do Not Depend on Source Mutation

Top-level bindings describe a lexical environment, not a sequence of variable assignments.

Top-level declarations are order-independent: a top-level name is visible throughout its file, including before its declaration.

```
fn main (_args: List<String>) -> Unit =
    greet "world"    # greet is declared below

fn greet (name: String) -> Unit = ...
```

However, cyclic value initialization must never produce observable partially initialized values.

Recursion is introduced only by `fn` declarations. A `fn` declaration's name is in scope within its own body, at top level or inside a block:

```
fn sum_to (n: UInt) -> UInt =
    fn go (i: UInt) (acc: UInt) -> UInt =
        match i
            0 => acc
            _ => go (i - 1) (acc + i)
    go n 0
```

A plain binding is never recursive. In:

```
f = fn x => f x
```

the `f` in the body refers to whatever `f` is visible in the enclosing scope, not to the binding being defined.

A `fn` declaration that refers to itself requires annotated parameter and return types, which keeps static checking straightforward.

Within a block, bindings remain sequential: a local `fn` declaration is visible from its own declaration onward.

General mutually recursive value initialization is deferred.

---

## 31. Errors Are Divided Into Three Categories

The language distinguishes:

### Compile-time errors

Programs rejected before execution.

Examples:

- type mismatch
- undefined name
- duplicate binding in one scope
- non-exhaustive match
- refutable binding pattern
- invalid function application

### Recoverable runtime outcomes

Represented explicitly using normal types.

Examples:

```
Maybe<T>
Result<T, E>
```

These are not language failures.

### Traps

Violations of programmer assertions or runtime invariants.

Examples:

- invalid direct indexing
- division or modulo by zero
- integer overflow or underflow
- stack exhaustion from non-tail recursion
- explicit `trap`
- other operations documented as partial

`traps` terminate execution in the initial language.

---

## 32. The Language Has No Undefined Behavior at the Source Level

A well-typed source program either:

- produces values,
- produces explicitly modeled failure values,
- or `traps` according to defined language rules.

The language specification must not permit arbitrary behavior merely because a programmer violated a runtime precondition.

If direct indexing fails, the specified result is a `trap`—not memory corruption, arbitrary values, or optimizer-dependent behavior.

Implementation bugs are, naturally, outside this guarantee.

---

## 33. Effects Are Observable and Therefore Ordered

Even before an effect system exists, operations such as:

```
print
Ref.new
reference reads
reference writes
```

may be externally observable.

Their order follows the language's strict left-to-right evaluation rules.

A future effect system may describe or constrain these effects, but it must not retroactively change evaluation semantics.

Effect typing is therefore metadata about what computation may do, not a replacement for defining execution order.

---

## 34. The Core Language Should Be Smaller Than the Surface Language

Convenient surface syntax should desugar into a small number of semantic constructs.

Examples:

```text
x |> f
```

desugars into:

```text
f x
```

`if` desugars into Boolean pattern matching.

`r.*` and `r := v` desugar into `Ref.get r` and `Ref.set r v`.

Multi-expression function bodies become block expressions, and block bindings become nested `let` scopes.

This distinction is intentional.

The parser accepts a pleasant language.

The evaluator implements a small language.

---

# Foundational Invariants

The following statements should remain true even as the language grows.

1. A value of type `T` is actually a `T`; absence is never hidden inside ordinary types.
2. Ordinary bindings do not mutate.
3. All mutation is visible through an explicitly mutable abstraction.
4. A `Ref<T>` has identity independent of its contained value.
5. Expected failure is represented as data.
6. Violated programmer assertions `trap` rather than silently producing invalid values.
7. Functions are semantically unary.
8. Function application has one ordinary meaning.
9. Patterns are the fundamental mechanism for decomposing structured values.
10. Matching is exhaustive.
11. Ordinary destructuring cannot secretly fail.
12. Evaluation order is deterministic.
13. There is no implicit truthiness.
14. There are no implicit unrelated type conversions.
15. Namespaces organize functions and values; they do not create an object model.
16. Surface-language conveniences should preferably desugar into a smaller core.
17. The source language has defined behavior even when execution traps.
18. Future effect tracking must describe existing semantics rather than redefine them.
19. Non-`Unit` values are never silently discarded.
20. Tail calls do not grow the stack.
21. Integer arithmetic never silently wraps.
22. Only `fn` declarations are recursive; `=` never refers to the binding it defines.

# Intentionally Unsettled

The following decisions are deliberately not constitutional yet:

- exact numeric types and widths
- whether integer `/` floors or truncates (must stay consistent with `%` being modulo)
- prelude / default scope (examples currently assume names such as `print` and `repeat`)
- syntax for generic type parameters
- typeclass/trait/capability mechanisms
- effect-system notation and exact semantics
- row polymorphism
- catchable `traps`
- asynchronous computation
- concurrency
- method-call sugar
- operator overloading
- user-defined operators
- module/import semantics
- visibility rules
- memory-management strategy
- equality constraints and derivation syntax
- compile-time evaluation
- macros
- whether top-level type annotations are mandatory
- exact type-inference algorithm

These should remain unset until concrete examples demonstrate a need.

# Design Test

When considering a new feature, ask:

1. Can this be expressed using ordinary values, functions, patterns, and types?
2. Does it introduce hidden mutation?
3. Does it introduce hidden failure?
4. Does it make evaluation order less obvious?
5. Does it give ordinary-looking function syntax unusual behavior?
6. Can it be implemented as surface sugar over the existing core?
7. Does the type system accurately describe what may happen at runtime?
8. Is a new keyword or punctuation rule buying enough semantic clarity to justify itself?

A feature need not answer every question favorably.

But an unfavorable answer should be deliberate.
