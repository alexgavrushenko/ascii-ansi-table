# ASCII Table

A Rust library for rendering ASCII and Unicode tables with automatic column width calculation and border support.

## Features

### Phase 1: Core Foundation ✅
- **Basic Data Structures** - Row/Cell types with table data validation
- **Simple Table Rendering** - Grid layout with fixed-width columns  
- **Column Width Calculation** - Automatic sizing based on content length
- **Basic Border Drawing** - Unicode box-drawing characters for table borders

## Usage

```rust
use ascii_table::{TableData, render_table_with_borders};

// Create table data
let data = TableData::new(vec![
    vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
    vec!["John".to_string(), "30".to_string(), "New York".to_string()],
    vec!["Jane".to_string(), "25".to_string(), "London".to_string()],
]);

// Render with borders
let table = render_table_with_borders(&data).unwrap();
println!("{}", table);
```

Output:
```
┌──────┬─────┬──────────┐
│ Name │ Age │ City     │
│ John │ 30  │ New York │
│ Jane │ 25  │ London   │
└──────┴─────┴──────────┘
```

## Roadmap

- Phase 2: Border System
- Phase 3: Content Processing  
- Phase 4: Advanced Text Handling
- Phase 5: Advanced Layout Features
- Phase 6: Streaming & Performance
- Phase 7: Integration & Output
- Phase 8: Developer Experience