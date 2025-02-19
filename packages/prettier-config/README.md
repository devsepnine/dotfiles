# Basic Prettier Config

### Installation


```bash
npm install -D @c0zycr473/prettier-config
```
```bash
yarn add -D @c0zycr473/prettier-config
```
```bash
pnpm add -D @c0zycr473/prettier-config
```

### Usage

`prettier.config.mjs` or `prettier.config.js`

```js
import preset from '@c0zycr473/prettier-config';

const config = {
  ...preset,
  // your custom config
}
```

`prettier.config.mts` or `prettier.config.ts`
```ts
import preset from '@c0zycr473/prettier-config';

import type { Options } from 'prettier';

const config: Options = {
  ...preset,
  // your custom config
}
```

### [READ Prettier Docs](https://prettier.io/docs/)