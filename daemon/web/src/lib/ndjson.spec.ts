import { describe, it, expect } from 'vitest';
import { parse_ndjson } from './ndjson';

describe('parsing newline-deliminated json', () => {
    it('parses normal JSON', () => {
        const json = JSON.stringify({ foo: 100 });
        const result = parse_ndjson(json);
        expect(result).toHaveLength(1);
        expect(result[0]).toEqual({ foo: 100 });
    });

    it('parses simple newline-deliminated json', () => {
        const json_a = JSON.stringify({ a: 100 });
        const json_b = JSON.stringify({ b: 200 });
        const result = parse_ndjson(`${json_a}\n${json_b}`);
        expect(result).toHaveLength(2);
        expect(result[0]).toEqual({ a: 100 });
        expect(result[1]).toEqual({ b: 200 });
    });

    it('parses newline-deliminated json with escaped newlines within', () => {
        const json_a = JSON.stringify({ a: 'this one has\n newlines and\nstuff' });
        const json_b = JSON.stringify({ b: 200 });
        const result = parse_ndjson(`${json_a}\n${json_b}`);
        expect(result).toHaveLength(2);
        expect(result[0]).toEqual({ a: 'this one has\n newlines and\nstuff' });
        expect(result[1]).toEqual({ b: 200 });
    });

    it('actually errors out on invalid ndjson', () => {
        expect(() => parse_ndjson('invalid\njson')).toThrow();
    });
});
