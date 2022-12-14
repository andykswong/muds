module.exports = api => {
  const isTest = api.env('test');

  const config = {
    assumptions: {
      setPublicClassFields: true
    },
    presets: [
      [
        '@babel/preset-env',
        {
          modules: false,
          targets: {
            node: true
          }
        }
      ]
    ],
    plugins: [],
    ignore: [
      'node_modules'
    ],
    comments: false,
    minified: true
  };

  if (!isTest) {
    config.plugins.push(
      ['babel-plugin-add-import-extension', { extension: 'js' }]
    );
    config.ignore.push('**/__tests__/**');
  }

  return config;
};
