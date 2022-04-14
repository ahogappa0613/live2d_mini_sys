#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// mod test;
// use libc::*;
// use miniquad::*;
// use std::{
//     ffi::{CStr, CString},
//     fs::File,
//     io::Read,
//     path::{Path, PathBuf},
//     usize,
// };
// use test::csmVector2;
// pub struct Live2D(test::live2d_mini);
// pub struct Live2DMoc(*mut test::csmMoc);
// pub struct Live2DModel(*mut test::csmModel);

// pub struct Live2DDrawable {
//     pub texture_index: i32,
//     pub vertex_count: i32,
//     pub vertex_positions: Vec<csmVector2>,
//     pub vertex_uvs: Vec<csmVector2>,
// }
// pub struct Live2DObject {
//     drawables: Vec<Live2DDrawable>,
// }

// impl Live2D {
//     pub fn new() -> Result<Self, libloading::Error> {
//         unsafe {
//             let lib = test::live2d_mini::new(
//                 "/Users/ahogappa/project/live2d_mini_rs/live2d_mini_sys/Core/dll/macos/Live2DCubismCore.bundle",
//             )?;
//             Ok(Self(lib))
//         }
//     }

//     pub fn get_version(&self) -> (u32, u32, u32) {
//         let ver = unsafe { self.0.csmGetVersion() };

//         (ver >> 24, (ver >> 16) & 0xff, ver & 0xffff)
//     }

//     #[inline]
//     pub fn get_latest_moc_version(&self) -> u32 {
//         unsafe { self.0.csmGetLatestMocVersion() }
//     }

//     pub fn csm_set_log_function(&self, handler: fn(&CStr)) {
//         let handler = unsafe { std::mem::transmute(handler) };
//         unsafe { self.0.csmSetLogFunction(Some(handler)) };
//     }

//     #[inline]
//     pub fn csm_get_log_function(&self) -> test::csmLogFunction {
//         unsafe { self.0.csmGetLogFunction() }
//     }

//     pub fn load_file(&self, path: &str) {
//         miniquad::fs::load_file(path, |res| {
//             if let Ok(ref res) = res {
//                 let ho = miniquad::Texture::empty();
//                 ho;
//             }
//             // print!("{:?}", res);
//         });
//     }

//     pub fn read_blob_aligned(
//         &self,
//         path: &str,
//         align: usize,
//         out_size: &mut u64,
//     ) -> std::io::Result<Box<[u8]>> {
//         let mut buf = vec![];
//         let mut file = File::open(Path::new(path))?;
//         file.read_to_end(&mut buf)?;
//         let size = file.metadata().unwrap().len();

//         let alloc_mem = Self::allocate_aligned(size as usize, test::csmAlignofMoc);

//         unsafe { buf.set_len(alloc_mem.len()) };
//         *out_size = size;

//         Ok(buf.into_boxed_slice())
//     }

//     pub fn csm_revive_moc_in_place(&self, mem: &mut Box<[u8]>, size: u64) -> Live2DMoc {
//         unsafe {
//             Live2DMoc(
//                 self.0
//                     .csmReviveMocInPlace(mem.as_mut_ptr().cast::<_>(), size as _),
//             )
//         }
//     }

//     #[inline]
//     pub fn csm_get_sizeof_model(&self, moc: &Live2DMoc) -> u32 {
//         unsafe { self.0.csmGetSizeofModel(moc.0) as u32 }
//     }

//     pub fn csm_initialize_model_in_place(&self, moc: &Live2DMoc, size: u32) -> Live2DModel {
//         let alloc_mem = Self::allocate_aligned(size as usize, test::csmAlignofModel)
//             .as_mut_ptr()
//             .cast::<c_void>();

//         unsafe { Live2DModel(self.0.csmInitializeModelInPlace(moc.0, alloc_mem, size)) }
//     }

//     pub fn csm_get_moc_version(
//         &self,
//         address: *const ::std::os::raw::c_void,
//         size: ::std::os::raw::c_uint,
//     ) -> u32 {
//         unsafe { self.0.csmGetMocVersion(address, size) }
//     }

//     pub fn csm_read_canvas_info(
//         &self,
//         model: &Live2DModel,
//         size: &mut test::csmVector2,
//         origin: &mut test::csmVector2,
//         unit: &mut f32,
//     ) {
//         unsafe {
//             self.0.csmReadCanvasInfo(model.0, size, origin, unit);
//         }
//     }

//     #[inline]
//     pub fn csm_get_drawable_count(&self, model: &Live2DModel) -> i32 {
//         unsafe { self.0.csmGetDrawableCount(model.0) }
//     }

//     #[inline]
//     pub fn csm_get_part_count(&self, model: &Live2DModel) -> i32 {
//         unsafe { self.0.csmGetPartCount(model.0) }
//     }

