
extern crate std;
extern crate libc;
extern crate va_list;

use self::libc::{c_void, c_float};
use self::va_list::VaList;
use std::ffi::CString;

#[repr(C)]
pub enum RendererKind {
    Noop,
    Direct3d9,
    Direct3d11,
    Direct3d12,
    Gnm,
    Metal,
    Opengles,
    Opengl,
    Vulkan,
}

pub const RENDERER_KIND_COUNT: usize = 9;

#[repr(C)]
pub enum Access {
    Read,
    Write,
    Readwrite,
}

pub const ACCESS_COUNT: usize = 3;

#[repr(C)]
pub enum Attrib {
    Position,
    Normal,
    Tangent,
    Bitangent,
    Color0,
    Color1,
    Color2,
    Color3,
    Indices,
    Weight,
    Texcoord0,
    Texcoord1,
    Texcoord2,
    Texcoord3,
    Texcoord4,
    Texcoord5,
    Texcoord6,
    Texcoord7,
}

pub const ATTRIB_COUNT: usize = 18;

#[repr(C)]
pub enum AttribKind {
    Uint8,
    Uint10,
    Int16,
    Half,
    Float,
}

pub const ATTRIB_KIND_COUNT: usize = 5;

#[repr(C)]
pub enum TextureFormat {
    Bc1,
    Bc2,
    Bc3,
    Bc4,
    Bc5,
    Bc6h,
    Bc7,
    Etc1,
    Etc2,
    Etc2a,
    Etc2a1,
    Ptc12,
    Ptc14,
    Ptc12a,
    Ptc14a,
    Ptc22,
    Ptc24,
    Atc,
    Atce,
    Atci,
    Astc4x4,
    Astc5x5,
    Astc6x6,
    Astc8x5,
    Astc8x6,
    Astc10x5,
    Unknown,
    R1,
    A8,
    R8,
    R8i,
    R8u,
    R8s,
    R16,
    R16i,
    R16u,
    R16f,
    R16s,
    R32i,
    R32u,
    R32f,
    Rg8,
    Rg8i,
    Rg8u,
    Rg8s,
    Rg16,
    Rg16i,
    Rg16u,
    Rg16f,
    Rg16s,
    Rg32i,
    Rg32u,
    Rg32f,
    Rgb8,
    Rgb8i,
    Rgb8u,
    Rgb8s,
    Rgb9e5f,
    Bgra8,
    Rgba8,
    Rgba8i,
    Rgba8u,
    Rgba8s,
    Rgba16,
    Rgba16i,
    Rgba16u,
    Rgba16f,
    Rgba16s,
    Rgba32i,
    Rgba32u,
    Rgba32f,
    R5g6b5,
    Rgba4,
    Rgb5a1,
    Rgb10a2,
    Rg11b10f,
    UnknownDepth,
    D16,
    D24,
    D24s8,
    D32,
    D16f,
    D24f,
    D32f,
    D0s8,
}

pub const TEXTURE_FORMAT_COUNT: usize = 85;

#[repr(C)]
pub enum UniformKind {
    Int1,
    End,
    Vec4,
    Mat3,
    Mat4,
}

pub const UNIFORM_KIND_COUNT: usize = 5;

#[repr(C)]
pub enum BackbufferRatio {
    Equal,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    Double,
}

pub const BACKBUFFER_RATIO_COUNT: usize = 6;

#[repr(C)]
pub enum OcclusionQueryResult {
    Invisible,
    Visible,
    Noresult,
}

pub const OCCLUSION_QUERY_RESULT_COUNT: usize = 3;

#[repr(C)]
pub enum Topology {
    TriList,
    TriStrip,
    LineList,
    LineStrip,
    PointList,
}

pub const TOPOLOGY_COUNT: usize = 5;

#[repr(C)]
pub enum TopologyConvert {
    TriListFlipWinding,
    TriStripFlipWinding,
    TriListToLineList,
    TriStripToTriList,
    LineStripToLineList,
}

pub const TOPOLOGY_CONVERT_COUNT: usize = 5;

#[repr(C)]
pub enum TopologySort {
    DirectionFrontToBackMin,
    DirectionFrontToBackAvg,
    DirectionFrontToBackMax,
    DirectionBackToFrontMin,
    DirectionBackToFrontAvg,
    DirectionBackToFrontMax,
    DistanceFrontToBackMin,
    DistanceFrontToBackAvg,
    DistanceFrontToBackMax,
    DistanceBackToFrontMin,
    DistanceBackToFrontAvg,
    DistanceBackToFrontMax,
}

