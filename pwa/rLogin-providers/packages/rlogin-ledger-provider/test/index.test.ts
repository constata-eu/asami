import { convertFromHex } from '../src/helpers'
describe('helpers', () => {
  test('convertFromHex hello world', () => {
    // https://www.online-toolz.com/tools/text-hex-convertor.php
    expect(convertFromHex('0x68656c6c6f20776f726c64')).toStrictEqual('hello world')
  })

  test('convertFromHex other example', () => {
    expect(convertFromHex('0x416e6f74686572207465787420746f20626520636f6e76656e746564')).toStrictEqual('Another text to be convented')
  })

  test('convertFromHex a wrong example', () => {
    expect(convertFromHex('0x416e6f746865722074')).not.toStrictEqual('hello')
  })

  test('convertFromHex empty', () => {
    expect(convertFromHex('')).toStrictEqual('')
  })
})
