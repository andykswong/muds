import { TextEncoder, TextDecoder } from 'util';
import { TestEnvironment as JSDOMTestEnvironment } from 'jest-environment-jsdom';

/** JSDOM test environment with polyfills. */
export class TestEnvironment extends JSDOMTestEnvironment {
  async setup() {
    await super.setup();
    if (typeof this.global.TextEncoder === 'undefined') {
      this.global.TextEncoder = TextEncoder;
      this.global.TextDecoder = TextDecoder;
    }
  }
}

export default TestEnvironment;