pub const TOPOLOGY_SORT_COUNT: usize = 12;

#[repr(C)]
pub enum ViewMode {
    Default,
    Sequential,
    DepthAscending,
    DepthDescending,
}

pub const VIEW_MODE_COUNT: usize = 4;

pub type DynamicIndexBufferHandle = u16;
pub type DynamicVertexBufferHandle = u16;
pub type FrameBufferHandle = u16;
pub type IndexBufferHandle = u16;
pub type IndirectBufferHandle = u16;
pub type OcclusionQueryHandle = u16;
pub type ProgramHandle = u16;
pub type ShaderHandle = u16;
pub type TextureHandle = u16;
pub type UniformHandle = u16;
pub type VertexBufferHandle = u16;
pub type VertexDeclHandle = u16;

pub type ReleaseFn = extern fn(ptr: *mut c_void, userData: *mut c_void);

#[repr(C)]
pub struct Memory {
    pub data: *mut u8,
    pub size: u32,
}

#[repr(C)]
pub struct Transform {
    pub data: *mut c_float,
    pub num : u16,
}

pub type ViewId = u16;

#[repr(C)]
pub struct ViewStats {
    pub name            : [u8; 256],
    pub view            : ViewId,
    pub cpu_time_elapsed: i64,
    pub gpu_time_elapsed: i64,
}

#[repr(C)]
pub struct EncoderStats {
    pub cpu_time_begin: i64,
    pub cpu_time_end  : i64,
}

#[repr(C)]
pub struct Stats {
    pub cpu_time_frame            : i64,
    pub cpu_time_begin            : i64,
    pub cpu_time_end              : i64,
    pub cpu_timer_freq            : i64,
    pub gpu_time_begin            : i64,
    pub gpu_time_end              : i64,
    pub gpu_timer_freq            : i64,
    pub wait_render               : i64,
    pub wait_submit               : i64,
    pub num_draw                  : u32,
    pub num_compute               : u32,
    pub max_gpu_latency           : u32,
    pub num_dynamic_index_buffers : u16,
    pub num_dynamic_vertex_buffers: u16,
    pub num_frame_buffers         : u16,
    pub num_index_buffers         : u16,
    pub num_occlusion_queries     : u16,
    pub num_programs              : u16,
    pub num_shaders               : u16,
    pub num_textures              : u16,
    pub num_uniforms              : u16,
    pub num_vertex_buffers        : u16,
    pub num_vertex_decls          : u16,
    pub texture_memory_used       : i64,
    pub rt_memory_used            : i64,
    pub transient_vb_used         : i32,
    pub transient_ib_used         : i32,
    pub num_prims                 : [u32; TOPOLOGY_COUNT],
    pub gpu_memory_max            : i64,
    pub gpu_memory_used           : i64,
    pub width                     : u16,
    pub height                    : u16,
    pub text_width                : u16,
    pub text_height               : u16,
    pub num_views                 : u16,
    pub view_stats                : *mut ViewStats,
    pub num_encoders              : u8,
    pub encoder_stats             : *mut EncoderStats,
}

#[repr(C)]
pub struct VertexDecl {
    pub hash      : u32,
    pub stride    : u16,
    pub offset    : [u16; ATTRIB_COUNT],
    pub attributes: [u16; ATTRIB_COUNT],
}

#[repr(C)]
pub struct TransientIndexBuffer {
    pub data       : *mut u8,
    pub size       : u32,
    pub handle     : IndexBufferHandle,
    pub start_index: u32,
}

#[repr(C)]
pub struct TransientVertexBuffer {
    pub data        : *mut u8,
    pub size        : u32,
    pub start_vertex: u32,
    pub stride      : u16,
    pub handle      : VertexBufferHandle,
    pub decl        : VertexDeclHandle,
}

#[repr(C)]
pub struct InstanceDataBuffer {
    pub data  : *mut u8,
    pub size  : u32,
    pub offset: u32,
    pub num   : u32,
    pub stride: u16,
    pub handle: VertexBufferHandle,
}

#[repr(C)]
pub struct TextureInfo {
    pub format        : TextureFormat,
    pub storage_size  : u32,
    pub width         : u16,
    pub height        : u16,
    pub depth         : u16,
    pub num_layers    : u16,
    pub num_mips      : u8,
    pub bits_per_pixel: u8,
    pub cube_map      : bool,
}

