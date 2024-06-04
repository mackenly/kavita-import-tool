import { describe, it, expect } from "vitest";

describe('next.config', () => {
    it('should export withNextra', () => {
        const result = require('./next.config.js');
        expect(result).toBeInstanceOf(Object);
        expect(result).toHaveProperty('pageExtensions');
        expect(result).toHaveProperty('webpack');
        expect(result).toHaveProperty('rewrites');
    });
});