
export function createPerformanceTestSuite(WasmTable, measureTime) {
    const tests = [
        {
            name: "Large Table Performance - 1000 rows with repeated newlines",
            testFn: async () => {
                return await testLargeTablePerformance(WasmTable, measureTime);
            }
        },
        {
            name: "ANSI Processing Performance - Complex table",
            testFn: async () => {
                return await testAnsiProcessingPerformance(WasmTable, measureTime);
            }
        }
    ];

    return { tests };
}

async function testLargeTablePerformance(WasmTable, measureTime) {
    const rows = 1000;
    const cols = 2;
    const cellSize = 75;

    console.log(`Creating large table: ${rows} rows × ${cols} cols, ${cellSize} newlines per cell`);

    const data = [];
    
    const header = [];
    for (let i = 0; i < cols; i++) {
        header.push(`Column ${i + 1}`);
    }
    data.push(header);

    
    const cellContent = "?\n".repeat(cellSize);


    
    for (let rowIdx = 0; rowIdx < rows; rowIdx++) {
        const rowData = [];
        for (let colIdx = 0; colIdx < cols; colIdx++) {
            rowData.push(cellContent);
        }
        data.push(rowData);
    }

    console.log(`Generated data: ${data.length} rows (including header)`);
    console.log(`Sample cell content length: ${cellContent.length} chars`);
    console.log(`Sample cell newline count: ${(cellContent.match(/\n/g) || []).length}`);

    
    const { result: table, time: creationTime } = await measureTime(() => {
        const wasmTable = new WasmTable();
        wasmTable.setData(data);
        return wasmTable;
    });

    console.log(`Table creation time: ${creationTime.toFixed(2)}ms`);

    
    const { result: tableString, time: renderTime } = await measureTime(() => {
        return table.render();
    });

    console.log(`Table render time: ${renderTime.toFixed(2)}ms`);

    
    const height = tableString.split('\n').length;
    const charCount = tableString.length;

    console.log(`Table height: ${height} lines`);
    console.log(`Table string length: ${charCount} characters`);

    
    if (creationTime > 500) {
        throw new Error(`Table creation took ${creationTime.toFixed(2)}ms, expected < 500ms`);
    }

    if (height === 0) {
        throw new Error('Table height should be greater than 0');
    }

    if (charCount === 0) {
        throw new Error('Table string should not be empty');
    }

    return {
        description: `Large table performance test: ${rows}×${cols} table with ${cellSize} newlines per cell`,
        creationTime: creationTime,
        renderTime: renderTime,
        height: height,
        charCount: charCount,
        output: tableString.slice(0, 500) + "...",
        maxLineLength: Math.max(...tableString.split('\n').map(line => line.length)),
        lineCount: height,
        performance: {
            rowsPerSecond: Math.round(rows / (renderTime / 1000)),
            charsPerSecond: Math.round(charCount / (renderTime / 1000))
        }
    };
}

async function testAnsiProcessingPerformance(WasmTable, measureTime) {
    const rows = 100;
    const cols = 4;

    const data = [];
    
    data.push([
        '\x1b[1m\x1b[34mID\x1b[0m',
        '\x1b[1m\x1b[32mName\x1b[0m',
        '\x1b[1m\x1b[33mStatus\x1b[0m',
        '\x1b[1m\x1b[35mDetails\x1b[0m'
    ]);

    
    for (let i = 0; i < rows; i++) {
        const statusColors = [
            '\x1b[32mOnline\x1b[0m',
            '\x1b[31mOffline\x1b[0m', 
            '\x1b[33mPending\x1b[0m',
            '\x1b[36mProcessing\x1b[0m'
        ];
        
        const details = `\x1b[1mEntry ${i}\x1b[0m\n\x1b[3mWith multiple\x1b[0m\n\x1b[4mlines and\x1b[0m\n\x1b[9mformatting\x1b[0m`;
        
        data.push([
            `\x1b[90m${i.toString().padStart(3, '0')}\x1b[0m`,
            `\x1b[1mUser_${i}\x1b[0m`,
            statusColors[i % statusColors.length],
            details
        ]);
    }

    console.log(`Creating ANSI performance test: ${rows} rows × ${cols} cols`);

    
    const { result: tableString, time: renderTime } = await measureTime(() => {
        const table = new WasmTable();
        table.setData(data);
        return table.render();
    });

    console.log(`ANSI table render time: ${renderTime.toFixed(2)}ms`);

    
    const hasAnsi = tableString.includes('\x1b[');
    const ansiSequenceCount = (tableString.match(/\x1b\[[0-9;]*m/g) || []).length;

    return {
        description: `ANSI processing performance: ${rows}×${cols} table with complex formatting`,
        renderTime: renderTime,
        hasAnsi: hasAnsi,
        ansiSequenceCount: ansiSequenceCount,
        height: tableString.split('\n').length,
        charCount: tableString.length,
        output: tableString.slice(0, 500) + "...",
        performance: {
            ansiSequencesPerSecond: Math.round(ansiSequenceCount / (renderTime / 1000))
        }
    };
}

export { testLargeTablePerformance, testAnsiProcessingPerformance };