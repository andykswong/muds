/**
 * Global configs for muds, which can be set via process.env for Node.js, or EnvironmentPlugin for Webpack
 * (or similar plugins for other bundlers).
 *
 * @packageDocumentation
 * @module config
 */

/**
 * True to enable debug mode.
 */
export const MUDS_DEBUG = process.env.NODE_ENV === 'development';
