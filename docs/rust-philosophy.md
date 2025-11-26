# Rust's Philosophy: Why No Traditional OOP?

Rust deliberately chose **not** to include traditional object-oriented programming features like classes and inheritance. This wasn't an oversight—it was a carefully considered design decision based on decades of experience with OOP languages. This document explains the ideology and reasoning behind Rust's approach.

## The Core Philosophy

Rust's design is guided by three main principles:

1. **Memory Safety Without Garbage Collection**
2. **Zero-Cost Abstractions**
3. **Practical Solutions Over Theoretical Purity**

Traditional OOP features often conflict with these goals, so Rust took a different path.

## Why No Classes?

### The Problem with Classes

In traditional OOP languages (Java, C++, Python), classes bundle:
- Data (fields)
- Behavior (methods)
- Inheritance relationships
- Access control
- Initialization logic

This creates several issues:

#### 1. **The Fragile Base Class Problem**

**"Fragile" means: The base class can break your code just by changing its internal implementation, even though your code looks correct.**

This is a **real problem** that happens in production code. Here's a famous example from Java's standard library:

```java
// Real example: Java's HashSet and its subclass
// HashSet internally uses a HashMap and counts elements

class HashSet<E> {
    private HashMap<K,V> map;
    private int size = 0;
    
    public boolean add(E element) {
        // Add to internal map
        boolean added = map.put(element, PRESENT) == null;
        if (added) size++;
        return added;
    }
    
    public boolean addAll(Collection<E> elements) {
        boolean modified = false;
        for (E element : elements) {
            if (add(element)) {  // Calls add() for each element
                modified = true;
            }
        }
        return modified;
    }
}

// Someone creates a counting HashSet
class CountingHashSet<E> extends HashSet<E> {
    private int addCount = 0;  // Track how many adds
    
    @Override
    public boolean add(E element) {
        addCount++;              // Count the add
        return super.add(element);
    }
    
    @Override
    public boolean addAll(Collection<E> elements) {
        addCount += elements.size();  // Count all adds
        return super.addAll(elements);
    }
}

// Usage:
CountingHashSet<String> set = new CountingHashSet<>();
set.addAll(Arrays.asList("one", "two", "three"));
System.out.println(set.addCount);  // Expected: 3, Actual: 6!

// WHY IS IT 6 INSTEAD OF 3? Let's trace the execution:
//
// 1. You call: addAll(["one", "two", "three"])
//
// 2. CountingHashSet.addAll() runs:
//    - addCount += 3  (addCount is now 3)
//    - Calls super.addAll()
//
// 3. HashSet.addAll() runs and loops through items:
//    - Calls add("one")
//
// 4. CountingHashSet.add() runs (you overrode it!):
//    - addCount++  (addCount is now 4)
//    - Calls super.add("one")
//
// 5. HashSet.addAll() continues:
//    - Calls add("two")
//
// 6. CountingHashSet.add() runs again:
//    - addCount++  (addCount is now 5)
//
// 7. HashSet.addAll() continues:
//    - Calls add("three")
//
// 8. CountingHashSet.add() runs again:
//    - addCount++  (addCount is now 6)
//
// Result: Double counting! Each item was counted twice.
//
// THE PROBLEM: You didn't know that HashSet.addAll() internally calls add()!
// This is a hidden implementation detail you had to discover by reading the source code.
```

**Why This is "Fragile":**

> **The base class (`HashSet`) can break your code (`CountingHashSet`) just by changing its internal implementation, even though your code looks correct.**

The `CountingHashSet` author had to know the **internal implementation** of `HashSet.addAll()`. They couldn't just override methods—they had to understand:
- Does `addAll()` call `add()` internally?
- Or does it directly manipulate the internal map?
- This is a **hidden dependency** you shouldn't need to know about!

**What if HashSet changes?**

```java
// Later, HashSet is "optimized"
class HashSet<E> {
    public boolean addAll(Collection<E> elements) {
        // New optimization: bulk insert directly
        boolean modified = map.putAll(elements);
        size += elements.size();
        return modified;
    }
}

// Now CountingHashSet breaks differently!
// addCount is only incremented once (in addAll)
// Individual add() calls are never made
// The counting logic is now wrong again!
```

**This actually happened** in Java's history, and it's why the Java documentation warns against extending collection classes.

**Another Real Example: Android's Fragment lifecycle**

