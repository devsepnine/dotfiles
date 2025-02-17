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

`.prettierrc.mjs` or `.prettierrc.js` or `prettier.config.js`

```js
import basicConfig from '@c0zycr473/prettier-config';

const config = {
  ...basicConfig,
  // your custom config
}
```

### Default Options
```js
{
  printWidth: 100,
  tabWidth: 2,
  useTabs: false,
  semi: true,
  singleQuote: true,
  trailingComma: "es5",
  bracketSpacing: true,
  arrowParens: "avoid",
  endOfLine: "lf",
  jsxSingleQuote: true
}
```

### [READ Prettier Docs](https://prettier.io/docs/)