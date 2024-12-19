# Rust Data Race Example

This repository demonstrates a common error in Rust: data races that can occur when modifying a vector while holding references to its elements.  The `bug.rs` file contains the problematic code, while `bugSolution.rs` offers a corrected version.

**Understanding the Issue:**

In Rust, borrowing rules are crucial for memory safety.  When a mutable reference is held to a part of a data structure, no other mutable or immutable references can exist concurrently.  This program violates this rule.

**Solution:**

The `bugSolution.rs` demonstrates a way to correctly handle this, ensuring no data races occur. The most common approach is to avoid holding onto the reference while modifying the vector.