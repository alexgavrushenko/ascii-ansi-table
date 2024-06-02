# ASCII Table

A Rust library for rendering ASCII and Unicode tables with automatic column width calculation and border support.

## Features

### Phase 1: Core Foundation ✅
- **Basic Data Structures** - Row/Cell types with table data validation
- **Simple Table Rendering** - Grid layout with fixed-width columns  
- **Column Width Calculation** - Automatic sizing based on content length
- **Basic Border Drawing** - Unicode box-drawing characters for table borders

### Phase 2: Border System ✅
- **Border Customization** - Custom border characters and styles
- **Border Templates** - Pre-defined styles (honeywell, ramac, norc, void)
- **Borderless Tables** - Clean output without borders
- **Horizontal Line Control** - Optional top/bottom borders and row separators

## Usage

```rust
use ascii_table::{TableData, render_table_with_borders, get_border_style, 
                 render_table_with_options, RenderOptions};

// Create table data
let data = TableData::new(vec![
    vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
    vec!["John".to_string(), "30".to_string(), "New York".to_string()],
    vec!["Jane".to_string(), "25".to_string(), "London".to_string()],
]);

// Default Unicode borders
let table = render_table_with_borders(&data).unwrap();

// Different border styles
let ramac_border = get_border_style("ramac").unwrap(); // ASCII
let norc_border = get_border_style("norc").unwrap();   // Double-line
let void_border = get_border_style("void").unwrap();   // No borders

// Advanced rendering options
let options = RenderOptions::with_row_separators();
let table = render_table_with_options(&data, &ramac_border, &options).unwrap();
```

Output with row separators:
```
+------+-----+----------+
| Name | Age | City     |
+------+-----+----------+
| John | 30  | New York |
+------+-----+----------+
| Jane | 25  | London   |
+------+-----+----------+
```

## Roadmap

- ✅ Phase 1: Core Foundation
- ✅ Phase 2: Border System
- Phase 3: Content Processing  
- Phase 4: Advanced Text Handling
- Phase 5: Advanced Layout Features
- Phase 6: Streaming & Performance
- Phase 7: Integration & Output
- Phase 8: Developer Experience