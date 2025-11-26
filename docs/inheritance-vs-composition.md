# Inheritance vs Composition: The Car Engine Analogy

## The Goal: Code Reuse Without Repetition

Both inheritance and composition aim to share logic across systems so we don't repeat ourselves. The difference is **how** they achieve this.

## The Car Engine Analogy

### Inheritance: Tweaking the Engine Itself

**Inheritance is like building a car by modifying the engine to fit your car.**

When you extend a class (inherit), you're reaching into the base class and tweaking its internals:
- You override methods (modify how the engine works)
- You call `super` methods (depend on the engine's internal behavior)
- You change the engine's behavior to fit your needs

**The problem:** By tweaking the engine itself, you can lose the engine's functionality guarantees. The engine was designed to work a certain way, and your modifications might break those assumptions. You're coupled to the engine's **implementation details**, not just its interface.

**Example:**
```java
class Engine {
    void start() {
        ignite();
        warmUp();
    }
}

class TurboEngine extends Engine {
    @Override
    void start() {
        super.start();  // Tweaking: depends on what start() does internally
        boostTurbo();   // Adding behavior by modifying the engine
    }
}
```

If `Engine.start()` changes how it calls `ignite()` or `warmUp()`, your `TurboEngine` might break.

### Composition: Building Around the Engine

**Composition is like building a car by leaving the engine as-is and building the system around it.**

When you use composition, you don't modify the engine at all:
- You keep the engine intact (no overriding)
- You build your car's systems to work with the engine's interface
- You use the engine's public API without depending on its internals

**The benefit:** The engine's functionality guarantees remain intact. You're only coupled to the engine's **public interface**, not its implementation. If the engine changes internally, your car still works as long as the interface stays the same.

**Example:**
```rust
struct Engine {
    // Engine internals
}

impl Engine {
    fn start(&self) {
        // Engine's own logic
    }
}

struct TurboCar {
    engine: Engine,  // Contains an engine, doesn't modify it
    turbo: Turbo,
}

impl TurboCar {
    fn start(&mut self) {
        self.engine.start();  // Use the engine as-is
        self.turbo.boost();   // Build your system around it
    }
}
```

The engine remains unchanged. Your car adapts to work with the engine, not the other way around.

## The Trade-off

**Yes, there is a trade-off:** Sometimes building your system around an unmodified component can produce complexity. You might need adapter layers, delegation methods, or wrapper logic.

**However:**
- This complexity is **explicit** and **visible** in your code
- You maintain the guarantees of the components you're using
- Changes to those components are less likely to break your system
- The complexity is in your code (which you control), not in hidden dependencies

**Inheritance hides complexity** (in the form of hidden dependencies and fragile base classes).
**Composition makes complexity explicit** (in the form of adapter code and delegation).

Explicit complexity is better than hidden fragility.

## Summary

| Approach | Metaphor | What You Do | Coupling | Risk |
|----------|----------|-------------|----------|------|
| **Inheritance** | Modify the engine to fit your car | Override methods, call `super` | Coupled to implementation | Fragile: engine changes break you |
| **Composition** | Build your car around the engine | Use the engine's public API | Coupled to interface only | Robust: engine internals can change |

**The Rust philosophy:** Accept the explicit complexity of composition to avoid the hidden fragility of inheritance.

## When Composition Feels Like Too Much Work

If building your system around a component creates excessive complexity, it might mean:

1. **The component's interface is poorly designed** - Consider if a better API would help
2. **You're trying to do too much** - Break your system into smaller pieces
3. **You need a different component** - Maybe the component doesn't fit your use case

But the answer is **not** to reach into the component and modify it (inheritance). That just hides the complexity and creates fragile dependencies.

## The Bottom Line

**Inheritance says:** "I'll modify this engine to work for me."
**Composition says:** "I'll build my car to work with this engine."

One creates hidden dependencies. The other creates explicit, maintainable systems.

Rust chose composition.
