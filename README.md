# Algebraic data type domain encoding
A reference implementation.

### Encode problem domain using product and sum types, with annotations for auto-serialization.

![alt text](img/encode.png)

### Construct types that represent valid states, making invalid states unrepresentable.

![alt text](img/construction.png)

### Save or load from/to config files. As domain has been encoded using algebraic data types, load/save will preserve integrity of state or fail to produce invalid state.

![alt text](img/config.png)

### In source code, all queries and filters would be strongly-typed and numerous branching logic would fail if new cases are not handled, or types have been refactored.

![alt text](img/query.png)