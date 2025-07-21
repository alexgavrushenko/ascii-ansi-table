
/**
 * Creates a simple test table with basic data
 * @param {number} rows - Number of data rows to create
 * @param {number} cols - Number of columns to create
 * @returns {Array<Array<string>>} Table data
 */
export function createSimpleTable(rows = 3, cols = 3) {
    const data = [];
    
    const headers = [];
    for (let c = 0; c < cols; c++) {
        headers.push(`Header ${c + 1}`);
    }
    data.push(headers);
    
    for (let r = 0; r < rows; r++) {
        const row = [];
        for (let c = 0; c < cols; c++) {
            row.push(`Row ${r + 1} Col ${c + 1} `);
        }
        data.push(row);
    }
    
    return data;
}

/**
 * Creates a table with ANSI color codes
 * @returns {Array<Array<string>>} Colorized table data
 */
export function createColorfulTable() {
    return [
        [
            '\x1b[1m\x1b[34mBlue Header\x1b[0m',
            '\x1b[1m\x1b[32mGreen Header\x1b[0m',
            '\x1b[1m\x1b[31mRed Header\x1b[0m'
        ],
        [
            '\x1b[36mCyan text\x1b[0m',
            '\x1b[33mYellow text\x1b[0m',
            '\x1b[35mMagenta text\x1b[0m'
        ],
        [
            '\x1b[42m\x1b[30mGreen BG\x1b[0m',
            '\x1b[41m\x1b[37mRed BG\x1b[0m',
            '\x1b[44m\x1b[37mBlue BG\x1b[0m'
        ]
    ];
}

/**
 * Creates a large table for performance testing
 * @param {number} rows - Number of rows to create
 * @param {number} cols - Number of columns to create
 * @returns {Array<Array<string>>} Large table data
 */
export function createLargeTable(rows = 100, cols = 10) {
    const data = [];
    
    const headers = [];
    for (let c = 0; c < cols; c++) {
        headers.push(`Column ${c + 1}`);
    }
    data.push(headers);
    
    for (let r = 0; r < rows; r++) {
        const row = [];
        for (let c = 0; c < cols; c++) {
            row.push(`Data ${r + 1}-${c + 1}`);
        }
        data.push(row);
    }
    
    return data;
}

/**
 * Creates a table with varying content lengths
 * @returns {Array<Array<string>>} Table with mixed content lengths
 */