#[repr(C)]
pub struct UniformInfo {
    pub name: [u8; 256],
    pub kind: UniformKind,
    pub num : u16,
}

#[repr(C)]
pub struct Attachment {
    pub handle: TextureHandle,
    pub mip   : u16,
    pub layer : u16,
}

#[repr(C)]
pub struct CapsGpu {
    pub vendor_id: u16,
    pub device_id: u16,
}

#[repr(C)]
pub struct CapsLimits {
    pub max_draw_calls            : u32,
    pub max_blits                 : u32,
    pub max_texture_size          : u32,
    pub max_texture_layers        : u32,
    pub max_views                 : u32,
    pub max_frame_buffers         : u32,
    pub max_fbattachments         : u32,
    pub max_programs              : u32,
    pub max_shaders               : u32,
    pub max_textures              : u32,
    pub max_texture_samplers      : u32,
    pub max_vertex_decls          : u32,
    pub max_vertex_streams        : u32,
    pub max_index_buffers         : u32,
    pub max_vertex_buffers        : u32,
    pub max_dynamic_index_buffers : u32,
    pub max_dynamic_vertex_buffers: u32,
    pub max_uniforms              : u32,
    pub max_occlusion_queries     : u32,
    pub max_encoders              : u32,
    pub transient_vb_size         : u32,
    pub transient_ib_size         : u32,
}

#[repr(C)]
pub struct Caps {
    pub renderer_kind     : RendererKind,
    pub supported         : u64,
    pub vendor_id         : u16,
    pub device_id         : u16,
    pub homogeneous_depth : bool,
    pub origin_bottom_left: bool,
    pub num_gpus          : u8,
    pub gpu               : [CapsGpu; 4],
    pub limits            : CapsLimits,
    pub formats           : [u16; TEXTURE_FORMAT_COUNT],
}

#[repr(C)]
pub enum Fatal {
    DebugCheck,
    InvalidShader,
    UnableToInitialize,
    UnableToCreateTexture,
    DeviceLost,
    Count
}

#[repr(C)]
pub struct CallbackVtbl {
    fatal: extern fn(this:*mut CallbackInterface,
                     code: Fatal,
                     str: CString),
    trace_vargs: extern fn(this:*mut CallbackInterface,
                           filePath: CString,
                           line: u16,
                           format: CString,
                           argList: VaList),
    profiler_begin: extern fn(this:*mut CallbackInterface,
                              name: CString,
                              abgr: u32,
                              filePath: CString,
                              line: u16),
    profiler_begin_literal: extern fn(this:*mut CallbackInterface,
                                      name: CString,
                                      abgr: u32,
                                      filePath: CString,
                                      line: u16),
    profiler_end: extern fn(this: *mut CallbackInterface),
    cache_read_size: extern fn(this: *mut CallbackInterface,
                               id: u64) -> u32,
    cache_read: extern fn(this: *mut CallbackInterface,
                          id: u64,
                          data: *mut c_void,
                          size: u32) -> bool,
    cache_write: extern fn(this: *mut CallbackInterface,
                           id: u64,
                           data: *const c_void,
                           size: u32),
    screen_shot: extern fn(this: *mut CallbackInterface,
                           filePath: CString,
                           width: u32,
                           height: u32,
                           pitch: u32,
                           data: *const c_void,
                           size: u32,
                           yflip: bool),
    capture_begin: extern fn(this: *mut CallbackInterface,
                             width: u32,
                             height: u32,
                             pitch: u32,
                             format: TextureFormat,
                             yflip: bool),
    capture_end: extern fn(this: *mut CallbackInterface),
    capture_frame: extern fn(this:*mut CallbackInterface,
                             data: *const c_void,
                             size: u32),
}

#[repr(C)]
pub struct CallbackInterface {
    pub vtbl: *const CallbackVtbl,
}

#[repr(C)]
pub struct AllocatorInterface {
    pub vtbl: *const AllocatorVtbl,
}

#[repr(C)]
pub struct AllocatorVtbl {
    pub realloc: extern fn(this: *mut AllocatorInterface,
                           ptr: *mut c_void,
                           size: usize,
                           align: usize,
                           file: CString,
                           line: u32) -> *mut c_void,
}

#[repr(C)]
pub struct Resolution {
    pub width : u32,
    pub height: u32,
    pub reset : u32,
}

