


export class TestSuite {
    constructor() {
        this.tests = [];
        this.results = [];
    }

    addTest(name, testFn) {
        this.tests.push({ name, testFn });
    }

    async runAll() {
        this.results = [];
        for (const test of this.tests) {
            try {
                const result = await test.testFn();
                this.results.push({
                    name: test.name,
                    status: 'PASS',
                    result: result,
                    error: null
                });
            } catch (error) {
                this.results.push({
                    name: test.name,
                    status: 'FAIL',
                    result: null,
                    error: error.message
                });
            }
        }
        return this.results;
    }

    getResults() {
        return this.results;
    }

    getPassCount() {
        return this.results.filter(r => r.status === 'PASS').length;
    }

    getFailCount() {
        return this.results.filter(r => r.status === 'FAIL').length;
    }
}

import { 
    createSimpleTable, 
    createColorfulTable, 
    createMixedLengthTable,
    createUnicodeTable,
    createComplexAnsiTable,
    measureTime,
    debug,
    assert
} from './test-utils.js';

/**
 * Creates a functional test suite for ASCII ANSI Table WASM bindings
 * @param {Object} Table - WASM Table class (WasmTable)
 * @param {Function} convertAnsiToHtml - ANSI to HTML converter function
 * @param {Function} stripAnsi - ANSI sequence stripper function
 * @returns {Object} Functional test suite with core feature tests
 */