//     #[inline]
//     pub fn csm_get_parameter_count(&self, model: &Live2DModel) -> i32 {
//         unsafe { self.0.csmGetParameterCount(model.0) }
//     }

//     #[inline]
//     pub fn csm_update_model(&self, model: &Live2DModel) {
//         unsafe { self.0.csmUpdateModel(model.0) }
//     }

//     pub fn csm_get_parameter_minimum_values(&self, model: &Live2DModel) -> Box<[f32]> {
//         let size = self.csm_get_parameter_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetParameterMinimumValues(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_parameter_maximum_values(&self, model: &Live2DModel) -> Box<[f32]> {
//         let size = self.csm_get_parameter_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetParameterMaximumValues(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_parameter_default_values(&self, model: &Live2DModel) -> Box<[f32]> {
//         let size = self.csm_get_parameter_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetParameterDefaultValues(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     // 返り値mutにする
//     pub fn csm_get_parameter_values(&self, model: &Live2DModel) -> Box<[f32]> {
//         let size = self.csm_get_parameter_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetParameterValues(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_index_counts(&self, model: &Live2DModel) -> Box<[i32]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableIndexCounts(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_texture_indices(&self, model: &Live2DModel) -> Box<[i32]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableTextureIndices(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_draw_orders(&self, model: &Live2DModel) -> Box<[i32]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableDrawOrders(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_render_orders(&self, model: &Live2DModel) -> Box<[i32]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableRenderOrders(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_opacities(&self, model: &Live2DModel) -> Box<[f32]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableOpacities(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_part_opacities(&self, model: &Live2DModel) -> Box<[f32]> {
//         let size = self.csm_get_part_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetPartOpacities(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_part_parent_part_indices(&self, model: &Live2DModel) -> Box<[i32]> {
//         let size = self.csm_get_part_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetPartParentPartIndices(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_mask_counts(&self, model: &Live2DModel) -> Box<[i32]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableMaskCounts(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_vertex_counts(&self, model: &Live2DModel) -> Box<[i32]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableVertexCounts(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     /// csmGetDrawableVertexPositions
//     ///
//     /// dynamic
//     pub fn csm_get_drawable_vertex_positions(&self, model: &Live2DModel) -> Vec<&[csmVector2]> {
//         let vertex_counts = self.csm_get_drawable_vertex_counts(model);
//         let drawable_count = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let positions = self.0.csmGetDrawableVertexPositions(model.0);
//             let vector_slice = std::slice::from_raw_parts(positions, drawable_count);
//             let ret = vector_slice
//                 .iter()
//                 .enumerate()
//                 .map(|(i, vec_p)| std::slice::from_raw_parts(*vec_p, vertex_counts[i] as _))
//                 .collect::<Vec<_>>();
//             ret
//         }
//     }

//     /// csmGetDrawableIndices
//     ///
//     /// static
//     pub fn csm_get_drawable_indices(&self, model: &Live2DModel) -> Box<[Vec<u16>]> {
//         let counts = self.csm_get_drawable_vertex_counts(model);
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let mut ret = vec![];
//             let hoge = self.0.csmGetDrawableIndices(model.0);
//             let j = std::slice::from_raw_parts(hoge, size);
//             for (i, b) in j.iter().enumerate() {
//                 let k = counts[i] as usize;
//                 let g = std::slice::from_raw_parts(*b, k).to_vec();
//                 ret.push(g);
//             }
//             ret.into_boxed_slice()
//         }
//     }

//     /// csmGetDrawableVertexUvs
//     ///
//     /// static
//     pub fn csm_get_drawable_vertex_uvs(&self, model: &Live2DModel) -> Vec<&[csmVector2]> {
//         let vertex_counts = self.csm_get_drawable_vertex_counts(model);
//         let drawable_count = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let positions = self.0.csmGetDrawableVertexUvs(model.0);
//             let vector_slice = std::slice::from_raw_parts(positions, drawable_count);
//             let ret = vector_slice
//                 .iter()
//                 .enumerate()
//                 .map(|(i, vec_p)| std::slice::from_raw_parts(*vec_p, vertex_counts[i] as _))
//                 .collect::<Vec<_>>();
//             ret
//         }
//     }

