# Miscellaneous

## Shallow Copy vs Deep Copy
<table>
  <thead>
    <tr>
      <th>Aspect</th>
      <th>Shallow Copy</th>
      <th>Deep Copy</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>Definition</strong></td>
      <td>Makes a new container/object but copies references for nested objects (one level).</td>
      <td>Recursively copies the object and all nested objects so the result is fully independent.</td>
    </tr>
    <tr>
      <td><strong>References</strong></td>
      <td>Top-level fields are copied; nested objects (arrays, objects) still reference the same memory.</td>
      <td>All nested structures are duplicated; no shared references remain.</td>
    </tr>
    <tr>
      <td><strong>Effect on nested mutable data</strong></td>
      <td>Mutating nested data in the copy affects the original (and vice-versa).</td>
      <td>Mutating nested data in the copy does NOT affect the original.</td>
    </tr>
    <tr>
      <td><strong>Primitives</strong></td>
      <td>Primitives (numbers, strings, booleans) are copied by value.</td>
      <td>Primitives are copied by value as well.</td>
    </tr>
    <tr>
      <td><strong>Performance / Cost</strong></td>
      <td>Faster and uses less memory (good for shallow structures).</td>
      <td>Slower and uses more memory, cost grows with object depth/size.</td>
    </tr>
    <tr>
      <td><strong>When to use</strong></td>
      <td>When you only need a new top-level container and shared nested data is acceptable or intended.</td>
      <td>When you need full independence between original and copy (safe isolation).</td>
    </tr>
    <tr>
      <td><strong>Common methods (JavaScript)</strong></td>
      <td><code>Object.assign({}, obj)</code>, spread <code>{...obj}</code>, <code>array.slice()</code>.</td>
      <td><code>structuredClone(obj)</code> (modern), or libraries like <code>lodash.cloneDeep</code>.</td>
    </tr>
    <tr>
      <td><strong>Common methods (Python)</strong></td>
      <td><code>copy.copy(obj)</code> from the <code>copy</code> module.</td>
      <td><code>copy.deepcopy(obj)</code>.</td>
    </tr>
    <tr>
      <td><strong>Limitations & pitfalls</strong></td>
      <td>Unexpected shared state bugs when nested objects are modified. Not safe for deeply nested mutable data.</td>
      <td>May fail or be expensive with circular references or objects with non-serializable state (functions, file handles) unless the method handles them.</td>
    </tr>
    <tr>
      <td><strong>Serialization trick</strong></td>
      <td>—</td>
      <td>Using JSON (e.g. <code>JSON.parse(JSON.stringify(obj))</code>) can approximate a deep copy but loses functions, <code>undefined</code>, dates, maps/sets, and custom prototypes.</td>
    </tr>
    <tr>
      <td><strong>Example (JS)</strong></td>
      <td><code>const copy = {...original};</code> (nested objects still shared)</td>
      <td><code>const copy = structuredClone(original);</code></td>
    </tr>
    <tr>
      <td><strong>Example (Python)</strong></td>
      <td><code>copy = copy.copy(original)</code></td>
      <td><code>copy = copy.deepcopy(original)</code></td>
    </tr>
  </tbody>
</table>


## References vs Pointers (C++ context)
<table>
  <thead>
    <tr>
      <th>Feature</th>
      <th><b>Reference</b></th>
      <th><b>Pointer</b></th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><b>Definition</b></td>
      <td>An alias (another name) for an existing variable.</td>
      <td>A variable that stores the memory address of another variable.</td>
    </tr>
    <tr>
      <td><b>Declaration</b></td>
      <td><code>int &amp;ref = var;</code></td>
      <td><code>int *ptr = &amp;var;</code></td>
    </tr>
    <tr>
      <td><b>Nullability</b></td>
      <td>Cannot be <code>null</code> (must refer to a valid object when created).</td>
      <td>Can be <code>null</code> (e.g., <code>int* ptr = nullptr;</code>).</td>
    </tr>
    <tr>
      <td><b>Reassignment</b></td>
      <td>Once initialized, a reference cannot be changed to refer to another object.</td>
      <td>A pointer can be reassigned to point to different objects.</td>
    </tr>
    <tr>
      <td><b>Dereferencing</b></td>
      <td>Implicit – using the reference automatically accesses the referred object.</td>
      <td>Explicit – must use <code>*</code> to access the value at the address.</td>
    </tr>
    <tr>
      <td><b>Memory usage</b></td>
      <td>Typically implemented under the hood as a pointer, but has no extra syntax overhead.</td>
      <td>Uses memory to store the address itself.</td>
    </tr>
    <tr>
      <td><b>Arithmetic</b></td>
      <td>No pointer arithmetic (cannot do <code>ref++</code> to move to the next memory location).</td>
      <td>Supports pointer arithmetic (e.g., <code>ptr++</code> moves to next element).</td>
    </tr>
    <tr>
      <td><b>Safety</b></td>
      <td>Safer and simpler (cannot be <code>null</code>, cannot change what it refers to).</td>
      <td>More flexible but riskier (dangling pointers, null dereference, memory leaks).</td>
    </tr>
    <tr>
      <td><b>Use cases</b></td>
      <td>Function arguments (pass-by-reference), operator overloading, improving readability.</td>
      <td>Dynamic memory allocation (<code>new</code>, <code>delete</code>), working with arrays, low-level system programming.</td>
    </tr>
  </tbody>
</table>


## Questions

<details>
<summary>1. Why does Rust only use stack and heap, not things like linked lists?</summary>
Because stack and heap are memory regions managed by the CPU/OS. Data structures (like linked lists, trees, graphs) are built on top of these regions and typically use heap allocation, with handles on the stack.
</details>

<details>
<summary>2. Can all data structures be categorized into stack vs heap allocation?</summary>
Yes. Values with fixed, compile-time size live on the stack; dynamically sized or growable contents live on the heap. Most structures are a mix: a handle/control on the stack pointing to data on the heap.
</details>

<details>
<summary>3. Where do trees and graphs live (stack or heap)?</summary>
Root pointer/handle on the stack; nodes, edges, and adjacency lists on the heap. Practically: mostly heap, accessed via a stack variable.
</details>

## Out of the scope concepts

> These concepts are out of the rust scope. But this section contains those terms or concepts that may directly (but not important) or indirectly link to the concepts in actual chapters.

- Look at CPU vs GPU vs NPU vs TPU