```java
// Android Fragment base class
class Fragment {
    public void onCreate() {
        // Setup code
        initializeViews();
    }
    
    protected void initializeViews() {
        // Default implementation
    }
}

// Your custom fragment
class MyFragment extends Fragment {
    @Override
    protected void initializeViews() {
        // Your view setup
        findViewById(R.id.button).setOnClickListener(...);
    }
}

// Later, Android updates Fragment:
class Fragment {
    public void onCreate() {
        // New: Views aren't ready yet!
        performEarlySetup();
        initializeViews();  // Moved here
        inflateViews();     // Views created AFTER initializeViews!
    }
}

// Now MyFragment crashes!
// findViewById() is called before views exist
// Your app breaks with an Android update
```

This type of breakage has happened countless times in Android development.

---

### How Rust Solves the Fragile Base Class Problem

**Rust uses composition instead of inheritance, which eliminates hidden dependencies.**

Let's rewrite the `CountingHashSet` example in Rust:

```rust
use std::collections::HashSet;

// Instead of extending HashSet, we CONTAIN it
struct CountingHashSet {
    inner: HashSet<String>,  // Composition: has-a HashSet
    add_count: usize,
}

impl CountingHashSet {
    fn new() -> Self {
        CountingHashSet {
            inner: HashSet::new(),
            add_count: 0,
        }
    }
    
    fn add(&mut self, item: String) {
        self.add_count += 1;
        self.inner.insert(item);  // Explicit call to HashSet
    }
    
    fn add_all(&mut self, items: Vec<String>) {
        self.add_count += items.len();  // Count once
        for item in items {
            self.inner.insert(item);    // Explicit: call insert directly
        }
        // No hidden calls! We control exactly what happens.
    }
}

// Usage:
let mut set = CountingHashSet::new();
set.add_all(vec!["one".to_string(), "two".to_string(), "three".to_string()]);
println!("{}", set.add_count);  // Always 3! No surprises!
```

**Why This Solves the Problem:**

1. **No `super` calls** - You don't call parent methods, so you don't depend on their implementation
2. **Explicit control** - YOU decide whether to call `insert()` or `add()` or anything else
3. **No hidden method calls** - There's no way for `HashSet` to secretly call your methods
4. **Implementation changes don't break you** - If `HashSet` changes internally, your code still works because you're calling its public API directly

**Comparison:**

| Inheritance (Fragile) | Composition (Robust) |
|----------------------|---------------------|
| `super.addAll()` - what does this do? | `self.inner.insert()` - explicit call |
| Hidden: does `addAll()` call `add()`? | Explicit: you control all calls |
| Base class changes break you | Base class changes don't affect you |
| Must know implementation details | Only need to know public API |

**The Key Insight:**

> With **inheritance**, you're coupled to the base class's **implementation** (how it works internally).
> 
> With **composition**, you're only coupled to the base class's **interface** (its public methods).

**Example of Robustness:**

Even if Rust's `HashSet` changes its internal implementation completely, your `CountingHashSet` still works:

```rust
// HashSet could change from using a HashMap to using a BTree
// Your code doesn't care! You just call insert()
self.inner.insert(item);  // Works regardless of internal changes
```

With inheritance, you'd be broken by internal changes because you depend on how methods call each other.

---

#### 2. **The Diamond Problem**

```
    Animal
    /    \
  Dog    Cat
    \    /
   DogCat  (???)
```

If `Dog` and `Cat` both override a method from `Animal`, which version does `DogCat` inherit?

Different languages solve this differently:
- C++: Allows it, causes confusion
- Java: Forbids multiple inheritance
- Python: Uses MRO (Method Resolution Order), complex

**The problem:** Multiple inheritance creates ambiguity and complexity.

#### 3. **Tight Coupling**

```java
// Inheritance creates tight coupling
class Vehicle {
    void start() { /* ... */ }
}

class Car extends Vehicle {
    // Car is now forever tied to Vehicle
    // Can't easily change the inheritance hierarchy
}
```

**The problem:** Inheritance creates rigid, hard-to-change hierarchies.

### Rust's Solution: Structs + Impl

```rust
// Separate data from behavior
struct Player {
    name: String,
    health: u32,
}

// Behavior is added separately
impl Player {
    fn new(name: String) -> Self {
        Player { name, health: 100 }
    }
    
    fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }
}
```

