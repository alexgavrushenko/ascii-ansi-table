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

### Phase 3: Content Processing ✅
- **Text Alignment** - Left, right, center, and justify alignment per column
- **Cell Padding** - Configurable left/right padding for each column
- **Text Truncation** - Cut long text with customizable ellipsis support
- **Justify Alignment** - Word spacing for full-width justification

## Usage

```rust
use ascii_table::{TableData, render_table_with_column_config, get_border_style, 
                 RenderOptions, ColumnConfig, Alignment, Padding, TruncationConfig};

// Create table data
let data = TableData::new(vec![
    vec!["Name".to_string(), "Age".to_string(), "Very Long City Name".to_string()],
    vec!["John".to_string(), "30".to_string(), "New York".to_string()],
    vec!["Jane".to_string(), "25".to_string(), "London".to_string()],
]);

// Advanced column configuration
let column_configs = vec![
    ColumnConfig::new()
        .with_width(10)
        .with_alignment(Alignment::Left)
        .with_padding(Padding::new(2, 1)),
    ColumnConfig::new()
        .with_width(5)
        .with_alignment(Alignment::Center),
    ColumnConfig::new()
        .with_width(12)
        .with_alignment(Alignment::Right)
        .with_truncation(TruncationConfig::new().with_max_width(12)),
];

let border = get_border_style("honeywell").unwrap();
let options = RenderOptions::default();
let table = render_table_with_column_config(&data, &border, &options, &column_configs).unwrap();
```

Output with advanced formatting:
```
┌──────────────┬───────┬──────────────┐
│  Name        │  Age  │  Very Lon... │
│  John        │  30   │    New York  │
│  Jane        │  25   │       London │
└──────────────┴───────┴──────────────┘
```

## Roadmap

- ✅ Phase 1: Core Foundation
- ✅ Phase 2: Border System
- ✅ Phase 3: Content Processing
- Phase 4: Advanced Text Handling
- Phase 5: Advanced Layout Features
- Phase 6: Streaming & Performance
- Phase 7: Integration & Output
- Phase 8: Developer Experience