//     pub fn csm_get_drawable_constant_flags(&self, model: &Live2DModel) -> Box<[u8]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableConstantFlags(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_dynamic_flags(&self, model: &Live2DModel) -> Box<[u8]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableDynamicFlags(model.0);
//             let j = std::slice::from_raw_parts(hoge, size).to_vec();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_drawable_ids(&self, model: &Live2DModel) -> Box<[&CStr]> {
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetDrawableIds(model.0);
//             let j = std::slice::from_raw_parts(hoge, size)
//                 .iter()
//                 .map(|ptr| CStr::from_ptr(*ptr))
//                 .collect::<Vec<_>>();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_part_ids(&self, model: &Live2DModel) -> Box<[&CStr]> {
//         let size = self.csm_get_part_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetPartIds(model.0);
//             let j = std::slice::from_raw_parts(hoge, size)
//                 .iter()
//                 .map(|ptr| CStr::from_ptr(*ptr))
//                 .collect::<Vec<_>>();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_get_parameter_ids(&self, model: &Live2DModel) -> Box<[&CStr]> {
//         let size = self.csm_get_parameter_count(model) as usize;
//         unsafe {
//             let hoge = self.0.csmGetParameterIds(model.0);
//             let j = std::slice::from_raw_parts(hoge, size)
//                 .iter()
//                 .map(|ptr| CStr::from_ptr(*ptr))
//                 .collect::<Vec<_>>();
//             j.into_boxed_slice()
//         }
//     }

//     pub fn csm_reset_drawable_dynamic_flags(&self, model: &Live2DModel) {
//         unsafe { self.0.csmResetDrawableDynamicFlags(model.0) }
//     }

//     // 配列の配列の変換をなんとかする
//     pub fn csm_get_drawable_masks(&self, model: &Live2DModel) -> Box<[Vec<i32>]> {
//         let counts = self.csm_get_drawable_mask_counts(model);
//         let size = self.csm_get_drawable_count(model) as usize;
//         unsafe {
//             let mut ret = vec![];
//             let hoge = self.0.csmGetDrawableMasks(model.0);
//             let j = std::slice::from_raw_parts(hoge, size);
//             for (i, b) in j.iter().enumerate() {
//                 let k = counts[i] as usize;
//                 let g = std::slice::from_raw_parts(*b, k).to_vec();
//                 ret.push(g);
//             }
//             ret.into_boxed_slice()
//         }
//     }

//     #[inline]
//     pub fn is_csm_is_visible(&self, flags: u8) -> bool {
//         if (flags as u32 & test::csmIsVisible) != test::csmIsVisible {
//             true
//         } else {
//             false
//         }
//     }

//     #[inline]
//     pub fn is_csm_visibility_did_change(&self, flags: u8) -> bool {
//         if (flags as u32 & test::csmVisibilityDidChange) != test::csmVisibilityDidChange {
//             true
//         } else {
//             false
//         }
//     }

//     #[inline]
//     pub fn is_csm_opacity_did_change(&self, flags: u8) -> bool {
//         if (flags as u32 & test::csmOpacityDidChange) != test::csmOpacityDidChange {
//             true
//         } else {
//             false
//         }
//     }

//     #[inline]
//     pub fn is_csm_draw_order_did_change(&self, flags: u8) -> bool {
//         if (flags as u32 & test::csmDrawOrderDidChange) != test::csmDrawOrderDidChange {
//             true
//         } else {
//             false
//         }
//     }

//     #[inline]
//     pub fn is_csm_render_order_did_change(&self, flags: u8) -> bool {
//         if (flags as u32 & test::csmRenderOrderDidChange) != test::csmRenderOrderDidChange {
//             true
//         } else {
//             false
//         }
//     }

//     #[inline]
//     pub fn is_vertex_positions_did_change(&self, flags: u8) -> bool {
//         if (flags as u32 & test::csmVertexPositionsDidChange) != test::csmVertexPositionsDidChange {
//             true
//         } else {
//             false
//         }
//     }

//     fn allocate_aligned(size: usize, align: u32) -> Box<[u8]> {
//         unsafe {
//             // 余りを切り上げながら4096で割る
//             let size = (size + align as usize - 1) / align as usize;
//             // メモリ確保
//             let mut vec = Vec::<AlignedData>::with_capacity(size);
//             // データの書き込み
//             vec.resize_with(size, Default::default);
//             // アライン付き確保が終わったのでデータをu8にする
//             let mut data = std::mem::transmute::<_, Vec<u8>>(vec);
//             // そのままだと4096分の1の要素しかないのでメタデータを変更する
//             data.set_len(size * align as usize);
//             data.into_boxed_slice()
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Live2dVector(pub test::csmVector2);
// impl Live2dVector {
//     pub fn new() -> Self {
//         Self(test::csmVector2 { X: 0.0, Y: 0.0 })
//     }
// }

// #[repr(align(64))]
// struct AlignedData {
//     #[allow(unused)]
//     data: [u8; test::csmAlignofMoc as usize],
// }

// impl Default for AlignedData {
//     fn default() -> Self {
//         AlignedData {
//             data: [0; test::csmAlignofMoc as usize],
//         }
//     }
// }