**Benefits:**
- ✅ Clear separation of data and behavior
- ✅ No hidden inheritance relationships
- ✅ Easy to understand what a type does
- ✅ No fragile base class problem
- ✅ Explicit ownership and borrowing

## Why No Inheritance?

### The Problems with Inheritance

#### 1. **Inheritance is Often Misused**

The classic example: "A square is a rectangle"

```java
// Seems logical, but causes problems
class Rectangle {
    protected int width;
    protected int height;
    
    void setWidth(int w) { width = w; }
    void setHeight(int h) { height = h; }
}

class Square extends Rectangle {
    @Override
    void setWidth(int w) {
        width = w;
        height = w;  // Keep it square!
    }
    
    @Override
    void setHeight(int h) {
        width = h;
        height = h;
    }
}

// This breaks!
Rectangle rect = new Square();
rect.setWidth(5);
rect.setHeight(10);
// Expected: 5x10 rectangle
// Actual: 10x10 square
```

**The problem:** Inheritance violates the Liskov Substitution Principle. A `Square` can't truly substitute for a `Rectangle`.

#### 2. **Deep Inheritance Hierarchies**

```
Object
  └─ Component
      └─ Container
          └─ Panel
              └─ Applet
                  └─ MyApplet
```

**Problems:**
- Hard to understand what `MyApplet` actually does
- Changes at any level can break everything below
- Difficult to test in isolation
- Encourages code reuse through inheritance (wrong tool!)

#### 3. **Implementation Inheritance vs Interface Inheritance**

```java
// Interface inheritance (good)
interface Drawable {
    void draw();
}

// Implementation inheritance (problematic)
class Shape {
    void draw() { /* default implementation */ }
}

class Circle extends Shape {
    // Inherits implementation - tight coupling!
}
```

**The problem:** Inheriting implementation creates dependencies on internal details.

### Rust's Solution: Composition + Traits

```rust
// Composition: Build complex types from simple ones
struct Engine {
    horsepower: u32,
}

struct Wheels {
    count: u8,
}

struct Car {
    engine: Engine,    // Has-a relationship
    wheels: Wheels,    // Not is-a relationship
}

// Traits: Define shared behavior
trait Drivable {
    fn drive(&self);
}

impl Drivable for Car {
    fn drive(&self) {
        println!("Driving with {} HP", self.engine.horsepower);
    }
}
```

**Benefits:**
- ✅ Flexible: Easy to change composition
- ✅ Clear: Explicit relationships
- ✅ Testable: Components can be tested independently
- ✅ Reusable: Traits can be implemented for any type

## The "Composition Over Inheritance" Principle

This isn't unique to Rust—it's a well-established principle in software engineering.

### Gang of Four (Design Patterns book, 1994):

> "Favor object composition over class inheritance."

### Why Composition is Better

#### Inheritance Says: "IS-A"
```
Dog IS-A Animal
```
- Rigid relationship
- Hard to change
- Creates tight coupling

#### Composition Says: "HAS-A"
```
Dog HAS-A name
Dog HAS-A age
Dog HAS-A ability to bark
```
- Flexible relationship
- Easy to change
- Loose coupling

### Real-World Example

**Bad (Inheritance):**
```java
class Employee { }
class Manager extends Employee { }
class Engineer extends Employee { }
class ManagerEngineer extends ??? // Problem!
```

**Good (Composition):**
```rust
struct Employee {
    name: String,
    roles: Vec<Role>,  // Can have multiple roles!
}

enum Role {
    Manager,
    Engineer,
    Designer,
}

impl Employee {
    fn add_role(&mut self, role: Role) {
        self.roles.push(role);
    }
}
```

## Why Traits Instead of Interfaces?

Traits are more powerful than traditional interfaces:

### 1. **Traits Can Have Default Implementations**

```rust
trait Logger {
    fn log(&self, message: &str) {
        println!("[LOG] {}", message);  // Default implementation
    }
    
    fn error(&self, message: &str) {
        println!("[ERROR] {}", message);  // Default implementation
    }
}

// Can use defaults or override
struct MyLogger;

impl Logger for MyLogger {
    // Use default log(), override error()
    fn error(&self, message: &str) {
        eprintln!("!!! ERROR: {}", message);
    }
}
```

### 2. **Traits Can Be Implemented for External Types**

