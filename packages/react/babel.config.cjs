module.exports = api => {
  const isWebpack = api.env('webpack');
  const isTest = api.env('test');

  const config = {
    "assumptions": {
      "setPublicClassFields": true
    },
    "presets": [
      [
        "@babel/preset-env",
        {
          "modules": false,
          "targets": {
            "node": true
          }
        }
      ],
      ["@babel/preset-typescript"],
      ["@babel/preset-react"]
    ],
    plugins: [],
    "ignore": ["node_modules"],
    "comments": false,
    "minified": true
  };

  if (!isTest && !isWebpack) {
    config.plugins.push(
      ["babel-plugin-add-import-extension", { "extension": "js" }]
    );
  }

  return config;
};