export function createMixedLengthTable() {
    return [
        ['Short', 'Medium length content', 'This is a very long piece of content that should test wrapping'],
        ['X', 'Testing', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit'],
        ['A', 'B', 'C'],
        ['Very long header text here', 'M', 'Short']
    ];
}

/**
 * Creates a table with Unicode characters
 * @returns {Array<Array<string>>} Table with Unicode content
 */
export function createUnicodeTable() {
    return [
        ['English', 'Chinese', 'Japanese', 'Emoji'],
        ['Hello', '你好', 'こんにちは', '👋'],
        ['World', '世界', '世界', '🌍'],
        ['Test', '测试', 'テスト', '🧪']
    ];
}

/**
 * Creates a table with complex ANSI sequences
 * @returns {Array<Array<string>>} Table with complex ANSI formatting
 */
export function createComplexAnsiTable() {
    return [
        [
            '\x1b[1m\x1b[4m\x1b[34mBold Blue Underlined\x1b[0m',
            '\x1b[38;5;196m\x1b[48;5;46mBright Red on Bright Green\x1b[0m',
            '\x1b[3m\x1b[33mItalic Yellow\x1b[0m'
        ],
        [
            '\x1b[9m\x1b[35mStrikethrough Magenta\x1b[0m',
            '\x1b[1m\x1b[31m\x1b[4m\x1b[5mBold Red Underlined Blinking\x1b[0m',
            '\x1b[2m\x1b[37mDim White\x1b[0m'
        ],
        [
            '\x1b[7m\x1b[36mReverse Cyan\x1b[0m',
            '\x1b[38;2;255;128;0mTrue Color Orange\x1b[0m',
            '\x1b[1m\x1b[4m\x1b[31m\x1b[42mComplex Mix\x1b[0m'
        ]
    ];
}

/**
 * Measures execution time of a function
 * @param {Function} fn - Function to measure
 * @returns {Promise<{result: any, time: number}>} Result and execution time in ms
 */
export async function measureTime(fn) {
    const start = performance.now();
    const result = await fn();
    const end = performance.now();
    return {
        result,
        time: end - start
    };
}

/**
 * Validates that a string contains ANSI escape sequences
 * @param {string} text - Text to check
 * @returns {boolean} True if ANSI sequences are found
 */
export function containsAnsi(text) {
    return /\x1b\[[0-9;]*m/.test(text);
}

/**
 * Strips ANSI escape sequences from text (fallback implementation)
 * @param {string} text - Text to strip
 * @returns {string} Text without ANSI codes
 */
export function stripAnsi(text) {
    return text.replace(/\x1b\[[0-9;]*m/g, '');
}

/**
 * Calculates the visual width of text (without ANSI codes)
 * @param {string} text - Text to measure
 * @returns {number} Visual width
 */
export function getVisualWidth(text) {
    const stripped = stripAnsi(text);
    return stripped.length;
}

/**
 * Creates a basic table configuration
 * @param {Object} options - Configuration options
 * @returns {Object} Table configuration
 */
export function createBasicConfig(options = {}) {
    return {
        border: options.border || null,
        columns: options.columns || null,
        column_default: options.column_default || null,
        single_line: options.single_line || false,
        spanning_cells: options.spanning_cells || null,
        header: options.header || null
    };
}

/**
 * Creates a column configuration
 * @param {Object} options - Column options
 * @returns {Object} Column configuration
 */
export function createColumnConfig(options = {}) {
    return {
        alignment: options.alignment || null,
        vertical_alignment: options.vertical_alignment || null,
        padding_left: options.padding_left || null,
        padding_right: options.padding_right || null,
        truncate: options.truncate || null,
        wrap_word: options.wrap_word || null,
        width: options.width || null
    };
}

/**
 * Creates border configuration
 * @param {string} style - Border style name
 * @returns {Object} Border configuration (placeholder)
 */
export function createBorderConfig(style = 'honeywell') {
    return { style };
}

/**
 * Test runner utility
 * @param {string} name - Test name
 * @param {Function} testFn - Test function
 * @returns {Object} Test definition
 */
export function createTest(name, testFn) {
    return { name, testFn };
}

/**
 * Debug utility for detailed error reporting
 */
export const debug = {
    /**
     * Creates detailed comparison information
     * @param {any} actual - Actual value
     * @param {any} expected - Expected value
     * @param {string} context - Context description
     * @returns {string} Detailed debug info
     */
    formatComparison(actual, expected, context = '') {
        const actualType = typeof actual;
        const expectedType = typeof expected;
        
        let debug = `\n=== DEBUG INFO ${context ? `(${context})` : ''} ===\n`;
        debug += `Expected: ${JSON.stringify(expected)} (${expectedType})\n`;
        debug += `Actual:   ${JSON.stringify(actual)} (${actualType})\n`;
        
        if (actualType === 'string' && expectedType === 'string') {
            debug += `Expected length: ${expected.length}\n`;
            debug += `Actual length:   ${actual.length}\n`;
            if (actual.length !== expected.length) {
                debug += `Length difference: ${actual.length - expected.length}\n`;
            }
        }
        
        if (actualType === 'number' && expectedType === 'number') {
            debug += `Difference: ${actual - expected}\n`;
        }
        
        debug += `===============================`;
        return debug;
    },

    /**
     * Shows table output details for debugging
     * @param {string} output - Table output
     * @param {string} context - Context description
     * @returns {string} Debug info
     */
    formatTableOutput(output, context = '') {
        const lines = output.split('\n');
        const lineLengths = lines.map(line => stripAnsi(line).length);
        const maxLength = Math.max(...lineLengths);
        
        let debug = `\n=== TABLE DEBUG ${context ? `(${context})` : ''} ===\n`;
        debug += `Total lines: ${lines.length}\n`;
        debug += `Max line length: ${maxLength}\n`;
        debug += `Line lengths: [${lineLengths.slice(0, 5).join(', ')}${lineLengths.length > 5 ? ', ...' : ''}]\n`;
        debug += `First few lines:\n`;
        
        for (let i = 0; i < Math.min(3, lines.length); i++) {
            const stripped = stripAnsi(lines[i]);
            debug += `  ${i}: "${stripped}" (len: ${stripped.length})\n`;
        }
        
        if (lines.length > 3) {
            debug += `  ... (${lines.length - 3} more lines)\n`;
        }
        
        debug += `============================`;
        return debug;
    }
};

/**
 * Assertion utilities for tests with enhanced debugging
 */
export const assert = {
    /**
     * Assert that a condition is true
     * @param {boolean} condition - Condition to check
     * @param {string} message - Error message
     * @param {any} actual - Actual value for debugging
     * @param {any} expected - Expected value for debugging
     */
    isTrue(condition, message = 'Assertion failed', actual = null, expected = true) {
        if (!condition) {
            let errorMsg = message;
            if (actual !== null) {
                errorMsg += debug.formatComparison(actual, expected, 'isTrue assertion');
            }
            throw new Error(errorMsg);
        }
    },

    /**
     * Assert that two values are equal
     * @param {any} actual - Actual value
     * @param {any} expected - Expected value
     * @param {string} message - Error message
     */
    equals(actual, expected, message = null) {
        if (actual !== expected) {
            const defaultMessage = `Values are not equal`;
            const finalMessage = message || defaultMessage;
            const errorMsg = finalMessage + debug.formatComparison(actual, expected, 'equals assertion');
            throw new Error(errorMsg);
        }
    },

    /**
     * Assert that a string contains a substring
     * @param {string} text - Text to search in
     * @param {string} substring - Substring to find
     * @param {string} message - Error message
     */
    contains(text, substring, message = null) {
        if (!text.includes(substring)) {
            const defaultMessage = `Text does not contain "${substring}"`;
            const finalMessage = message || defaultMessage;
            let errorMsg = finalMessage;
            errorMsg += `\n=== DEBUG INFO (contains assertion) ===\n`;
            errorMsg += `Searching for: "${substring}"\n`;
            errorMsg += `In text (first 200 chars): "${text.slice(0, 200)}${text.length > 200 ? '...' : ''}"\n`;
            errorMsg += `Text length: ${text.length}\n`;
            errorMsg += `=====================================`;
            throw new Error(errorMsg);
        }
    },

    /**
     * Assert that a value is not null or undefined
     * @param {any} value - Value to check
     * @param {string} message - Error message
     */
    notNull(value, message = 'Value is null or undefined') {
        if (value == null) {
            const errorMsg = message + debug.formatComparison(value, 'not null/undefined', 'notNull assertion');
            throw new Error(errorMsg);
        }
    },

    /**
     * Assert that a number is within a range
     * @param {number} value - Value to check
     * @param {number} min - Minimum value
     * @param {number} max - Maximum value
     * @param {string} message - Error message
     */
    inRange(value, min, max, message = null) {
        if (value < min || value > max) {
            const defaultMessage = `Value not in range`;
            const finalMessage = message || defaultMessage;
            let errorMsg = finalMessage;
            errorMsg += `\n=== DEBUG INFO (inRange assertion) ===\n`;
            errorMsg += `Value: ${value}\n`;
            errorMsg += `Range: [${min}, ${max}]\n`;
            errorMsg += `Too low: ${value < min}\n`;
            errorMsg += `Too high: ${value > max}\n`;
            errorMsg += `Distance from min: ${value - min}\n`;
            errorMsg += `Distance from max: ${value - max}\n`;
            errorMsg += `=====================================`;
            throw new Error(errorMsg);
        }
    },

    /**
     * Assert that a number is less than or equal to a maximum
     * @param {number} value - Value to check
     * @param {number} max - Maximum allowed value
     * @param {string} message - Error message
     * @param {string} context - Additional context for debugging
     */
    lessThanOrEqual(value, max, message = null, context = '') {
        if (value > max) {
            const defaultMessage = `Value ${value} should be <= ${max}`;
            const finalMessage = message || defaultMessage;
            let errorMsg = finalMessage;
            errorMsg += `\n=== DEBUG INFO (lessThanOrEqual assertion) ===\n`;
            errorMsg += `Value: ${value}\n`;
            errorMsg += `Maximum allowed: ${max}\n`;
            errorMsg += `Excess: ${value - max}\n`;
            if (context) {
                errorMsg += `Context: ${context}\n`;
            }
            errorMsg += `============================================`;
            throw new Error(errorMsg);
        }
    }
};

/**
 * Performance benchmarking utility
 */
export class Benchmark {
    constructor(name) {
        this.name = name;
        this.runs = [];
    }

    async run(fn, iterations = 100) {
        console.log(`Running benchmark: ${this.name} (${iterations} iterations)`);
        
        for (let i = 0; i < iterations; i++) {
            const start = performance.now();
            await fn();
            const end = performance.now();
            this.runs.push(end - start);
        }

        return this.getStats();
    }

    getStats() {
        if (this.runs.length === 0) return null;

        const sorted = [...this.runs].sort((a, b) => a - b);
        const sum = this.runs.reduce((a, b) => a + b, 0);

        return {
            name: this.name,
            iterations: this.runs.length,
            min: Math.min(...this.runs),
            max: Math.max(...this.runs),
            mean: sum / this.runs.length,
            median: sorted[Math.floor(sorted.length / 2)],
            p95: sorted[Math.floor(sorted.length * 0.95)],
            p99: sorted[Math.floor(sorted.length * 0.99)]
        };
    }

    clear() {
        this.runs = [];
    }
}

/**
 * WASM module loader utility
 * @param {string} wasmPath - Path to WASM module
 * @returns {Promise<Object>} Loaded WASM module
 */
export async function loadWasmModule(wasmPath = '../pkg/ascii_ansi_table.js') {
    try {
        const module = await import(wasmPath);
        await module.default();
        return module;
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        throw error;
    }
}

export default {
    createSimpleTable,
    createColorfulTable,
    createLargeTable,
    createMixedLengthTable,
    createUnicodeTable,
    createComplexAnsiTable,
    measureTime,
    containsAnsi,
    stripAnsi,
    getVisualWidth,
    createBasicConfig,
    createColumnConfig,
    createBorderConfig,
    createTest,
    debug,
    assert,
    Benchmark,
    loadWasmModule
};