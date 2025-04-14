export type NewlineDeliminatedJson = any[];

export function parse_ndjson(input: string): NewlineDeliminatedJson {
    const lines = input.split('\n');
    const result = [];
    let current_line = '';
    while (lines.length > 0) {
        current_line += lines.shift();
        if (current_line.length === 0) {
            continue;
        }
        try {
            const entry = JSON.parse(current_line);
            result.push(entry);
            current_line = '';
        } catch (e) {
            // if this chunk wasn't valid JSON, assume there was an escaped
            // newline in the JSON line, so simply continue to the next one.
            // however, if we've reached the end of the input, that means we
            // were given invalid nd-json
            if (lines.length === 0) {
                throw new Error(`unable to parse invalid nd-json: ${e}, "${current_line}"`);
            }
        }
    }
    return result;
}