#[repr(C)]
pub struct InitLimits {
    pub max_encoders     : u16,
    pub transient_vb_size: u32,
    pub transient_ib_size: u32,
}

#[repr(C)]
pub struct Init {
    pub kind      : RendererKind,
    pub vendor_id : u16,
    pub device_id : u16,
    pub debug     : bool,
    pub profile   : bool,
    pub resolution: Resolution,
    pub limits    : InitLimits,
    pub callback  : *mut CallbackInterface,
    pub allocator : *mut AllocatorInterface,
}

pub type Encoder = c_void;

extern "C" {
    fn bgfx_vertex_decl_begin(decl: *mut VertexDecl, renderer: RendererKind);
    fn bgfx_vertex_decl_add(decl: *mut VertexDecl, attrib: Attrib, num: u8, kind: AttribKind, normalized: bool, asInt: bool);
    fn bgfx_vertex_decl_decode(decl: *const VertexDecl, attrib: Attrib, num: *mut u8, kind: *mut AttribKind, normalized: *mut bool, asInt: *mut bool);
    fn bgfx_vertex_decl_has(decl: *const VertexDecl, attrib: Attrib) -> bool;
    fn bgfx_vertex_decl_skip(decl: *mut VertexDecl, num: u8);
    fn bgfx_vertex_decl_end(decl: *mut VertexDecl );
    fn bgfx_vertex_pack(input: [c_float; 4], inputNormalized: bool, attr: Attrib, decl: *const VertexDecl, data: *mut c_void, index: u32);
    fn bgfx_vertex_unpack(output: [c_float; 4], attr: Attrib, decl: *const VertexDecl, data: *const c_void, index: u32);
    fn bgfx_vertex_convert(destDecl: *const VertexDecl, destData: *mut c_void, srcDecl: *const VertexDecl, srcData: *const c_void, num: u32);
    fn bgfx_weld_vertices(output: *mut u16, decl: *const VertexDecl, data: *const c_void, num: u16, epsilon: c_float) -> u16;
    fn bgfx_topology_convert(conversion: TopologyConvert, dst: *mut c_void, dstSize: u32, indices: *const c_void, numIndices: u32, index32: bool) -> u32;
    fn bgfx_topology_sort_tri_list(sort: TopologySort, dst: *mut c_void, dstSize: u32, dir: [c_float; 3], pos: [c_float; 3], vertices: *const c_void, stride: u32, indices: *const c_void, numIndices: u32, index32: bool);
    fn bgfx_get_supported_renderers(max: u8, kind: *mut RendererKind ) -> u8;
    fn bgfx_get_renderer_name(kind: RendererKind) -> CString;
    fn bgfx_init_ctor(init: *mut Init );
    fn bgfx_init(init: *const Init ) -> bool;
    fn bgfx_shutdown();
    fn bgfx_reset(width: u32, height: u32, flags: u32);
    fn bgfx_begin() -> *mut Encoder;
    fn bgfx_end(encoder: *mut Encoder );
    fn bgfx_frame(capture: bool) -> u32;
    fn bgfx_get_renderer_type() -> RendererKind;
    fn bgfx_get_caps() -> *const Caps;
    fn bgfx_get_stats() -> *const Stats;
    fn bgfx_alloc(size: u32) -> *const Memory;
    fn bgfx_copy(data: *const c_void, size: u32) -> *const Memory;
    fn bgfx_make_ref(data: *const c_void, size: u32) -> *const Memory;
    fn bgfx_make_ref_release(data: *const c_void, size: u32, releaseFn: ReleaseFn, userData: *mut c_void) -> *const Memory;
    fn bgfx_set_debug(debug: u32);
    fn bgfx_dbg_text_clear(attr: u8, small: bool);
    fn bgfx_dbg_text_printf(x: u16, y: u16, attr: u8, format: CString, ... );
    fn bgfx_dbg_text_vprintf(x: u16, y: u16, attr: u8, format: CString, argList: VaList);
    fn bgfx_dbg_text_image(x: u16, y: u16, width: u16, height: u16, data: *const c_void, pitch: u16);
    fn bgfx_create_index_buffer(mem: *const Memory, flags: u16) -> IndexBufferHandle;
    fn bgfx_destroy_index_buffer(handle: IndexBufferHandle);
    fn bgfx_create_vertex_buffer(mem: *const Memory, decl: *const VertexDecl, flags: u16) -> VertexBufferHandle;
    fn bgfx_destroy_vertex_buffer(handle: VertexBufferHandle);
    fn bgfx_create_dynamic_index_buffer(num: u32, flags: u16) -> DynamicIndexBufferHandle;
    fn bgfx_create_dynamic_index_buffer_mem(mem: *const Memory, flags: u16) -> DynamicIndexBufferHandle;
    fn bgfx_update_dynamic_index_buffer(handle: DynamicIndexBufferHandle, startIndex: u32, mem: *const Memory );
    fn bgfx_destroy_dynamic_index_buffer(handle: DynamicIndexBufferHandle);
    fn bgfx_create_dynamic_vertex_buffer(num: u32, decl: *const VertexDecl, flags: u16) -> DynamicVertexBufferHandle;
    fn bgfx_create_dynamic_vertex_buffer_mem(mem: *const Memory, decl: *const VertexDecl, flags: u16) -> DynamicVertexBufferHandle;
    fn bgfx_update_dynamic_vertex_buffer(handle: DynamicVertexBufferHandle, startVertex: u32, mem: *const Memory );
    fn bgfx_destroy_dynamic_vertex_buffer(handle: DynamicVertexBufferHandle);
    fn bgfx_get_avail_transient_index_buffer(num: u32) -> u32;
    fn bgfx_get_avail_transient_vertex_buffer(num: u32, decl: *const VertexDecl) -> u32;
    fn bgfx_get_avail_instance_data_buffer(num: u32, stride: u16) -> u32;
    fn bgfx_alloc_transient_index_buffer(tib: *mut TransientIndexBuffer, num: u32);
    fn bgfx_alloc_transient_vertex_buffer(tvb: *mut TransientVertexBuffer, num: u32, decl: *const VertexDecl );
    fn bgfx_alloc_transient_buffers(tvb: *mut TransientVertexBuffer, decl: *const VertexDecl, numVertices: u32, tib: *mut TransientIndexBuffer, numIndices: u32) -> bool;
    fn bgfx_alloc_instance_data_buffer(idb: *mut InstanceDataBuffer, num: u32, stride: u16);
    fn bgfx_create_indirect_buffer(num: u32) -> IndirectBufferHandle;
    fn bgfx_destroy_indirect_buffer(handle: IndirectBufferHandle);
    fn bgfx_create_shader(mem: *const Memory ) -> ShaderHandle;
    fn bgfx_get_shader_uniforms(handle: ShaderHandle, uniforms: *mut UniformHandle, max: u16) -> u16;
    fn bgfx_get_uniform_info(handle: UniformHandle, info: *mut UniformInfo );
    fn bgfx_set_shader_name(handle: ShaderHandle, name: CString, len: i32);
    fn bgfx_destroy_shader(handle: ShaderHandle);
    fn bgfx_create_program(vsh: ShaderHandle, fsh: ShaderHandle, destroyShaders: bool) -> ProgramHandle;
    fn bgfx_create_compute_program(csh: ShaderHandle, destroyShaders: bool) -> ProgramHandle;
    fn bgfx_destroy_program(handle: ProgramHandle);
    fn bgfx_is_texture_valid(depth: u16, cubeMap: bool, numLayers: u16, format: TextureFormat, flags: u32) -> bool;
    fn bgfx_calc_texture_size(info: *mut TextureInfo, width: u16, height: u16, depth: u16, cubeMap: bool, hasMips: bool, numLayers: u16, format: TextureFormat);
    fn bgfx_create_texture(mem: *const Memory, flags: u32, skip: u8, info: *mut TextureInfo ) -> TextureHandle;
    fn bgfx_create_texture_2d(width: u16, height: u16, hasMips: bool, numLayers: u16, format: TextureFormat, flags: u32, mem: *const Memory ) -> TextureHandle;
    fn bgfx_create_texture_2d_scaled(ratio: BackbufferRatio, hasMips: bool, numLayers: u16, format: TextureFormat, flags: u32) -> TextureHandle;
    fn bgfx_create_texture_3d(width: u16, height: u16, depth: u16, hasMips: bool, format: TextureFormat, flags: u32, mem: *const Memory ) -> TextureHandle;
    fn bgfx_create_texture_cube(size: u16, hasMips: bool, numLayers: u16, format: TextureFormat, flags: u32, mem: *const Memory ) -> TextureHandle;
    fn bgfx_update_texture_2d(handle: TextureHandle, layer: u16, mip: u8, x: u16, y: u16, width: u16, height: u16, mem: *const Memory, pitch: u16);
    fn bgfx_update_texture_3d(handle: TextureHandle, mip: u8, x: u16, y: u16, z: u16, width: u16, height: u16, depth: u16, mem: *const Memory );
    fn bgfx_update_texture_cube(handle: TextureHandle, layer: u16, side: u8, mip: u8, x: u16, y: u16, width: u16, height: u16, mem: *const Memory, pitch: u16);
    fn bgfx_read_texture(handle: TextureHandle, data: *mut c_void, mip: u8) -> u32;
    fn bgfx_set_texture_name(handle: TextureHandle, name: CString, len: i32);
    fn bgfx_destroy_texture(handle: TextureHandle);
    fn bgfx_create_frame_buffer(width: u16, height: u16, format: TextureFormat, textureFlags: u32) -> FrameBufferHandle;
    fn bgfx_create_frame_buffer_scaled(ratio: BackbufferRatio, format: TextureFormat, textureFlags: u32) -> FrameBufferHandle;
    fn bgfx_create_frame_buffer_from_handles(num: u8, handles: *const TextureHandle, destroyTextures: bool) -> FrameBufferHandle;
    fn bgfx_create_frame_buffer_from_attachment(num: u8, attachment: *const Attachment, destroyTextures: bool) -> FrameBufferHandle;
    fn bgfx_create_frame_buffer_from_nwh(nwh: *mut c_void, width: u16, height: u16, depthFormat: TextureFormat) -> FrameBufferHandle;
    fn bgfx_get_texture(handle: FrameBufferHandle, attachment: u8) -> TextureHandle;
    fn bgfx_destroy_frame_buffer(handle: FrameBufferHandle);
    fn bgfx_create_uniform(name: CString, kind: UniformKind, num: u16) -> UniformHandle;
    fn bgfx_destroy_uniform(handle: UniformHandle);
    fn bgfx_create_occlusion_query() -> OcclusionQueryHandle;
    fn bgfx_get_result(handle: OcclusionQueryHandle, result: *mut i32) -> OcclusionQueryResult;
    fn bgfx_destroy_occlusion_query(handle: OcclusionQueryHandle);
    fn bgfx_set_palette_color(index: u8, rgba: [c_float; 4]);
    fn bgfx_set_view_name(id: ViewId, name: CString);
    fn bgfx_set_view_rect(id: ViewId, x: u16, y: u16, width: u16, height: u16);
    fn bgfx_set_view_rect_auto(id: ViewId, x: u16, y: u16, ratio: BackbufferRatio);
    fn bgfx_set_view_scissor(id: ViewId, x: u16, y: u16, width: u16, height: u16);
    fn bgfx_set_view_clear(id: ViewId, flags: u16, rgba: u32, depth: c_float, stencil: u8);
    fn bgfx_set_view_clear_mrt(id: ViewId, flags: u16, depth: c_float, stencil: u8, param0: u8, param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8);
    fn bgfx_set_view_mode(id: ViewId, mode: ViewMode);
    fn bgfx_set_view_frame_buffer(id: ViewId, handle: FrameBufferHandle);
    fn bgfx_set_view_transform(id: ViewId, view: *const c_void, proj: *const c_void);
    fn bgfx_set_view_transform_stereo(id: ViewId, view: *const c_void, projL: *const c_void, flags: u8, projR: *const c_void);
    fn bgfx_set_view_order(id: ViewId, num: u16, order: *const ViewId);
    fn bgfx_reset_view(id: ViewId);
    fn bgfx_set_marker(marker: CString);
    fn bgfx_set_state(state: u64, rgba: u32);
    fn bgfx_set_condition(handle: OcclusionQueryHandle, visible: bool);
    fn bgfx_set_stencil(fstencil: u32, bstencil: u32);
    fn bgfx_set_scissor(x: u16, y: u16, width: u16, height: u16) -> u16;
    fn bgfx_set_scissor_cached(cache: u16);
    fn bgfx_set_transform(mtx: *const c_void, num: u16) -> u32;
    fn bgfx_alloc_transform(transform: *mut Transform, num: u16) -> u32;
    fn bgfx_set_transform_cached(cache: u32, num: u16);
    fn bgfx_set_uniform(handle: UniformHandle, value: *const c_void, num: u16);
    fn bgfx_set_index_buffer(handle: IndexBufferHandle, firstIndex: u32, numIndices: u32);
    fn bgfx_set_dynamic_index_buffer(handle: DynamicIndexBufferHandle, firstIndex: u32, numIndices: u32);
    fn bgfx_set_transient_index_buffer(tib: *const TransientIndexBuffer, firstIndex: u32, numIndices: u32);
    fn bgfx_set_vertex_buffer(stream: u8, handle: VertexBufferHandle, startVertex: u32, numVertices: u32);
    fn bgfx_set_dynamic_vertex_buffer(stream: u8, handle: DynamicVertexBufferHandle, startVertex: u32, numVertices: u32);
    fn bgfx_set_transient_vertex_buffer(stream: u8, tvb: *const TransientVertexBuffer, startVertex: u32, numVertices: u32);
    fn bgfx_set_vertex_count(numVertices: u32);
    fn bgfx_set_instance_data_buffer(idb: *const InstanceDataBuffer, start: u32, num: u32);
    fn bgfx_set_instance_data_from_vertex_buffer(handle: VertexBufferHandle, startVertex: u32, num: u32);
    fn bgfx_set_instance_data_from_dynamic_vertex_buffer(handle: DynamicVertexBufferHandle, startVertex: u32, num: u32);
    fn bgfx_set_texture(stage: u8, sampler: UniformHandle, handle: TextureHandle, flags: u32);
    fn bgfx_touch(id: ViewId);
    fn bgfx_submit(id: ViewId, handle: ProgramHandle, depth: i32, preserveState: bool);
    fn bgfx_submit_occlusion_query(id: ViewId, program: ProgramHandle, occlusionQuery: OcclusionQueryHandle, depth: i32, preserveState: bool);
    fn bgfx_submit_indirect(id: ViewId, handle: ProgramHandle, indirectHandle: IndirectBufferHandle, start: u16, num: u16, depth: i32, preserveState: bool);
    fn bgfx_set_image(stage: u8, handle: TextureHandle, mip: u8, access: Access, format: TextureFormat);
    fn bgfx_set_compute_index_buffer(stage: u8, handle: IndexBufferHandle, access: Access);
    fn bgfx_set_compute_vertex_buffer(stage: u8, handle: VertexBufferHandle, access: Access);
    fn bgfx_set_compute_dynamic_index_buffer(stage: u8, handle: DynamicIndexBufferHandle, access: Access);
    fn bgfx_set_compute_dynamic_vertex_buffer(stage: u8, handle: DynamicVertexBufferHandle, access: Access);
    fn bgfx_set_compute_indirect_buffer(stage: u8, handle: IndirectBufferHandle, access: Access);
    fn bgfx_dispatch(id: ViewId, handle: ProgramHandle, numX: u32, numY: u32, numZ: u32, flags: u8);
    fn bgfx_dispatch_indirect(id: ViewId, handle: ProgramHandle, indirectHandle: IndirectBufferHandle, start: u16, num: u16, flags: u8);
    fn bgfx_discard();
    fn bgfx_blit(id: ViewId, dst: TextureHandle, dstMip: u8, dstX: u16, dstY: u16, dstZ: u16, src: TextureHandle, srcMip: u8, srcX: u16, srcY: u16, srcZ: u16, width: u16, height: u16, depth: u16);
    fn bgfx_encoder_set_marker(encoder: *mut Encoder, marker: CString);
    fn bgfx_encoder_set_state(encoder: *mut Encoder, state: u64, rgba: u32);
    fn bgfx_encoder_set_condition(encoder: *mut Encoder, handle: OcclusionQueryHandle, visible: bool);
    fn bgfx_encoder_set_stencil(encoder: *mut Encoder, fstencil: u32, bstencil: u32);
    fn bgfx_encoder_set_scissor(encoder: *mut Encoder, x: u16, y: u16, width: u16, height: u16) -> u16;
    fn bgfx_encoder_set_scissor_cached(encoder: *mut Encoder, cache: u16);
    fn bgfx_encoder_set_transform(encoder: *mut Encoder, mtx: *const c_void, num: u16) -> u32;
    fn bgfx_encoder_alloc_transform(encoder: *mut Encoder, transform: *mut Transform, num: u16) -> u32;
    fn bgfx_encoder_set_transform_cached(encoder: *mut Encoder, cache: u32, num: u16);
    fn bgfx_encoder_set_uniform(encoder: *mut Encoder, handle: UniformHandle, value: *const c_void, num: u16);
    fn bgfx_encoder_set_index_buffer(encoder: *mut Encoder, handle: IndexBufferHandle, firstIndex: u32, numIndices: u32);
    fn bgfx_encoder_set_dynamic_index_buffer(encoder: *mut Encoder, handle: DynamicIndexBufferHandle, firstIndex: u32, numIndices: u32);
    fn bgfx_encoder_set_transient_index_buffer(encoder: *mut Encoder, tib: *const TransientIndexBuffer, firstIndex: u32, numIndices: u32);
    fn bgfx_encoder_set_vertex_buffer(encoder: *mut Encoder, stream: u8, handle: VertexBufferHandle, startVertex: u32, numVertices: u32);
    fn bgfx_encoder_set_dynamic_vertex_buffer(encoder: *mut Encoder, stream: u8, handle: DynamicVertexBufferHandle, startVertex: u32, numVertices: u32);
    fn bgfx_encoder_set_transient_vertex_buffer(encoder: *mut Encoder, stream: u8, tvb: *const TransientVertexBuffer, startVertex: u32, numVertices: u32);
    fn bgfx_encoder_set_vertex_count(encoder: *mut Encoder, numVertices: u32);
    fn bgfx_encoder_set_instance_data_buffer(encoder: *mut Encoder, idb: *const InstanceDataBuffer, start: u32, num: u32);
    fn bgfx_encoder_set_instance_data_from_vertex_buffer(encoder: *mut Encoder, handle: VertexBufferHandle, startVertex: u32, num: u32);
    fn bgfx_encoder_set_instance_data_from_dynamic_vertex_buffer(encoder: *mut Encoder, handle: DynamicVertexBufferHandle, startVertex: u32, num: u32);
    fn bgfx_encoder_set_texture(encoder: *mut Encoder, stage: u8, sampler: UniformHandle, handle: TextureHandle, flags: u32);
    fn bgfx_encoder_touch(encoder: *mut Encoder, id: ViewId);
    fn bgfx_encoder_submit(encoder: *mut Encoder, id: ViewId, handle: ProgramHandle, depth: i32, preserveState: bool);
    fn bgfx_encoder_submit_occlusion_query(encoder: *mut Encoder, id: ViewId, program: ProgramHandle, occlusionQuery: OcclusionQueryHandle, depth: i32, preserveState: bool);
    fn bgfx_encoder_submit_indirect(encoder: *mut Encoder, id: ViewId, handle: ProgramHandle, indirectHandle: IndirectBufferHandle, start: u16, num: u16, depth: i32, preserveState: bool);
    fn bgfx_encoder_set_image(encoder: *mut Encoder, stage: u8, handle: TextureHandle, mip: u8, access: Access, format: TextureFormat);
    fn bgfx_encoder_set_compute_index_buffer(encoder: *mut Encoder, stage: u8, handle: IndexBufferHandle, access: Access);
    fn bgfx_encoder_set_compute_vertex_buffer(encoder: *mut Encoder, stage: u8, handle: VertexBufferHandle, access: Access);
    fn bgfx_encoder_set_compute_dynamic_index_buffer(encoder: *mut Encoder, stage: u8, handle: DynamicIndexBufferHandle, access: Access);
    fn bgfx_encoder_set_compute_dynamic_vertex_buffer(encoder: *mut Encoder, stage: u8, handle: DynamicVertexBufferHandle, access: Access);
    fn bgfx_encoder_set_compute_indirect_buffer(encoder: *mut Encoder, stage: u8, handle: IndirectBufferHandle, access: Access);
    fn bgfx_encoder_dispatch(encoder: *mut Encoder, id: ViewId, handle: ProgramHandle, numX: u32, numY: u32, numZ: u32, flags: u8);
    fn bgfx_encoder_dispatch_indirect(encoder: *mut Encoder, id: ViewId, handle: ProgramHandle, indirectHandle: IndirectBufferHandle, start: u16, num: u16, flags: u8);
    fn bgfx_encoder_discard(encoder: *mut Encoder);
    fn bgfx_encoder_blit(encoder: *mut Encoder, id: ViewId, dst: TextureHandle, dstMip: u8, dstX: u16, dstY: u16, dstZ: u16, src: TextureHandle, srcMip: u8, srcX: u16, srcY: u16, srcZ: u16, width: u16, height: u16, depth: u16);
    fn bgfx_request_screen_shot(handle: FrameBufferHandle, filePath: CString);
}