```rust
// You can implement traits for types you don't own!
trait Summarize {
    fn summary(&self) -> String;
}

// Implement for String (from std library)
impl Summarize for String {
    fn summary(&self) -> String {
        format!("String with {} chars", self.len())
    }
}

// This is impossible with traditional interfaces!
```

### 3. **Traits Enable Zero-Cost Abstractions**

```rust
// Static dispatch (compile-time, no runtime cost)
fn process<T: Drawable>(item: &T) {
    item.draw();
}

// Compiler generates specialized versions:
// process_for_Circle(item: &Circle) { item.draw(); }
// process_for_Rectangle(item: &Rectangle) { item.draw(); }
```

**Result:** Abstraction with **zero runtime cost**!

## The Ownership System's Role

Rust's ownership system makes traditional OOP patterns problematic:

### Problem: Shared Mutable State

```java
// Java: Easy to share mutable state (dangerous!)
class Node {
    Node parent;
    List<Node> children;
}

// Can create cycles, memory leaks, data races
```

```rust
// Rust: Ownership prevents this
struct Node {
    parent: Box<Node>,      // Can't have cycles!
    children: Vec<Node>,
}

// Compiler error: Can't have multiple owners
// This forces you to think about ownership
```

### Solution: Explicit Ownership

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

// Explicit: Rc = shared ownership, RefCell = interior mutability
// Verbose, but safe and clear!
```

**Philosophy:** Make the costs visible. If sharing is complex, the code should reflect that.

## Zero-Cost Abstractions

Rust's motto: "You don't pay for what you don't use."

### Traditional OOP Costs

```java
// Java: Virtual method calls have runtime cost
class Animal {
    void makeSound() { }  // Virtual by default
}

class Dog extends Animal {
    @Override
    void makeSound() { System.out.println("Woof"); }
}

// Runtime cost: vtable lookup for every call
animal.makeSound();
```

### Rust's Approach

```rust
// Static dispatch (default, zero cost)
trait Animal {
    fn make_sound(&self);
}

fn call_animal<T: Animal>(animal: &T) {
    animal.make_sound();  // Resolved at compile time!
}

// Dynamic dispatch (opt-in, when needed)
fn call_animal_dyn(animal: &dyn Animal) {
    animal.make_sound();  // Runtime cost, but explicit
}
```

**Philosophy:** 
- Default to zero-cost (static dispatch)
- Make runtime costs explicit (`dyn`)
- Let the programmer choose

## Practical Over Theoretical: The "Regular Dude" Approach

**Rust does reusability the way a regular person would think about it, not the academic way.**

If you asked a non-programmer: "How would you reuse code from a HashSet to make a counting version?"

They'd probably say: "Just put a HashSet inside it and keep a counter. When you add stuff, update the counter and add to the HashSet."

That's **exactly** what Rust does! Composition is the common-sense approach.

### Academic/Theoretical Approach (Traditional OOP):
```
"A CountingHashSet IS-A HashSet, therefore it should inherit from HashSet.
We'll use polymorphism and the Liskov Substitution Principle..."
```
- Sounds smart
- Creates hidden dependencies
- Breaks in practice
- Requires understanding complex theory

### Practical Approach (Rust):
```
"A CountingHashSet HAS-A HashSet inside it, plus a counter.
Just wrap it and add your counting logic."
```
- Sounds simple
- No hidden dependencies
- Works reliably
- Just makes sense

### Why Rust Feels "Natural"

Rust's approach matches how you'd explain it to someone:

**Inheritance explanation:**
> "Well, you see, when you extend a class, you inherit its methods, but you can override them, and when you call super, it invokes the parent implementation, but you have to be careful about which methods call which other methods internally, and there's this thing called the fragile base class problem..."

**Composition explanation:**
> "You put a HashSet inside your struct. When someone calls your method, you update your counter and then call the HashSet's method. That's it."

One requires a PhD to explain. The other is just... obvious.

### Rust Chooses Simplicity

Rust isn't trying to be a "pure" OOP or functional language. It's pragmatic and picks what works:

**From OOP:** Encapsulation, methods on types
**From Functional:** Immutability by default, pattern matching
**Rejects:** Inheritance (too complex), mandatory purity (too restrictive)

**The philosophy:** If it makes sense to a regular developer, it's probably good design.

### Can You Do This in Java/C++?

**Yes! You absolutely can use composition instead of inheritance in any language.**

```java
// Java - Using composition (the "Rust way")
class CountingHashSet<E> {
    private HashSet<E> inner;  // Composition, not inheritance!
    private int addCount = 0;
    
