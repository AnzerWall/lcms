var addon = require('../native');

const Intent = exports.Intent = {
  /// ICC Intents
  Perceptual: 0,
  RelativeColorimetric: 1,
  Saturation: 2,
  AbsoluteColorimetric: 3,
}


exports.convert_cmyk2srgb = function (iccp, input, intent = Intent.Perceptual) {
  return addon.convert_cmyk2srgb(iccp, input, intent)
}
exports.convert_cmyk2rgb = function (source_iccp, target_iccp, input, intent = Intent.Perceptual) {
  return addon.convert_cmyk2rgb(source_iccp, target_iccp, input, intent)
}

exports.convert_cmyk2srgb_by_pixel_float = function (iccp, c, m, y, k, intent = Intent.Perceptual) {
  return addon.convert_cmyk2srgb_by_pixel_float(iccp, c, m, y, k, intent)
}

