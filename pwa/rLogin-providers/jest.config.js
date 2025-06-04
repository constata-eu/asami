
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  reporters: ['default', 'jest-junit'],
  testResultsProcessor: 'jest-junit',
  transform: {
    '^.+\\.tsx?$': [
      'ts-jest',
      {
        compilerOptions: {
          preserveConstEnums: true,
          sourceMap: true,
          target: "es6",
          module: "commonjs",
          moduleResolution: "node",
          esModuleInterop: true,
          downlevelIteration: true,
          declarationMap: true,
          declaration: true,
          composite: true,
          emitDecoratorMetadata: true,
          experimentalDecorators: true,
          resolveJsonModule: true
        },
        exclude: ["**/test/**/*", "**/lib/**/*"]
      }
  ]
  },
}
