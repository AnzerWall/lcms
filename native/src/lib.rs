use lcms2::*;
use neon::prelude::*;

fn convert_cmyk2srgb(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let icc_buffer_v8 = cx.argument::<JsBuffer>(0)?;
    let source_buffer_v8 = cx.argument::<JsBuffer>(1)?;
    let intent = match cx.argument::<JsNumber>(2)?.value(&mut cx) as u32 {
        0 => Intent::Perceptual,
        1 => Intent::RelativeColorimetric,
        2 => Intent::Saturation,
        3 => Intent::AbsoluteColorimetric,
        _ => Intent::Perceptual,
    };

    let icc_buffer: &[u8] = cx.borrow(&icc_buffer_v8, |buf_data| buf_data.as_slice().clone());
    let source_buffer: Vec<(u8, u8, u8, u8)> = cx.borrow(&source_buffer_v8, |buf_data| {
        let s: &[u8] = buf_data.as_slice();
        let mut result: Vec<(u8, u8, u8, u8)> = Vec::with_capacity(s.len());
        let mut i = 0;
        while i < s.len() {
            result.push((s[i], s[i + 1], s[i + 2], s[i + 3]));
            i += 4;
        }
        result
    });
    let mut target_buffer: Vec<(u8, u8, u8)> = Vec::new();
    target_buffer.resize(source_buffer.len(), (0, 0, 0));
    let source_buffer = source_buffer.as_slice();

    let source_profile = Profile::new_icc(icc_buffer).unwrap();

    let t = Transform::new(
        &source_profile,
        PixelFormat::CMYK_8_REV,
        &Profile::new_srgb(),
        PixelFormat::RGB_8,
        intent,
    )
    .unwrap();
    t.transform_pixels(&source_buffer, &mut target_buffer.as_mut_slice());

    let mut target_buffer_v8 = JsBuffer::new(&mut cx, (target_buffer.len() * 3) as u32)?;
    cx.borrow_mut(&mut target_buffer_v8, |buf_data| {
        let s = buf_data.as_mut_slice();
        let mut i = 0;
        while i < s.len() {
            s[i] = target_buffer[i / 3].0;
            s[i + 1] = target_buffer[i / 3].1;
            s[i + 2] = target_buffer[i / 3].2;
            i += 3;
        }
    });

    Ok(target_buffer_v8)
}

fn convert_cmyk2srgb_by_pixel_float(mut cx: FunctionContext) -> JsResult<JsArray> {
    let icc_buffer_v8 = cx.argument::<JsBuffer>(0)?;
    let c = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let m = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let k = cx.argument::<JsNumber>(4)?.value(&mut cx);
    let intent = match cx.argument::<JsNumber>(5)?.value(&mut cx) as u32 {
        0 => Intent::Perceptual,
        1 => Intent::RelativeColorimetric,
        2 => Intent::Saturation,
        3 => Intent::AbsoluteColorimetric,
        _ => Intent::Perceptual,
    };
    let icc_buffer: &[u8] = cx.borrow(&icc_buffer_v8, |buf_data| buf_data.as_slice().clone());
    let source_buffer = [(c, m, y, k)];

    let mut target_buffer = [(0u8, 0u8, 0u8)];

    let source_profile = Profile::new_icc(icc_buffer).unwrap();

    let t = Transform::new(
        &source_profile,
        PixelFormat::CMYK_DBL,
        &Profile::new_srgb(),
        PixelFormat::RGB_8,
        intent,
    )
    .unwrap();
    t.transform_pixels(&source_buffer, &mut target_buffer);

    let result = JsArray::new(&mut cx, 3u32);
    let r = JsNumber::new(&mut cx, target_buffer[0].0);
    let g = JsNumber::new(&mut cx, target_buffer[0].1);
    let b = JsNumber::new(&mut cx, target_buffer[0].2);
    result.set(&mut cx, 0u32, r)?;
    result.set(&mut cx, 1u32, g)?;
    result.set(&mut cx, 2u32, b)?;

    Ok(result)
}
fn convert_cmyk2rgb(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let source_icc_buffer_v8 = cx.argument::<JsBuffer>(0)?;
    let target_icc_buffer_v8 = cx.argument::<JsBuffer>(1)?;

    let source_buffer_v8 = cx.argument::<JsBuffer>(2)?;
    let intent = match cx.argument::<JsNumber>(3)?.value(&mut cx) as u32 {
        0 => Intent::Perceptual,
        1 => Intent::RelativeColorimetric,
        2 => Intent::Saturation,
        3 => Intent::AbsoluteColorimetric,
        _ => Intent::Perceptual,
    };
    let source_icc_buffer: &[u8] = cx.borrow(&source_icc_buffer_v8, |buf_data| {
        buf_data.as_slice().clone()
    });
    let target_icc_buffer: &[u8] = cx.borrow(&target_icc_buffer_v8, |buf_data| {
        buf_data.as_slice().clone()
    });
    let source_buffer: Vec<(u8, u8, u8, u8)> = cx.borrow(&source_buffer_v8, |buf_data| {
        let s: &[u8] = buf_data.as_slice();
        let mut result: Vec<(u8, u8, u8, u8)> = Vec::with_capacity(s.len());
        let mut i = 0;
        while i < s.len() {
            result.push((s[i], s[i + 1], s[i + 2], s[i + 3]));
            i += 4;
        }
        result
    });
    let mut target_buffer: Vec<(u8, u8, u8)> = Vec::new();
    target_buffer.resize(source_buffer.len(), (0, 0, 0));
    let source_buffer = source_buffer.as_slice();

    let source_profile = Profile::new_icc(source_icc_buffer).unwrap();
    let target_profile = Profile::new_icc(target_icc_buffer).unwrap();

    let t = Transform::new(
        &source_profile,
        PixelFormat::CMYK_8_REV,
        &target_profile,
        PixelFormat::RGB_8,
        intent,
    )
    .unwrap();
    t.transform_pixels(&source_buffer, &mut target_buffer.as_mut_slice());

    let mut target_buffer_v8 = JsBuffer::new(&mut cx, (target_buffer.len() * 3) as u32)?;
    cx.borrow_mut(&mut target_buffer_v8, |buf_data| {
        let s = buf_data.as_mut_slice();
        let mut i = 0;
        while i < s.len() {
            s[i] = target_buffer[i / 3].0;
            s[i + 1] = target_buffer[i / 3].1;
            s[i + 2] = target_buffer[i / 3].2;
            i += 3;
        }
    });

    Ok(target_buffer_v8)
}

register_module!(mut cx, {
    cx.export_function("convert_cmyk2srgb", convert_cmyk2srgb)?;
    cx.export_function("convert_cmyk2rgb", convert_cmyk2rgb)?;
    cx.export_function(
        "convert_cmyk2srgb_by_pixel_float",
        convert_cmyk2srgb_by_pixel_float,
    )?;
    Ok(())
});