    public CountingHashSet() {
        this.inner = new HashSet<>();
    }
    
    public boolean add(E element) {
        addCount++;
        return inner.add(element);  // Explicit delegation
    }
    
    public boolean addAll(Collection<E> elements) {
        addCount += elements.size();
        for (E element : elements) {
            inner.add(element);  // Explicit control
        }
        return true;
    }
    
    public int getAddCount() {
        return addCount;
    }
}

// Now it works correctly!
CountingHashSet<String> set = new CountingHashSet<>();
set.addAll(Arrays.asList("one", "two", "three"));
System.out.println(set.getAddCount());  // 3 (correct!)
```

**This is actually recommended practice in Java!** The Gang of Four said "Favor composition over inheritance" back in 1994.

### So Why Does Rust Matter?

**The difference:** In Java/C++, you have a choice. In Rust, you're **guided** toward the better pattern.

| Language | Inheritance | Composition | Result |
|----------|-------------|-------------|--------|
| **Java/C++** | Easy, default | Verbose, manual | Most people use inheritance (fragile) |
| **Rust** | Impossible | Natural, enforced | Everyone uses composition (robust) |

**In Java:**
- Inheritance is the "easy" path (just add `extends`)
- Composition requires more boilerplate
- Developers often choose inheritance for convenience
- Results in fragile code

**In Rust:**
- Inheritance doesn't exist
- Composition is the only option
- No choice = no fragile code
- Forces you into the better pattern

### The Language Design Insight

> **Rust doesn't just recommend good practices—it makes them the only option.**

You *can* write good Java code using composition. But Rust *ensures* you write good code by removing the bad option entirely.

**It's like:**
- Java: "Here's a knife and a gun. Please use the knife responsibly."
- Rust: "Here's a knife. There is no gun."

### What Rust Adds Beyond "Just Use Composition"

Even if you use composition in Java, Rust adds:

1. **Ownership system** - Prevents data races and memory bugs
2. **Zero-cost abstractions** - Composition has no runtime overhead
3. **Traits** - More powerful than interfaces (can implement for external types)
4. **Pattern matching** - Better than inheritance for variants
5. **No null** - Uses `Option<T>` instead

So Rust isn't just "Java with composition enforced"—it's a whole different approach to safe, fast systems programming.

### What Rust Borrowed from OOP:
- ✅ Encapsulation (private fields)
- ✅ Polymorphism (traits)
- ✅ Methods on types (impl blocks)

### What Rust Rejected from OOP:
- ❌ Inheritance
- ❌ Classes
- ❌ Implicit virtual methods
- ❌ Null references (uses Option instead)

### What Rust Borrowed from Functional Programming:
- ✅ Immutability by default
- ✅ Pattern matching
- ✅ Closures
- ✅ Algebraic data types (enums)

### What Rust Rejected from Functional Programming:
- ❌ Mandatory purity
- ❌ No mutation (allows `mut`)
- ❌ Lazy evaluation by default

## The Rust Way: Explicit is Better

Rust values **explicitness** over **convenience**:

### Explicit Mutability
```rust
let x = 5;        // Immutable by default
let mut y = 5;    // Explicit mutability
```

### Explicit Ownership
```rust
fn take_ownership(s: String) { }      // Takes ownership
fn borrow(s: &String) { }             // Borrows
fn borrow_mut(s: &mut String) { }     // Mutable borrow
```

### Explicit Error Handling
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
// No exceptions, no hidden control flow
```

### Explicit Costs
```rust
// Static dispatch (fast, explicit)
fn process<T: Trait>(item: T) { }

// Dynamic dispatch (flexible, explicit cost)
fn process_dyn(item: Box<dyn Trait>) { }
```

**Philosophy:** If something has a cost (performance, complexity, safety), make it visible in the code.

## Lessons from History

Rust learned from decades of OOP experience:

### 1990s-2000s: OOP Dominance
- Java, C++, C# popularize OOP
- Deep inheritance hierarchies become common
- "Everything is an object" philosophy

### Problems Emerged:
- Fragile base classes
- Tight coupling
- Hard to test
- Hard to reason about
- Performance issues (virtual calls)

