# Signal Frames

A signal Frame gets translated to an execution context and associated query
for execution.

- Based on datafusion primitives to leverage datafusion optimizers
- may require custom logical and physical nodes

## Provider Mapping

| Provider              | Target Trait   | Implementation |
| --------------------- | -------------- | -------------- |
| `TableReference`      | `TableSource`  | various        |
| `ModelReference`      | `UDF`          | `FlightUdf`    |
| `ExpressionReference` | `PhysicalExpr` | datafusion     |