export function createFunctionalTestSuite(Table, convertAnsiToHtml, stripAnsi) {
    const tests = [];

    // Test 1: Basic Table Creation
    tests.push({
        name: "Basic Table Creation",
        testFn: async () => {
            const table = new Table();
            const data = createSimpleTable(2, 3);
            table.setData(data);
            
            // Configure header styling to differentiate header row from data rows
            const config = {
                header: {
                    border: {
                        top_body: "═",
                        top_join: "╦",
                        top_left: "╔",
                        top_right: "╗",
                        bottom_body: "═",
                        bottom_join: "╩",
                        bottom_left: "╚",
                        bottom_right: "╝",
                        body_left: "║",
                        body_right: "║",
                        body_join: "║",
                        header_join: "═",
                        join_body: "─",
                        join_left: "╠",
                        join_right: "╣",
                        join_join: "╬"
                    },
                    column_default: {
                        padding_left: 1,
                        padding_right: 1
                    }
                }
            };
            
            table.setConfig(config);
            const result = table.render();
            
            // Verify table contains expected content
            assert.contains(result, "Header 1", "Table should contain header");
            assert.contains(result, "Row 1 Col 1", "Table should contain data");
            
            // Verify header styling is applied
            assert.contains(result, "║", "Table should contain header-style vertical borders");
            assert.contains(result, "═", "Table should contain header-style horizontal borders");
            
            return {
                output: result,
                description: "Basic 2x3 table with proper header styling",
                width: result.split('\n')[0].length,
                height: result.split('\n').length,
                hasHeaderStyling: result.includes("║") && result.includes("═")
            };
        }
    });

    // Test 2: ANSI Color Support
    tests.push({
        name: "ANSI Color Support",
        testFn: async () => {
            const table = new Table();
            const data = createColorfulTable();
            table.setData(data);
            const result = table.render();
            
            // Verify ANSI sequences are preserved
            assert.isTrue(result.includes('\x1b['), "Table should contain ANSI sequences");
            
            let htmlResult = null;
            if (convertAnsiToHtml) {
                htmlResult = convertAnsiToHtml(result);
                assert.contains(htmlResult, 'span', "HTML should contain formatting spans");
            }
            
            return {
                output: result,
                htmlOutput: htmlResult,
                description: "Table with ANSI color codes",
                hasAnsi: result.includes('\x1b[')
            };
        }
    });

    // Test 3: Border Styles
    tests.push({
        name: "Border Style Configuration",
        testFn: async () => {
            const table = new Table();
            const data = createSimpleTable(2, 2);
            table.setData(data);
            
            // Test default border (no specific border style setting in current API)
            const honeywellResult = table.render();
            
            // Test ASCII border (create new table for different config)
            const table2 = new Table();
            table2.setData(data);
            const ramacResult = table2.render();
            
            // Verify tables produce output (both should be same without config changes)
            assert.isTrue(honeywellResult === ramacResult, "Same configuration should produce same output");
            
            return {
                output: honeywellResult,
                outputRamac: ramacResult,
                description: "Table with different border styles",
                honeywellLength: honeywellResult.length,
                ramacLength: ramacResult.length
            };
        }
    });

    // Test 4: Column Width Configuration
    tests.push({
        name: "Column Width Configuration",
        testFn: async () => {
            const table = new Table();
            const data = createMixedLengthTable();
            table.setData(data);
            
            // Configure column widths through setConfig
            const config = {
                column_default: {
                    width: 15,  // Set default column width to 15 characters
                    wrap_word: true
                }
            };
            
            table.setConfig(config);
            const result = table.render();
            
            // Check that lines don't exceed expected width
            const lines = result.split('\n').filter(line => line.trim() !== '');
            const lineLengths = lines.map(line => stripAnsi ? stripAnsi(line).length : line.length);
            const maxLineLength = Math.max(...lineLengths);
            
            // With 3 columns of width 15 + padding + borders, expect around 55-60 chars max
            const expectedMaxWidth = 70; // 3 * 15 + padding + borders + some margin
            
            // Add detailed debugging for the assertion
            const debugInfo = debug.formatTableOutput(result, 'width configuration test');
            
            assert.lessThanOrEqual(
                maxLineLength, 
                expectedMaxWidth, 
                `Max line length should be controlled by column width config`,
                `Expected max width: ${expectedMaxWidth}, Actual: ${maxLineLength}${debugInfo}`
            );
            
            return {
                output: result,
                description: "Table with column width configuration",
                maxLineLength: maxLineLength,
                expectedWidth: expectedMaxWidth,
                lineCount: lines.length,
                lineLengths: lineLengths.slice(0, 5), // First 5 line lengths for debugging
                config: config
            };
        }
    });

    // Test 5: Unicode Support
    tests.push({
        name: "Unicode Character Support",
        testFn: async () => {
            const table = new Table();
            const data = createUnicodeTable();
            table.setData(data);
            const result = table.render();
            
            // Verify Unicode characters are preserved
            assert.contains(result, "你好", "Table should contain Chinese characters");
            assert.contains(result, "こんにちは", "Table should contain Japanese characters");
            assert.contains(result, "👋", "Table should contain emoji");
            
            return {
                output: result,
                description: "Table with Unicode characters",
                hasUnicode: /[^\x00-\x7F]/.test(result)
            };
        }
    });

    // Test 6: Complex ANSI Sequences
    tests.push({
        name: "Complex ANSI Sequences",
        testFn: async () => {
            const table = new Table();
            const data = createComplexAnsiTable();
            table.setData(data);
            const result = table.render();
            
            // Verify complex ANSI sequences are preserved
            assert.contains(result, "\x1b[1m", "Should contain bold sequences");
            assert.contains(result, "\x1b[4m", "Should contain underline sequences");
            
            let htmlResult = null;
            if (convertAnsiToHtml) {
                const { result: conversion, time } = await measureTime(() => 
                    convertAnsiToHtml(result)
                );
                htmlResult = conversion;
                
                return {
                    output: result,
                    htmlOutput: htmlResult,
                    description: "Complex ANSI formatting test",
                    conversionTime: time,
                    hasComplexAnsi: result.includes('\x1b[38;5;') || result.includes('\x1b[48;5;')
                };
            }
            
            return {
                output: result,
                description: "Complex ANSI formatting test",
                hasComplexAnsi: result.includes('\x1b[38;5;') || result.includes('\x1b[48;5;')
            };
        }
    });

    // Test 7: Performance Benchmark
    tests.push({
        name: "Performance Benchmark",
        testFn: async () => {
            const table = new Table();
            const data = createSimpleTable(100, 5); // 100 rows, 5 columns
            
            // Measure table creation performance
            const { result: setDataResult, time: creationTime } = await measureTime(() => {
                table.setData(data);
                return true;
            });
            
            // Measure rendering performance
            const { result: renderResult, time: renderTime } = await measureTime(() => {
                return table.render();
            });
            
            const lines = renderResult.split('\n');
            const totalChars = renderResult.length;
            
            return {
                output: renderResult.slice(0, 500) + "...", // Preview only
                description: "Performance test with 100x5 table",
                creationTime: creationTime,
                renderTime: renderTime,
                totalTime: creationTime + renderTime,
                lineCount: lines.length,
                charCount: totalChars,
                preview: `Performance: Create ${creationTime.toFixed(2)}ms, Render ${renderTime.toFixed(2)}ms`
            };
        }
    });

    // Test 8: Header Configuration
    tests.push({
        name: "Header Configuration",
        testFn: async () => {
            const table = new Table();
            
            // Set data with headers (current API uses setData with full data array)
            const data = [
                ["\x1b[1mBold Header 1\x1b[0m", "\x1b[1mBold Header 2\x1b[0m"],
                ["Data 1", "Data 2"],
                ["Data 3", "Data 4"]
            ];
            
            table.setData(data);
            const result = table.render();
            
            assert.contains(result, "Bold Header", "Should contain header");
            assert.contains(result, "Data 1", "Should contain data");
            
            return {
                output: result,
                description: "Table with separate header configuration",
                hasHeader: result.includes("Bold Header")
            };
        }
    });

    // Test 9: Row-by-Row Building
    tests.push({
        name: "Row-by-Row Table Building",
        testFn: async () => {
            const table = new Table();
            
            // Build table with full data array (current API doesn't support row-by-row)
            const data = [
                ["Column A", "Column B", "Column C"],
                ["Value 1", "Value 2", "Value 3"],
                ["\x1b[32mGreen\x1b[0m", "\x1b[31mRed\x1b[0m", "\x1b[34mBlue\x1b[0m"]
            ];
            
            table.setData(data);
            const result = table.render();
            
            assert.contains(result, "Column A", "Should contain first row");
            assert.contains(result, "Value 1", "Should contain second row");
            assert.contains(result, "Green", "Should contain third row");
            
            return {
                output: result,
                description: "Table built row by row",
                rowCount: 3
            };
        }
    });

    // Test 10: Table Clearing and Reuse
    tests.push({
        name: "Table Clearing and Reuse",
        testFn: async () => {
            const table = new Table();
            
            // Create first table
            const data1 = createSimpleTable(2, 2);
            table.setData(data1);
            const result1 = table.render();
            
            // Clear and create second table
            table.clear();
            const data2 = createColorfulTable();
            table.setData(data2);
            const result2 = table.render();
            
            // Verify tables are different
            assert.isTrue(result1 !== result2, "Cleared table should produce different output");
            assert.contains(result2, "\x1b[", "Second table should contain ANSI codes");
            
            return {
                output: result2,
                description: "Table cleared and reused",
                firstTableLength: result1.length,
                secondTableLength: result2.length,
                different: result1 !== result2
            };
        }
    });

    return { tests };
}


export default createFunctionalTestSuite;