### Industry Response (2000s-2010s):
- "Composition over inheritance" becomes best practice
- Interfaces preferred over abstract classes
- Dependency injection frameworks
- Functional programming gains popularity

### Rust's Approach (2010s):
- Learn from OOP's mistakes
- Build better primitives from the start
- Composition and traits, not inheritance
- Explicit over implicit
- Zero-cost abstractions

## Comparison with Other Modern Languages

| Language | Approach | Philosophy |
|----------|----------|------------|
| **Rust** | Structs + Traits + Composition | Explicit, zero-cost, safe |
| **Go** | Structs + Interfaces | Simple, practical |
| **Swift** | Classes + Protocols | OOP + functional hybrid |
| **Kotlin** | Classes + Interfaces | Better Java |
| **TypeScript** | Classes + Interfaces | JavaScript with types |

Rust is unique in completely rejecting inheritance while maintaining type safety and zero-cost abstractions.

## The Benefits of Rust's Approach

### 1. **Easier to Understand**
```rust
// What does this type do? Look at the impl blocks!
struct Player { }

impl Player {
    fn new() -> Self { }
    fn move_to(&mut self, x: i32, y: i32) { }
}

// No hidden inherited methods
// No need to traverse a class hierarchy
```

### 2. **Easier to Change**
```rust
// Want to add new behavior? Add a new trait!
trait Serializable {
    fn serialize(&self) -> String;
}

impl Serializable for Player {
    fn serialize(&self) -> String { /* ... */ }
}

// No need to modify existing code
// No risk of breaking inheritance chains
```

### 3. **Better Performance**
```rust
// Static dispatch by default
fn process<T: Trait>(item: T) {
    item.method();  // Inlined at compile time!
}

// No vtable lookups
// No runtime overhead
// Compiler can optimize aggressively
```

### 4. **Safer Code**
```rust
// Ownership prevents:
// - Data races
// - Use-after-free
// - Double-free
// - Null pointer dereferences

// Traits prevent:
// - Fragile base classes
// - Diamond problems
// - Tight coupling
```

## When You Might Miss Traditional OOP

### Scenario: Deep Type Hierarchies

If you're modeling something with natural hierarchies:

```
GUI Framework:
  Widget
    ├─ Button
    ├─ TextBox
    └─ Container
        ├─ Panel
        └─ Window
```

**In OOP:** Easy with inheritance
**In Rust:** Use composition + traits (more verbose, but more flexible)

### Rust's Response:

"If the hierarchy is complex, the code should reflect that complexity. Making it look simple (via inheritance) doesn't make it actually simple—it just hides the complexity."

## The Bottom Line

Rust's rejection of traditional OOP isn't ideological purity—it's based on practical experience:

1. **Inheritance causes more problems than it solves**
2. **Composition is more flexible and maintainable**
3. **Traits are more powerful than interfaces**
4. **Explicit costs lead to better performance**
5. **Ownership makes shared mutable state explicit**

**Rust's philosophy:** Give programmers powerful, safe tools and let them build the abstractions they need, rather than forcing a particular paradigm.

## Quotes from Rust Designers

### From the Rust Book:
> "Rust takes a different approach to OOP than other languages. We'll explore how to use traits, which are similar to interfaces in other languages, and how to use trait objects to enable polymorphism."

### From Rust RFC discussions:
> "Inheritance is not a good fit for Rust's ownership system. Composition and traits provide the same benefits without the drawbacks."

## Resources

- **Rust Book - OOP Features**: https://doc.rust-lang.org/book/ch17-00-oop.html
- **Composition Over Inheritance**: https://en.wikipedia.org/wiki/Composition_over_inheritance
- **Gang of Four Design Patterns**: Classic book on OOP best practices
- **Why Inheritance is Bad**: Numerous blog posts and papers from the 2000s-2010s

## Summary

Rust doesn't have traditional OOP because:

1. ✅ **Inheritance causes fragile base classes and tight coupling**
2. ✅ **Composition is more flexible and maintainable**
3. ✅ **Traits are more powerful than interfaces**
4. ✅ **Ownership makes shared mutable state problematic**
5. ✅ **Zero-cost abstractions require static dispatch**
6. ✅ **Explicit is better than implicit**
7. ✅ **Learn from 30+ years of OOP experience**

**The Rust way:** Composition + Traits + Explicit Ownership = Safe, Fast, Maintainable Code 🦀
