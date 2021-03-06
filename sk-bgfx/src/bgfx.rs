#![allow(improper_ctypes)] // because VaArgs are platform-dependent

extern crate std;
extern crate libc;
extern crate va_list;

use self::libc::{c_char, c_void, c_float};
use self::va_list::VaList;
use std::ffi::{CString,CStr};
use std::mem::{uninitialized,transmute};

/// Sentinel to manage objects with a single-frame lifetime, Advances
/// to the next frame when dropped.
pub struct Frame {}

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
    Default
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

pub type DynamicIndexBufferHandleImpl = u16;
pub type DynamicVertexBufferHandleImpl = u16;
pub type FrameBufferHandleImpl = u16;
pub type IndexBufferHandleImpl = u16;
pub type IndirectBufferHandleImpl = u16;
pub type OcclusionQueryHandleImpl = u16;
pub type ProgramHandleImpl = u16;
pub type ShaderHandleImpl = u16;
pub type TextureHandleImpl = u16;
pub type UniformHandleImpl = u16;
pub type VertexBufferHandleImpl = u16;
pub type VertexDeclHandle = u16;

pub type ReleaseFn = extern fn(ptr: *mut c_void, user_data: *mut c_void);

pub struct TextureHandle2D { handle: TextureHandleImpl }
pub struct TextureHandle3D { handle: TextureHandleImpl }
pub struct TextureHandleCube { handle: TextureHandleImpl }

pub trait TextureHandle {
    fn set_name(&mut self, name: &str);
    unsafe fn expose_handle(&self) -> TextureHandleImpl;
}

pub struct FrameBufferHandle { handle: FrameBufferHandleImpl }
pub struct IndexBufferHandle { handle: IndexBufferHandleImpl }
pub struct VertexBufferHandle { handle: VertexBufferHandleImpl }
pub struct DynamicIndexBufferHandle { handle: DynamicIndexBufferHandleImpl }
pub struct DynamicVertexBufferHandle { handle: DynamicVertexBufferHandleImpl }
pub struct ShaderHandle { handle: ShaderHandleImpl }
pub struct UniformHandle { handle: UniformHandleImpl }
pub struct IndirectBufferHandle { handle: IndirectBufferHandleImpl }
pub struct OcclusionQueryHandle { handle: OcclusionQueryHandleImpl }
pub struct ProgramHandle { handle: ProgramHandleImpl }

#[repr(C)]
pub struct MemoryImpl{
    pub data: *mut u8,
    pub size: u32,
}

pub struct Memory{
    handle: *const MemoryImpl
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
    pub handle     : IndexBufferHandleImpl,
    pub start_index: u32,
}

#[repr(C)]
pub struct TransientVertexBuffer {
    pub data        : *mut u8,
    pub size        : u32,
    pub start_vertex: u32,
    pub stride      : u16,
    pub handle      : VertexBufferHandleImpl,
    pub decl        : VertexDeclHandle,
}

#[repr(C)]
pub struct InstanceDataBuffer {
    pub data  : *mut u8,
    pub size  : u32,
    pub offset: u32,
    pub num   : u32,
    pub stride: u16,
    pub handle: VertexBufferHandleImpl,
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
    pub handle: TextureHandleImpl,
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
                     str: *const c_char),
    trace_vargs: extern fn(this:*mut CallbackInterface,
                           filePath: *const c_char,
                           line: u16,
                           format: *const c_char,
                           argList: VaList),
    profiler_begin: extern fn(this:*mut CallbackInterface,
                              name: *const c_char,
                              abgr: u32,
                              filePath: *const c_char,
                              line: u16),
    profiler_begin_literal: extern fn(this:*mut CallbackInterface,
                                      name: *const c_char,
                                      abgr: u32,
                                      filePath: *const c_char,
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
                           filePath: *const c_char,
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
                           file: *const c_char,
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

pub type EncoderImpl = c_void;

/// A safe handle around a BGFX encoder instance.
pub struct Encoder {
    handle: *mut EncoderImpl
}

extern "C" {
    fn bgfx_vertex_decl_begin(decl: *mut VertexDecl, renderer: RendererKind);
    fn bgfx_vertex_decl_add(decl: *mut VertexDecl, attrib: Attrib, num: u8, kind: AttribKind, normalized: bool, asInt: bool);
    fn bgfx_vertex_decl_decode(decl: *const VertexDecl, attrib: *mut Attrib, num: *mut u8, kind: *mut AttribKind, normalized: *mut bool, asInt: *mut bool);
    fn bgfx_vertex_decl_has(decl: *const VertexDecl, attrib: Attrib) -> bool;
    fn bgfx_vertex_decl_skip(decl: *mut VertexDecl, num: u8);
    fn bgfx_vertex_decl_end(decl: *mut VertexDecl);
    fn bgfx_vertex_pack(input: [c_float; 4], inputNormalized: bool, attr: Attrib, decl: *const VertexDecl, data: *mut c_void, index: u32);
    fn bgfx_vertex_unpack(output: [c_float; 4], attr: Attrib, decl: *const VertexDecl, data: *const c_void, index: u32);
    fn bgfx_vertex_convert(destDecl: *const VertexDecl, destData: *mut c_void, srcDecl: *const VertexDecl, srcData: *const c_void, num: u32);
    fn bgfx_weld_vertices(output: *mut u16, decl: *const VertexDecl, data: *const c_void, num: u16, epsilon: c_float) -> u16;
    fn bgfx_topology_convert(conversion: TopologyConvert, dst: *mut c_void, dstSize: u32, indices: *const c_void, numIndices: u32, index32: bool) -> u32;
    fn bgfx_topology_sort_tri_list(sort: TopologySort, dst: *mut c_void, dstSize: u32, dir: [c_float; 3], pos: [c_float; 3], vertices: *const c_void, stride: u32, indices: *const c_void, numIndices: u32, index32: bool);
    fn bgfx_get_supported_renderers(max: u8, kind: *mut RendererKind) -> u8;
    fn bgfx_get_renderer_name(kind: RendererKind) -> *const c_char;
    fn bgfx_init_ctor(init: *mut Init);
    fn bgfx_init(init: *const Init) -> bool;
    fn bgfx_shutdown();
    fn bgfx_reset(width: u32, height: u32, flags: u32);
    fn bgfx_begin() -> *mut EncoderImpl;
    fn bgfx_end(encoder: *mut EncoderImpl);
    fn bgfx_frame(capture: bool) -> u32;
    fn bgfx_get_renderer_type() -> RendererKind;
    fn bgfx_get_caps() -> *const Caps;
    fn bgfx_get_stats() -> *const Stats;
    fn bgfx_alloc(size: u32) -> *const MemoryImpl;
    fn bgfx_copy(data: *const c_void, size: u32) -> *const MemoryImpl;
    fn bgfx_make_ref(data: *const c_void, size: u32) -> *const MemoryImpl;
    fn bgfx_make_ref_release(data: *const c_void, size: u32, release_fn: ReleaseFn, user_data: *mut c_void) -> *const MemoryImpl;
    fn bgfx_set_debug(debug: u32);
    fn bgfx_dbg_text_clear(attr: u8, small: bool);
    fn bgfx_dbg_text_printf(x: u16, y: u16, attr: u8, format: *const c_char, ...);
    fn bgfx_dbg_text_vprintf(x: u16, y: u16, attr: u8, format: *const c_char, argList: VaList);
    fn bgfx_dbg_text_image(x: u16, y: u16, width: u16, height: u16, data: *const c_void, pitch: u16);
    fn bgfx_create_index_buffer(mem: *const MemoryImpl, flags: u16) -> IndexBufferHandleImpl;
    fn bgfx_destroy_index_buffer(handle: IndexBufferHandleImpl);
    fn bgfx_create_vertex_buffer(mem: *const MemoryImpl, decl: *const VertexDecl, flags: u16) -> VertexBufferHandleImpl;
    fn bgfx_destroy_vertex_buffer(handle: VertexBufferHandleImpl);
    fn bgfx_create_dynamic_index_buffer(num: u32, flags: u16) -> DynamicIndexBufferHandleImpl;
    fn bgfx_create_dynamic_index_buffer_mem(mem: *const MemoryImpl, flags: u16) -> DynamicIndexBufferHandleImpl;
    fn bgfx_update_dynamic_index_buffer(handle: DynamicIndexBufferHandleImpl, startIndex: u32, mem: *const MemoryImpl);
    fn bgfx_destroy_dynamic_index_buffer(handle: DynamicIndexBufferHandleImpl);
    fn bgfx_create_dynamic_vertex_buffer(num: u32, decl: *const VertexDecl, flags: u16) -> DynamicVertexBufferHandleImpl;
    fn bgfx_create_dynamic_vertex_buffer_mem(mem: *const MemoryImpl, decl: *const VertexDecl, flags: u16) -> DynamicVertexBufferHandleImpl;
    fn bgfx_update_dynamic_vertex_buffer(handle: DynamicVertexBufferHandleImpl, startVertex: u32, mem: *const MemoryImpl);
    fn bgfx_destroy_dynamic_vertex_buffer(handle: DynamicVertexBufferHandleImpl);
    fn bgfx_get_avail_transient_index_buffer(num: u32) -> u32;
    fn bgfx_get_avail_transient_vertex_buffer(num: u32, decl: *const VertexDecl) -> u32;
    fn bgfx_get_avail_instance_data_buffer(num: u32, stride: u16) -> u32;
    fn bgfx_alloc_transient_index_buffer(tib: *mut TransientIndexBuffer, num: u32);
    fn bgfx_alloc_transient_vertex_buffer(tvb: *mut TransientVertexBuffer, num: u32, decl: *const VertexDecl);
    fn bgfx_alloc_transient_buffers(tvb: *mut TransientVertexBuffer, decl: *const VertexDecl, numVertices: u32, tib: *mut TransientIndexBuffer, numIndices: u32) -> bool;
    fn bgfx_alloc_instance_data_buffer(idb: *mut InstanceDataBuffer, num: u32, stride: u16);
    fn bgfx_create_indirect_buffer(num: u32) -> IndirectBufferHandleImpl;
    fn bgfx_destroy_indirect_buffer(handle: IndirectBufferHandleImpl);
    fn bgfx_create_shader(mem: *const MemoryImpl) -> ShaderHandleImpl;
    fn bgfx_get_shader_uniforms(handle: ShaderHandleImpl, uniforms: *mut UniformHandleImpl, max: u16) -> u16;
    fn bgfx_get_uniform_info(handle: UniformHandleImpl, info: *mut UniformInfo);
    fn bgfx_set_shader_name(handle: ShaderHandleImpl, name: *const c_char, len: i32);
    fn bgfx_destroy_shader(handle: ShaderHandleImpl);
    fn bgfx_create_program(vsh: ShaderHandleImpl, fsh: ShaderHandleImpl, destroyShaders: bool) -> ProgramHandleImpl;
    fn bgfx_create_compute_program(csh: ShaderHandleImpl, destroyShaders: bool) -> ProgramHandleImpl;
    fn bgfx_destroy_program(handle: ProgramHandleImpl);
    fn bgfx_is_texture_valid(depth: u16, cube_map: bool, num_layers: u16, format: TextureFormat, flags: u32) -> bool;
    fn bgfx_calc_texture_size(info: *mut TextureInfo, width: u16, height: u16, depth: u16, cube_map: bool, has_mips: bool, num_layers: u16, format: TextureFormat);
    fn bgfx_create_texture(mem: *const MemoryImpl, flags: u32, skip: u8, info: *mut TextureInfo) -> TextureHandleImpl;
    fn bgfx_create_texture_2d(width: u16, height: u16, has_mips: bool, num_layers: u16, format: TextureFormat, flags: u32, mem: *const MemoryImpl) -> TextureHandleImpl;
    fn bgfx_create_texture_2d_scaled(ratio: BackbufferRatio, has_mips: bool, num_layers: u16, format: TextureFormat, flags: u32) -> TextureHandleImpl;
    fn bgfx_create_texture_3d(width: u16, height: u16, depth: u16, has_mips: bool, format: TextureFormat, flags: u32, mem: *const MemoryImpl) -> TextureHandleImpl;
    fn bgfx_create_texture_cube(size: u16, has_mips: bool, num_layers: u16, format: TextureFormat, flags: u32, mem: *const MemoryImpl) -> TextureHandleImpl;
    fn bgfx_update_texture_2d(handle: TextureHandleImpl, layer: u16, mip: u8, x: u16, y: u16, width: u16, height: u16, mem: *const MemoryImpl, pitch: u16);
    fn bgfx_update_texture_3d(handle: TextureHandleImpl, mip: u8, x: u16, y: u16, z: u16, width: u16, height: u16, depth: u16, mem: *const MemoryImpl);
    fn bgfx_update_texture_cube(handle: TextureHandleImpl, layer: u16, side: u8, mip: u8, x: u16, y: u16, width: u16, height: u16, mem: *const MemoryImpl, pitch: u16);
    fn bgfx_read_texture(handle: TextureHandleImpl, data: *mut c_void, mip: u8) -> u32;
    fn bgfx_set_texture_name(handle: TextureHandleImpl, name: *const c_char, len: i32);
    fn bgfx_destroy_texture(handle: TextureHandleImpl);
    fn bgfx_create_frame_buffer(width: u16, height: u16, format: TextureFormat, texture_flags: u32) -> FrameBufferHandleImpl;
    fn bgfx_create_frame_buffer_scaled(ratio: BackbufferRatio, format: TextureFormat, texture_flags: u32) -> FrameBufferHandleImpl;
    fn bgfx_create_frame_buffer_from_handles(num: u8, handles: *const TextureHandleImpl, destroyTextures: bool) -> FrameBufferHandleImpl;
    fn bgfx_create_frame_buffer_from_attachment(num: u8, attachment: *const Attachment, destroyTextures: bool) -> FrameBufferHandleImpl;
    fn bgfx_create_frame_buffer_from_nwh(nwh: *mut c_void, width: u16, height: u16, depth_format: TextureFormat) -> FrameBufferHandleImpl;
    fn bgfx_get_texture(handle: FrameBufferHandleImpl, attachment: u8) -> TextureHandleImpl;
    fn bgfx_destroy_frame_buffer(handle: FrameBufferHandleImpl);
    fn bgfx_create_uniform(name: *const c_char, kind: UniformKind, num: u16) -> UniformHandleImpl;
    fn bgfx_destroy_uniform(handle: UniformHandleImpl);
    fn bgfx_create_occlusion_query() -> OcclusionQueryHandleImpl;
    fn bgfx_get_result(handle: OcclusionQueryHandleImpl, result: *mut i32) -> OcclusionQueryResult;
    fn bgfx_destroy_occlusion_query(handle: OcclusionQueryHandleImpl);
    fn bgfx_set_palette_color(index: u8, rgba: [c_float; 4]);
    fn bgfx_set_view_name(id: ViewId, name: *const c_char);
    fn bgfx_set_view_rect(id: ViewId, x: u16, y: u16, width: u16, height: u16);
    fn bgfx_set_view_rect_auto(id: ViewId, x: u16, y: u16, ratio: BackbufferRatio);
    fn bgfx_set_view_scissor(id: ViewId, x: u16, y: u16, width: u16, height: u16);
    fn bgfx_set_view_clear(id: ViewId, flags: u16, rgba: u32, depth: c_float, stencil: u8);
    fn bgfx_set_view_clear_mrt(id: ViewId, flags: u16, depth: c_float, stencil: u8, param0: u8, param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8);
    fn bgfx_set_view_mode(id: ViewId, mode: ViewMode);
    fn bgfx_set_view_frame_buffer(id: ViewId, handle: FrameBufferHandleImpl);
    fn bgfx_set_view_transform(id: ViewId, view: *const c_void, proj: *const c_void);
    fn bgfx_set_view_transform_stereo(id: ViewId, view: *const c_void, projL: *const c_void, flags: u8, projR: *const c_void);
    fn bgfx_set_view_order(id: ViewId, num: u16, order: *const ViewId);
    fn bgfx_reset_view(id: ViewId);
    fn bgfx_set_marker(marker: *const c_char);
    fn bgfx_set_state(state: u64, rgba: u32);
    fn bgfx_set_condition(handle: OcclusionQueryHandleImpl, visible: bool);
    fn bgfx_set_stencil(fstencil: u32, bstencil: u32);
    fn bgfx_set_scissor(x: u16, y: u16, width: u16, height: u16) -> u16;
    fn bgfx_set_scissor_cached(cache: u16);
    fn bgfx_set_transform(mtx: *const c_void, num: u16) -> u32;
    fn bgfx_alloc_transform(transform: *mut Transform, num: u16) -> u32;
    fn bgfx_set_transform_cached(cache: u32, num: u16);
    fn bgfx_set_uniform(handle: UniformHandleImpl, value: *const c_void, num: u16);
    fn bgfx_set_index_buffer(handle: IndexBufferHandleImpl, firstIndex: u32, numIndices: u32);
    fn bgfx_set_dynamic_index_buffer(handle: DynamicIndexBufferHandleImpl, firstIndex: u32, numIndices: u32);
    fn bgfx_set_transient_index_buffer(tib: *const TransientIndexBuffer, firstIndex: u32, numIndices: u32);
    fn bgfx_set_vertex_buffer(stream: u8, handle: VertexBufferHandleImpl, startVertex: u32, numVertices: u32);
    fn bgfx_set_dynamic_vertex_buffer(stream: u8, handle: DynamicVertexBufferHandleImpl, startVertex: u32, numVertices: u32);
    fn bgfx_set_transient_vertex_buffer(stream: u8, tvb: *const TransientVertexBuffer, startVertex: u32, numVertices: u32);
    fn bgfx_set_vertex_count(numVertices: u32);
    fn bgfx_set_instance_data_buffer(idb: *const InstanceDataBuffer, start: u32, num: u32);
    fn bgfx_set_instance_data_from_vertex_buffer(handle: VertexBufferHandleImpl, startVertex: u32, num: u32);
    fn bgfx_set_instance_data_from_dynamic_vertex_buffer(handle: DynamicVertexBufferHandleImpl, startVertex: u32, num: u32);
    fn bgfx_set_texture(stage: u8, sampler: UniformHandleImpl, handle: TextureHandleImpl, flags: u32);
    fn bgfx_touch(id: ViewId);
    fn bgfx_submit(id: ViewId, handle: ProgramHandleImpl, depth: i32, preserveState: bool);
    fn bgfx_submit_occlusion_query(id: ViewId, program: ProgramHandleImpl, occlusionQuery: OcclusionQueryHandleImpl, depth: i32, preserveState: bool);
    fn bgfx_submit_indirect(id: ViewId, handle: ProgramHandleImpl, indirectHandle: IndirectBufferHandleImpl, start: u16, num: u16, depth: i32, preserveState: bool);
    fn bgfx_set_image(stage: u8, handle: TextureHandleImpl, mip: u8, access: Access, format: TextureFormat);
    fn bgfx_set_compute_index_buffer(stage: u8, handle: IndexBufferHandleImpl, access: Access);
    fn bgfx_set_compute_vertex_buffer(stage: u8, handle: VertexBufferHandleImpl, access: Access);
    fn bgfx_set_compute_dynamic_index_buffer(stage: u8, handle: DynamicIndexBufferHandleImpl, access: Access);
    fn bgfx_set_compute_dynamic_vertex_buffer(stage: u8, handle: DynamicVertexBufferHandleImpl, access: Access);
    fn bgfx_set_compute_indirect_buffer(stage: u8, handle: IndirectBufferHandleImpl, access: Access);
    fn bgfx_dispatch(id: ViewId, handle: ProgramHandleImpl, numX: u32, numY: u32, numZ: u32, flags: u8);
    fn bgfx_dispatch_indirect(id: ViewId, handle: ProgramHandleImpl, indirectHandle: IndirectBufferHandleImpl, start: u16, num: u16, flags: u8);
    fn bgfx_discard();
    fn bgfx_blit(id: ViewId, dst: TextureHandleImpl, dstMip: u8, dstX: u16, dstY: u16, dstZ: u16, src: TextureHandleImpl, srcMip: u8, srcX: u16, srcY: u16, srcZ: u16, width: u16, height: u16, depth: u16);
    fn bgfx_encoder_set_marker(encoder: *mut EncoderImpl, marker: *const c_char);
    fn bgfx_encoder_set_state(encoder: *mut EncoderImpl, state: u64, rgba: u32);
    fn bgfx_encoder_set_condition(encoder: *mut EncoderImpl, handle: OcclusionQueryHandleImpl, visible: bool);
    fn bgfx_encoder_set_stencil(encoder: *mut EncoderImpl, fstencil: u32, bstencil: u32);
    fn bgfx_encoder_set_scissor(encoder: *mut EncoderImpl, x: u16, y: u16, width: u16, height: u16) -> u16;
    fn bgfx_encoder_set_scissor_cached(encoder: *mut EncoderImpl, cache: u16);
    fn bgfx_encoder_set_transform(encoder: *mut EncoderImpl, mtx: *const c_void, num: u16) -> u32;
    fn bgfx_encoder_alloc_transform(encoder: *mut EncoderImpl, transform: *mut Transform, num: u16) -> u32;
    fn bgfx_encoder_set_transform_cached(encoder: *mut EncoderImpl, cache: u32, num: u16);
    fn bgfx_encoder_set_uniform(encoder: *mut EncoderImpl, handle: UniformHandleImpl, value: *const c_void, num: u16);
    fn bgfx_encoder_set_index_buffer(encoder: *mut EncoderImpl, handle: IndexBufferHandleImpl, firstIndex: u32, numIndices: u32);
    fn bgfx_encoder_set_dynamic_index_buffer(encoder: *mut EncoderImpl, handle: DynamicIndexBufferHandleImpl, firstIndex: u32, numIndices: u32);
    fn bgfx_encoder_set_transient_index_buffer(encoder: *mut EncoderImpl, tib: *const TransientIndexBuffer, firstIndex: u32, numIndices: u32);
    fn bgfx_encoder_set_vertex_buffer(encoder: *mut EncoderImpl, stream: u8, handle: VertexBufferHandleImpl, startVertex: u32, numVertices: u32);
    fn bgfx_encoder_set_dynamic_vertex_buffer(encoder: *mut EncoderImpl, stream: u8, handle: DynamicVertexBufferHandleImpl, startVertex: u32, numVertices: u32);
    fn bgfx_encoder_set_transient_vertex_buffer(encoder: *mut EncoderImpl, stream: u8, tvb: *const TransientVertexBuffer, startVertex: u32, numVertices: u32);
    fn bgfx_encoder_set_vertex_count(encoder: *mut EncoderImpl, numVertices: u32);
    fn bgfx_encoder_set_instance_data_buffer(encoder: *mut EncoderImpl, idb: *const InstanceDataBuffer, start: u32, num: u32);
    fn bgfx_encoder_set_instance_data_from_vertex_buffer(encoder: *mut EncoderImpl, handle: VertexBufferHandleImpl, startVertex: u32, num: u32);
    fn bgfx_encoder_set_instance_data_from_dynamic_vertex_buffer(encoder: *mut EncoderImpl, handle: DynamicVertexBufferHandleImpl, startVertex: u32, num: u32);
    fn bgfx_encoder_set_texture(encoder: *mut EncoderImpl, stage: u8, sampler: UniformHandleImpl, handle: TextureHandleImpl, flags: u32);
    fn bgfx_encoder_touch(encoder: *mut EncoderImpl, id: ViewId);
    fn bgfx_encoder_submit(encoder: *mut EncoderImpl, id: ViewId, handle: ProgramHandleImpl, depth: i32, preserveState: bool);
    fn bgfx_encoder_submit_occlusion_query(encoder: *mut EncoderImpl, id: ViewId, program: ProgramHandleImpl, occlusionQuery: OcclusionQueryHandleImpl, depth: i32, preserveState: bool);
    fn bgfx_encoder_submit_indirect(encoder: *mut EncoderImpl, id: ViewId, handle: ProgramHandleImpl, indirectHandle: IndirectBufferHandleImpl, start: u16, num: u16, depth: i32, preserveState: bool);
    fn bgfx_encoder_set_image(encoder: *mut EncoderImpl, stage: u8, handle: TextureHandleImpl, mip: u8, access: Access, format: TextureFormat);
    fn bgfx_encoder_set_compute_index_buffer(encoder: *mut EncoderImpl, stage: u8, handle: IndexBufferHandleImpl, access: Access);
    fn bgfx_encoder_set_compute_vertex_buffer(encoder: *mut EncoderImpl, stage: u8, handle: VertexBufferHandleImpl, access: Access);
    fn bgfx_encoder_set_compute_dynamic_index_buffer(encoder: *mut EncoderImpl, stage: u8, handle: DynamicIndexBufferHandleImpl, access: Access);
    fn bgfx_encoder_set_compute_dynamic_vertex_buffer(encoder: *mut EncoderImpl, stage: u8, handle: DynamicVertexBufferHandleImpl, access: Access);
    fn bgfx_encoder_set_compute_indirect_buffer(encoder: *mut EncoderImpl, stage: u8, handle: IndirectBufferHandleImpl, access: Access);
    fn bgfx_encoder_dispatch(encoder: *mut EncoderImpl, id: ViewId, handle: ProgramHandleImpl, numX: u32, numY: u32, numZ: u32, flags: u8);
    fn bgfx_encoder_dispatch_indirect(encoder: *mut EncoderImpl, id: ViewId, handle: ProgramHandleImpl, indirectHandle: IndirectBufferHandleImpl, start: u16, num: u16, flags: u8);
    fn bgfx_encoder_discard(encoder: *mut EncoderImpl);
    fn bgfx_encoder_blit(encoder: *mut EncoderImpl, id: ViewId, dst: TextureHandleImpl, dstMip: u8, dstX: u16, dstY: u16, dstZ: u16, src: TextureHandleImpl, srcMip: u8, srcX: u16, srcY: u16, srcZ: u16, width: u16, height: u16, depth: u16);
    fn bgfx_request_screen_shot(handle: FrameBufferHandleImpl, filePath: *const c_char);
}

impl Init {
    pub fn with_defaults() -> Init {
        unsafe {
            // bgfx includes its own ctor for this
            let mut result: Init = uninitialized();
            result.ctor();
            result
        }
    }

    pub fn ctor(&mut self) {
        unsafe { bgfx_init_ctor(self); }
    }
}

pub fn init(init: &Init) -> bool {
    unsafe { return bgfx_init(init); }

}

pub fn shutdown() {
    unsafe { bgfx_shutdown(); }
}

impl VertexDecl {
    pub fn begin(&mut self, renderer: RendererKind) {
        unsafe { bgfx_vertex_decl_begin(self, renderer); }
    }

    pub fn add(&mut self, attrib: Attrib, num: u8, kind: AttribKind, normalized: bool, as_int: bool) {
        unsafe { bgfx_vertex_decl_add(self, attrib, num, kind, normalized, as_int); }
    }

    pub fn decode(&self, attrib: &mut Attrib, num: &mut u8, kind: &mut AttribKind, normalized: &mut bool, as_int: &mut bool) {
        unsafe { bgfx_vertex_decl_decode(self, attrib, num, kind, normalized, as_int); }
    }

    pub fn has(&self, attrib: Attrib) -> bool { 
        unsafe { return bgfx_vertex_decl_has(self, attrib); }
    }

    pub fn skip(&mut self, num: u8) {
        unsafe { bgfx_vertex_decl_skip(self, num); }
    }

    pub fn end(&mut self) { 
        unsafe { bgfx_vertex_decl_end(self); }
    }
}

impl Encoder {
    pub fn begin() -> Encoder {
        unsafe { return Encoder{handle: bgfx_begin()} }
    }

    pub fn set_marker(&mut self, marker: &CStr) {
        let ptr = marker.as_ptr();
        unsafe { bgfx_encoder_set_marker(self.handle, ptr); }
    }

    pub fn set_state(&mut self, state: u64, rgba: u32) {
        unsafe { bgfx_encoder_set_state(self.handle, state, rgba); }
    }

    pub fn set_condition(&mut self, handle: OcclusionQueryHandle, visible: bool) {
        unsafe { bgfx_encoder_set_condition(self.handle, handle.handle, visible); }
    }

    pub fn set_stencil(&mut self, fstencil: u32, bstencil: u32) {
        unsafe { bgfx_encoder_set_stencil(self.handle, fstencil, bstencil); }
    }

    pub fn set_scissor(&mut self, x: u16, y: u16, width: u16, height: u16) -> u16 {
        unsafe { return bgfx_encoder_set_scissor(self.handle, x, y, width, height); }
    }

    pub fn set_scissor_cached(&mut self, cache: u16) {
        unsafe { bgfx_encoder_set_scissor_cached(self.handle, cache); }
    }

    pub fn set_transform(&mut self, mtx: &c_void, num: u16) -> u32 {
        unsafe { return bgfx_encoder_set_transform(self.handle, mtx, num); }
    }

    pub fn alloc_transform(&mut self, transform: *mut Transform, num: u16) -> u32 {
        unsafe { return bgfx_encoder_alloc_transform(self.handle, transform, num); }
    }

    pub fn set_transform_cached(&mut self, cache: u32, num: u16) {
        unsafe { bgfx_encoder_set_transform_cached(self.handle, cache, num); }
    }

    pub fn set_uniform(&mut self, handle: UniformHandle, value: &c_void, num: u16) {
        unsafe { bgfx_encoder_set_uniform(self.handle, handle.handle, value, num); }
    }

    pub fn set_index_buffer(&mut self, handle: IndexBufferHandle, first_index: u32, num_indices: u32) {
        unsafe { bgfx_encoder_set_index_buffer(self.handle, handle.handle, first_index, num_indices); }
    }

    pub fn set_dynamic_index_buffer(&mut self, handle: DynamicIndexBufferHandle, first_index: u32, num_indices: u32) {
        unsafe { bgfx_encoder_set_dynamic_index_buffer(self.handle, handle.handle, first_index, num_indices); }
    }

    pub fn set_transient_index_buffer(&mut self, tib: &TransientIndexBuffer, first_index: u32, num_indices: u32) {
        unsafe { bgfx_encoder_set_transient_index_buffer(self.handle, tib, first_index, num_indices); }
    }

    pub fn set_vertex_buffer(&mut self, stream: u8, handle: VertexBufferHandle, start_vertex: u32, num_vertices: u32) {
        unsafe { bgfx_encoder_set_vertex_buffer(self.handle, stream, handle.handle, start_vertex, num_vertices); }
    }

    pub fn set_dynamic_vertex_buffer(&mut self, stream: u8, handle: DynamicVertexBufferHandle, start_vertex: u32, num_vertices: u32) {
        unsafe { bgfx_encoder_set_dynamic_vertex_buffer(self.handle, stream, handle.handle, start_vertex, num_vertices); }
    }

    pub fn set_transient_vertex_buffer(&mut self, stream: u8, tvb: &TransientVertexBuffer, start_vertex: u32, num_vertices: u32) {
        unsafe { bgfx_encoder_set_transient_vertex_buffer(self.handle, stream, tvb, start_vertex, num_vertices); }
    }

    pub fn set_vertex_count(&mut self, num_vertices: u32) {
        unsafe { bgfx_encoder_set_vertex_count(self.handle, num_vertices); }
    }

    pub fn set_instance_data_buffer(&mut self, idb: &InstanceDataBuffer, start: u32, num: u32) {
        unsafe { bgfx_encoder_set_instance_data_buffer(self.handle, idb, start, num); }
    }

    pub fn set_instance_data_from_vertex_buffer(&mut self, handle: VertexBufferHandle, start_vertex: u32, num: u32) {
        unsafe { bgfx_encoder_set_instance_data_from_vertex_buffer(self.handle, handle.handle, start_vertex, num); }
    }

    pub fn set_instance_data_from_dynamic_vertex_buffer(&mut self, handle: DynamicVertexBufferHandle, start_vertex: u32, num: u32) {
        unsafe { bgfx_encoder_set_instance_data_from_dynamic_vertex_buffer(self.handle, handle.handle, start_vertex, num); }
    }

    pub fn set_texture(&mut self, stage: u8, sampler: UniformHandle, handle: &TextureHandle, flags: u32) {
        unsafe { bgfx_encoder_set_texture(self.handle, stage, sampler.handle, handle.expose_handle(), flags); }
    }

    pub fn touch(&mut self, id: ViewId) {
        unsafe { bgfx_encoder_touch(self.handle, id); }
    }

    pub fn submit(&mut self, id: ViewId, handle: ProgramHandle, depth: i32, preserve_state: bool) {
        unsafe { bgfx_encoder_submit(self.handle, id, handle.handle, depth, preserve_state); }
    }

    pub fn submit_occlusion_query(&mut self, id: ViewId, program: ProgramHandle, occlusion_query: OcclusionQueryHandle, depth: i32, preserve_state: bool) {
        unsafe { bgfx_encoder_submit_occlusion_query(self.handle, id, program.handle, occlusion_query.handle, depth, preserve_state); }
    }

    pub fn submit_indirect(&mut self, id: ViewId, handle: ProgramHandle, indirect_handle: IndirectBufferHandle, start: u16, num: u16, depth: i32, preserve_state: bool) {
        unsafe { bgfx_encoder_submit_indirect(self.handle, id, handle.handle, indirect_handle.handle, start, num, depth, preserve_state); }
    }

    pub fn set_image(&mut self, stage: u8, handle: &TextureHandle, mip: u8, access: Access, format: TextureFormat) {
        unsafe { bgfx_encoder_set_image(self.handle, stage, handle.expose_handle(), mip, access, format); }
    }

    pub fn set_compute_index_buffer(&mut self, stage: u8, handle: IndexBufferHandle, access: Access) {
        unsafe { bgfx_encoder_set_compute_index_buffer(self.handle, stage, handle.handle, access); }
    }

    pub fn set_compute_vertex_buffer(&mut self, stage: u8, handle: VertexBufferHandle, access: Access) {
        unsafe { bgfx_encoder_set_compute_vertex_buffer(self.handle, stage, handle.handle, access); }
    }

    pub fn set_compute_dynamic_index_buffer(&mut self, stage: u8, handle: DynamicIndexBufferHandle, access: Access) {
        unsafe { bgfx_encoder_set_compute_dynamic_index_buffer(self.handle, stage, handle.handle, access); }
    }

    pub fn set_compute_dynamic_vertex_buffer(&mut self, stage: u8, handle: DynamicVertexBufferHandle, access: Access) {
        unsafe { bgfx_encoder_set_compute_dynamic_vertex_buffer(self.handle, stage, handle.handle, access); }
    }

    pub fn set_compute_indirect_buffer(&mut self, stage: u8, handle: IndirectBufferHandle, access: Access) {
        unsafe { bgfx_encoder_set_compute_indirect_buffer(self.handle, stage, handle.handle, access); }
    }

    pub fn dispatch(&mut self, id: ViewId, handle: ProgramHandle, num_x: u32, num_y: u32, num_z: u32, flags: u8) {
        unsafe { bgfx_encoder_dispatch(self.handle, id, handle.handle, num_x, num_y, num_z, flags); }
    }

    pub fn dispatch_indirect(&mut self, id: ViewId, handle: ProgramHandle, indirect_handle: IndirectBufferHandle, start: u16, num: u16, flags: u8) {
        unsafe { bgfx_encoder_dispatch_indirect(self.handle, id, handle.handle, indirect_handle.handle, start, num, flags); }
    }

    pub fn discard(&mut self) {
        unsafe { bgfx_encoder_discard(self.handle); }
    }

    pub fn blit(&mut self, id: ViewId, dst: &TextureHandle, dst_mip: u8, dst_x: u16, dst_y: u16, dst_z: u16, src: &TextureHandle, src_mip: u8, src_x: u16, src_y: u16, src_z: u16, width: u16, height: u16, depth: u16) {
        unsafe { bgfx_encoder_blit(self.handle, id, dst.expose_handle(), dst_mip, dst_x, dst_y, dst_z, src.expose_handle(), src_mip, src_x, src_y, src_z, width, height, depth); }
    }

}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { bgfx_end(self.handle); }
    }
}

pub fn submit(id: ViewId, handle: ProgramHandle, depth: i32, preserve_state: bool) {
    unsafe { bgfx_submit(id, handle.handle, depth, preserve_state); }
}

pub fn submit_occlusion_query(id: ViewId, program: ProgramHandle, occlusion_query: OcclusionQueryHandle, depth: i32, preserve_state: bool) {
    unsafe { bgfx_submit_occlusion_query(id, program.handle, occlusion_query.handle, depth, preserve_state); }
}

pub fn submit_indirect(id: ViewId, handle: ProgramHandle, indirect_handle: IndirectBufferHandle, start: u16, num: u16, depth: i32, preserve_state: bool) {
    unsafe { bgfx_submit_indirect(id, handle.handle, indirect_handle.handle, start, num, depth, preserve_state); }
}

pub fn set_image(stage: u8, handle: &TextureHandle, mip: u8, access: Access, format: TextureFormat) {
    unsafe { bgfx_set_image(stage, handle.expose_handle(), mip, access, format); }
}

pub fn set_compute_index_buffer(stage: u8, handle: IndexBufferHandle, access: Access) {
    unsafe { bgfx_set_compute_index_buffer(stage, handle.handle, access); }
}

pub fn set_compute_vertex_buffer(stage: u8, handle: VertexBufferHandle, access: Access) {
    unsafe { bgfx_set_compute_vertex_buffer(stage, handle.handle, access); }
}

pub fn set_compute_dynamic_index_buffer(stage: u8, handle: DynamicIndexBufferHandle, access: Access) {
    unsafe { bgfx_set_compute_dynamic_index_buffer(stage, handle.handle, access); }
}

pub fn set_compute_dynamic_vertex_buffer(stage: u8, handle: DynamicVertexBufferHandle, access: Access) {
    unsafe { bgfx_set_compute_dynamic_vertex_buffer(stage, handle.handle, access); }
}

pub fn set_compute_indirect_buffer(stage: u8, handle: IndirectBufferHandle, access: Access) {
    unsafe { bgfx_set_compute_indirect_buffer(stage, handle.handle, access); }
}

pub fn dispatch(id: ViewId, handle: ProgramHandle, num_x: u32, num_y: u32, num_z: u32, flags: u8) {
    unsafe { bgfx_dispatch(id, handle.handle, num_x, num_y, num_z, flags); }
}

pub fn dispatch_indirect(id: ViewId, handle: ProgramHandle, indirect_handle: IndirectBufferHandle, start: u16, num: u16, flags: u8) {
    unsafe { bgfx_dispatch_indirect(id, handle.handle, indirect_handle.handle, start, num, flags); }
}

pub fn discard() {
    unsafe { bgfx_discard(); }
}

pub fn blit(id: ViewId, dst: &TextureHandle, dst_mip: u8, dst_x: u16, dst_y: u16, dst_z: u16, src: &TextureHandle, src_mip: u8, src_x: u16, src_y: u16, src_z: u16, width: u16, height: u16, depth: u16) {
    unsafe { bgfx_blit(id, dst.expose_handle(), dst_mip, dst_x, dst_y, dst_z, src.expose_handle(), src_mip, src_x, src_y, src_z, width, height, depth); }
}

pub fn request_screen_shot_cstr(handle: FrameBufferHandle, file_path: &CStr) {
    let ptr = file_path.as_ptr();
    unsafe { bgfx_request_screen_shot(handle.handle, ptr); }
}

pub fn reset_view(id: ViewId) {
    unsafe { bgfx_reset_view(id); }
}

pub fn set_marker(marker: &CStr) {
    let ptr = marker.as_ptr();
    unsafe { bgfx_set_marker(ptr); }
}

pub fn set_state(state: u64, rgba: u32) {
    unsafe { bgfx_set_state(state, rgba); }
}

pub fn set_condition(handle: OcclusionQueryHandle, visible: bool) {
    unsafe { bgfx_set_condition(handle.handle, visible); }
}

pub fn set_stencil(fstencil: u32, bstencil: u32) {
    unsafe { bgfx_set_stencil(fstencil, bstencil); }
}

pub fn set_scissor(x: u16, y: u16, width: u16, height: u16) -> u16 {
    unsafe { return bgfx_set_scissor(x, y, width, height); }
}

pub fn set_scissor_cached(cache: u16) {
    unsafe { bgfx_set_scissor_cached(cache); }
}

pub fn set_transform(mtx: &c_void, num: u16) -> u32 {
    unsafe { return bgfx_set_transform(mtx, num); }
}

pub fn alloc_transform(transform: *mut Transform, num: u16) -> u32 {
    unsafe { return bgfx_alloc_transform(transform, num); }
}

pub fn set_transform_cached(cache: u32, num: u16) {
    unsafe { bgfx_set_transform_cached(cache, num); }
}

pub fn set_uniform(handle: UniformHandle, value: &c_void, num: u16) {
    unsafe { bgfx_set_uniform(handle.handle, value, num); }
}

pub fn set_index_buffer(handle: IndexBufferHandle, first_index: u32, num_indices: u32) {
    unsafe { bgfx_set_index_buffer(handle.handle, first_index, num_indices); }
}

pub fn set_dynamic_index_buffer(handle: DynamicIndexBufferHandle, first_index: u32, num_indices: u32) {
    unsafe { bgfx_set_dynamic_index_buffer(handle.handle, first_index, num_indices); }
}

pub fn set_transient_index_buffer(tib: &TransientIndexBuffer, first_index: u32, num_indices: u32) {
    unsafe { bgfx_set_transient_index_buffer(tib, first_index, num_indices); }
}

pub fn set_vertex_buffer(stream: u8, handle: VertexBufferHandle, start_vertex: u32, num_vertices: u32) {
    unsafe { bgfx_set_vertex_buffer(stream, handle.handle, start_vertex, num_vertices); }
}

pub fn set_dynamic_vertex_buffer(stream: u8, handle: DynamicVertexBufferHandle, start_vertex: u32, num_vertices: u32) {
    unsafe { bgfx_set_dynamic_vertex_buffer(stream, handle.handle, start_vertex, num_vertices); }
}

pub fn set_transient_vertex_buffer(stream: u8, tvb: &TransientVertexBuffer, start_vertex: u32, num_vertices: u32) {
    unsafe { bgfx_set_transient_vertex_buffer(stream, tvb, start_vertex, num_vertices); }
}

pub fn set_vertex_count(num_vertices: u32) {
    unsafe { bgfx_set_vertex_count(num_vertices); }
}

pub fn set_instance_data_buffer(idb: &InstanceDataBuffer, start: u32, num: u32) {
    unsafe { bgfx_set_instance_data_buffer(idb, start, num); }
}

pub fn set_instance_data_from_vertex_buffer(handle: VertexBufferHandle, start_vertex: u32, num: u32) {
    unsafe { bgfx_set_instance_data_from_vertex_buffer(handle.handle, start_vertex, num); }
}

pub fn set_instance_data_from_dynamic_vertex_buffer(handle: DynamicVertexBufferHandle, start_vertex: u32, num: u32) {
    unsafe { bgfx_set_instance_data_from_dynamic_vertex_buffer(handle.handle, start_vertex, num); }
}

pub fn set_texture(stage: u8, sampler: UniformHandle, handle: &TextureHandle, flags: u32) {
    unsafe { bgfx_set_texture(stage, sampler.handle, handle.expose_handle(), flags); }
}

pub fn touch(id: ViewId) {
    unsafe { bgfx_touch(id); }
}

pub fn set_palette_color(index: u8, rgba: [c_float; 4]) {
    unsafe { bgfx_set_palette_color(index, rgba); }
}

pub fn set_view_name(id: ViewId, name: &CStr) {
    let ptr = name.as_ptr();
    unsafe { bgfx_set_view_name(id, ptr); }
}

pub fn set_view_rect(id: ViewId, x: u16, y: u16, width: u16, height: u16) {
    unsafe { bgfx_set_view_rect(id, x, y, width, height); }
}

pub fn set_view_rect_auto(id: ViewId, x: u16, y: u16, ratio: BackbufferRatio) {
    unsafe { bgfx_set_view_rect_auto(id, x, y, ratio); }
}

pub fn set_view_scissor(id: ViewId, x: u16, y: u16, width: u16, height: u16) {
    unsafe { bgfx_set_view_scissor(id, x, y, width, height); }
}

pub fn set_view_clear(id: ViewId, flags: u16, rgba: u32, depth: c_float, stencil: u8) {
    unsafe { bgfx_set_view_clear(id, flags, rgba, depth, stencil); }
}

pub fn set_view_clear_mrt(id: ViewId, flags: u16, depth: c_float, stencil: u8, param0: u8, param1: u8, param2: u8, param3: u8, param4: u8, param5: u8, param6: u8, param7: u8) {
    unsafe { bgfx_set_view_clear_mrt(id, flags, depth, stencil, param0, param1, param2, param3, param4, param5, param6, param7); }
}

pub fn set_view_mode(id: ViewId, mode: ViewMode) {
    unsafe { bgfx_set_view_mode(id, mode); }
}

pub fn set_view_frame_buffer(id: ViewId, handle: FrameBufferHandle) {
    unsafe { bgfx_set_view_frame_buffer(id, handle.handle); }
}

/* TODO rust api
pub fn set_view_transform(id: ViewId, view: *c_void, proj: *c_void) {
    unsafe { bgfx_set_view_transform(id, view, proj); }
}

pub fn set_view_transform_stereo(id: ViewId, view: *c_void, projL: *c_void, flags: u8, projR: *c_void) {
    unsafe { bgfx_set_view_transform_stereo(id, view, projL, flags, projR); }
}

pub fn set_view_order(id: ViewId, num: u16, order: *ViewId) {
    unsafe { bgfx_set_view_order(id, num, order); }
}
*/

pub fn get_renderer_name_cstr(kind: RendererKind) -> &'static CStr {
    // XXX this is mapped from an enum to a static string
    // on BGFX's side, so its probably safe to assume a
    // static lifetime
    unsafe { return CStr::from_ptr(bgfx_get_renderer_name(kind)); }
}

pub fn reset(width: u32, height: u32, flags: u32) {
    unsafe { bgfx_reset(width, height, flags); }
}

/// Advances to the next frame. `capture` will catch the frame in
/// the graphics debugger. Returns the current frame number. Note
/// that the `Frame` sentinel calls this when dropped, so you don't
/// need to use both at once.
pub fn frame(capture: bool) -> u32 {
    unsafe { return bgfx_frame(capture); }
}

//---------------------------------------------------------------------
// Texture handles, 2D
//---------------------------------------------------------------------

impl TextureHandle2D {
    /// *TODO*: API is subject to change; texture flags to be replaced with a typed bitfield.
    pub fn with_memory(width: u16, height: u16, has_mips: bool, num_layers: u16, format: TextureFormat, flags: u32, mem: &Memory) -> TextureHandle2D {
        unsafe{TextureHandle2D{handle:bgfx_create_texture_2d(width, height, has_mips, num_layers, format, flags, mem.handle)}}
    }

    /// *TODO*: API is subject to change; texture flags to be replaced with a typed bitfield.
    pub fn with_scale(ratio: BackbufferRatio, has_mips: bool, num_layers: u16, format: TextureFormat, flags: u32) -> TextureHandle2D {
        unsafe{TextureHandle2D{handle:bgfx_create_texture_2d_scaled(ratio, has_mips, num_layers, format, flags)}}
    }

    pub fn update(&mut self, layer: u16, mip: u8, x: u16, y: u16, width: u16, height: u16, mem: &Memory, pitch: u16) {
        unsafe { bgfx_update_texture_2d(self.handle, layer, mip, x, y, width, height, mem.handle, pitch); }
    }

    pub fn read_texture(&self, data: &mut [u8], mip: u8) -> u32 {
        // TODO ensure vector has enough space to receive the texture
        unsafe { bgfx_read_texture(self.handle, transmute(data.as_mut_ptr()), mip) }
    }
}

impl TextureHandle for TextureHandle2D {
    fn set_name(&mut self, name: &str) {
        let cstring = CString::new(name).unwrap();
        let cstr = cstring.as_c_str();
        let ptr = cstr.as_ptr();

        unsafe { bgfx_set_texture_name(self.handle, ptr, cstr.to_bytes().len() as i32) }
    }

    unsafe fn expose_handle(&self) -> TextureHandleImpl {
        self.handle
    }
}

impl Drop for TextureHandle2D {
    fn drop(&mut self) {
        unsafe { bgfx_destroy_texture(self.handle); }
    }
}

//---------------------------------------------------------------------
// Texture handles, 3D
//---------------------------------------------------------------------

impl TextureHandle3D {
    pub fn update(&mut self, mip: u8, x: u16, y: u16, z: u16, width: u16, height: u16, depth: u16, mem: &Memory) {
        unsafe { bgfx_update_texture_3d(self.handle, mip, x, y, z, width, height, depth, mem.handle); }
    }

    pub fn with_memory(width: u16, height: u16, depth: u16, has_mips: bool, format: TextureFormat, flags: u32, mem: &Memory) -> TextureHandle3D {
        unsafe { TextureHandle3D{handle:bgfx_create_texture_3d(width, height, depth, has_mips, format, flags, mem.handle)}}
    }

    pub fn read_texture(&self, data: &mut [u8], mip: u8) -> u32 {
        // TODO ensure vector has enough space to receive the texture
        unsafe { bgfx_read_texture(self.handle, transmute(data.as_mut_ptr()), mip) }
    }
}

impl TextureHandle for TextureHandle3D {
    fn set_name(&mut self, name: &str) {
        let cstring = CString::new(name).unwrap();
        let cstr = cstring.as_c_str();
        let ptr = cstr.as_ptr();

        unsafe { bgfx_set_texture_name(self.handle, ptr, cstr.to_bytes().len() as i32); }
    }

    unsafe fn expose_handle(&self) -> TextureHandleImpl {
        self.handle
    }
}

impl Drop for TextureHandle3D {
    fn drop(&mut self) {
        unsafe { bgfx_destroy_texture(self.handle); }
    }
}

//---------------------------------------------------------------------
// Texture handles, Cube
//---------------------------------------------------------------------

impl TextureHandleCube {
    pub fn update_cube(&mut self, layer: u16, side: u8, mip: u8, x: u16, y: u16, width: u16, height: u16, mem: &Memory, pitch: u16) {
        unsafe { bgfx_update_texture_cube(self.handle, layer, side, mip, x, y, width, height, mem.handle, pitch); }
    }

    /// *TODO*: API is subject to change; texture flags to be replaced with a typed bitfield.
    pub fn with_memory(size: u16, has_mips: bool, num_layers: u16, format: TextureFormat, flags: u32, mem: &Memory) -> TextureHandleCube {
        unsafe { TextureHandleCube{handle:bgfx_create_texture_cube(size, has_mips, num_layers, format, flags, mem.handle)}}
    }

    pub fn read_texture(&self, data: &mut [u8], mip: u8) -> u32 {
        // TODO ensure vector has enough space to receive the texture
        unsafe { bgfx_read_texture(self.handle, transmute(data.as_mut_ptr()), mip) }
    }
}

impl TextureHandle for TextureHandleCube {
    fn set_name(&mut self, name: &str) {
        let cstring = CString::new(name).unwrap();
        let cstr = cstring.as_c_str();
        let ptr = cstr.as_ptr();

        unsafe { bgfx_set_texture_name(self.handle, ptr, cstr.to_bytes().len() as i32); }
    }

    unsafe fn expose_handle(&self) -> TextureHandleImpl {
        self.handle
    }
}

impl Drop for TextureHandleCube {
    fn drop(&mut self) {
        unsafe { bgfx_destroy_texture(self.handle); }
    }
}

pub fn get_supported_renderers(max: u8, kind: RendererKind ) -> u8 {
    // i don't think the enum actually gets modified C-side,
    // but they didn't mark it as const so we make a copy
    // here to be safe.
    let mut here = kind;
    unsafe { bgfx_get_supported_renderers(max, &mut here) }
}

pub fn get_renderer_type() -> RendererKind {
    unsafe { bgfx_get_renderer_type() }
}

pub fn get_avail_transient_index_buffer(len: u32) -> u32 {
    unsafe { bgfx_get_avail_transient_index_buffer(len) }
}

pub fn get_avail_transient_vertex_buffer(len: u32, decl: &VertexDecl) -> u32 {
    unsafe { bgfx_get_avail_transient_vertex_buffer(len, decl) }
}

pub fn get_avail_instance_data_buffer(len: u32, stride: u16) -> u32 {
    unsafe { bgfx_get_avail_instance_data_buffer(len, stride) }
}

impl FrameBufferHandle {
    /// *TODO*: API is subject to change; texture flags to be replaced with a typed bitfield.
    pub fn with_size(width: u16, height: u16, format: TextureFormat, texture_flags: u32) -> FrameBufferHandle {
        unsafe { FrameBufferHandle{handle:bgfx_create_frame_buffer(width, height, format, texture_flags)} }
    }

    /// *TODO*: API is subject to change; texture flags to be replaced with a typed bitfield.
    pub fn with_backbuffer_ratio(ratio: BackbufferRatio, format: TextureFormat, texture_flags: u32) -> FrameBufferHandle {
        unsafe { FrameBufferHandle{handle:bgfx_create_frame_buffer_scaled(ratio, format, texture_flags)} }
    }

    pub fn with_handles<T: TextureHandle>(handles: &[T]) -> FrameBufferHandle {
        if handles.len() > 255 { panic!("Handle slice size must be <= 255!") }
        // I'm pretty sure BGFX isn't meant to actually
        // modify these, UNLESS you allowed it to delete the
        // textures. We don't (to keep the lifetimes of our
        // rustified handles valid) so we might be able to
        // get away with bullshitting the const checker here.
        unsafe {
            let mut y: Vec<TextureHandleImpl> = handles.into_iter().map(|x| x.expose_handle()).collect();
            let x = y.as_mut_ptr() as *mut TextureHandleImpl;
            FrameBufferHandle{handle:bgfx_create_frame_buffer_from_handles(handles.len() as u8, x, false)}
        }
    }

/* TODO
this is tricky because we don't want to clone attachments
(would violate lifetime assertion of their textures) but
creating them requires the texture handle; so we end up
having to keep a public/private pair of these structs and
convert them here to avoid violating lifetime concerns

    pub fn with_attachments(attachments: &[Attachment]) -> FrameBufferHandle {
        if attachments.len() > 255 { panic!("Handle slice size must be <= 255!") }
        // I'm pretty sure BGFX isn't meant to actually
        // modify these, UNLESS you allowed it to delete the
        // textures. We don't (to keep the lifetimes of our
        // rustified handles valid) so we might be able to
        // get away with bullshitting the const checker here.
        unsafe { FrameBufferHandle{handle:bgfx_create_frame_buffer_from_attachment(attachments.len() as u8, x as *mut Attachment, false)} }
    }
*/

    /// *TODO*: API is subject to change; texture flags to be replaced with a typed bitfield.
    pub unsafe fn with_native_window_handle(nwh: *mut c_void, width: u16, height: u16, depth_format: TextureFormat) -> FrameBufferHandle {
        FrameBufferHandle{handle:bgfx_create_frame_buffer_from_nwh(nwh, width, height, depth_format)}
    }
}

impl Drop for FrameBufferHandle {
    fn drop(&mut self) {
        unsafe {bgfx_destroy_frame_buffer(self.handle) }
    }
}

impl IndexBufferHandle {
    /// *TODO*: API is subject to change; flags to be replaced with a typed bitfield.
    pub fn bgfx_create_index_buffer(mem: &Memory, flags: u16) -> IndexBufferHandle {
        unsafe {IndexBufferHandle{handle:bgfx_create_index_buffer(mem.handle, flags)}}
    }
}

impl Drop for IndexBufferHandle {
    fn drop(&mut self) {
        unsafe {bgfx_destroy_index_buffer(self.handle)}
    }
}

impl VertexBufferHandle {
    /// *TODO*: API is subject to change; flags to be replaced with a typed bitfield.
    pub fn from_memory(mem: &Memory, decl: &VertexDecl, flags: u16) -> VertexBufferHandle {
        unsafe { VertexBufferHandle{handle:bgfx_create_vertex_buffer(mem.handle, decl, flags)}}
    }
}

impl Drop for VertexBufferHandle {
    fn drop(&mut self) {
        unsafe { bgfx_destroy_vertex_buffer(self.handle) }
    }
}

impl DynamicIndexBufferHandle {
    /// *TODO*: API is subject to change; flags to be replaced with a typed bitfield.
    pub fn with_length(len: u32, flags: u16) -> DynamicIndexBufferHandle {
        unsafe { DynamicIndexBufferHandle{handle: bgfx_create_dynamic_index_buffer(len, flags)}}
    }

    /// *TODO*: API is subject to change; flags to be replaced with a typed bitfield.
    pub fn with_memory(mem: &Memory, flags: u16) -> DynamicIndexBufferHandle {
        unsafe { DynamicIndexBufferHandle{handle: bgfx_create_dynamic_index_buffer_mem(mem.handle, flags)}}
    }

    pub fn update(&mut self, start_index: u32, mem: &Memory) {
        unsafe { bgfx_update_dynamic_index_buffer(self.handle, start_index, mem.handle) }
    }
}

impl Drop for DynamicIndexBufferHandle {
    fn drop(&mut self) {
        unsafe { bgfx_destroy_dynamic_index_buffer(self.handle) }
    }
}

impl DynamicVertexBufferHandle {
    /// *TODO*: API is subject to change; flags to be replaced with a typed bitfield.
    pub fn with_length(len: u32, decl: &VertexDecl, flags: u16) -> DynamicVertexBufferHandle {
        unsafe {DynamicVertexBufferHandle{handle:bgfx_create_dynamic_vertex_buffer(len, decl, flags)}}
    }

    /// *TODO*: API is subject to change; flags to be replaced with a typed bitfield.
    pub fn with_memory(mem: &Memory, decl: &VertexDecl, flags: u16) -> DynamicVertexBufferHandle {
        unsafe {DynamicVertexBufferHandle{handle:bgfx_create_dynamic_vertex_buffer_mem(mem.handle, decl, flags)}}
    }

    pub fn update(&self, start_vertex: u32, mem: &Memory) {
        unsafe {bgfx_update_dynamic_vertex_buffer(self.handle, start_vertex, mem.handle)}
    }
}

impl Drop for DynamicVertexBufferHandle {
    fn drop(&mut self) {
        unsafe {bgfx_destroy_dynamic_vertex_buffer(self.handle)}
    }
}

impl TransientIndexBuffer {
    pub unsafe fn into(tib: &mut TransientIndexBuffer, len: u32) {
        bgfx_alloc_transient_index_buffer(tib, len)
    }

    pub fn allocate<'a>(_frame: &'a Frame, buf: &'a mut TransientIndexBuffer, len: u32) {
        unsafe {TransientIndexBuffer::into(buf, len)}
    }
}

impl TransientVertexBuffer {
    pub unsafe fn into(tvb: &mut TransientVertexBuffer, len: u32, decl: &VertexDecl) {
        bgfx_alloc_transient_vertex_buffer(tvb, len, decl)
    }

    pub fn allocate<'a>(_frame: &'a Frame, buf: &'a mut TransientVertexBuffer, len: u32, decl: &VertexDecl) {
        unsafe {TransientVertexBuffer::into(buf, len, decl)}
    }
}

impl InstanceDataBuffer {
    pub unsafe fn into(idb: &mut InstanceDataBuffer, len: u32, stride: u16) {
        // TODO check that stride is a multiple of 16
        bgfx_alloc_instance_data_buffer(idb, len, stride)
    }

    pub fn allocate<'a>(_frame: &'a Frame, idb: &mut InstanceDataBuffer, len: u32, stride: u16) {
        unsafe{InstanceDataBuffer::into(idb, len, stride)}
    }
}

impl ShaderHandle {
    pub fn with_memory(mem: &Memory) -> ShaderHandle {
        unsafe{ShaderHandle{handle:bgfx_create_shader(mem.handle)}}
    }

    pub fn get_uniform_count(&mut self) -> u16 {
        unsafe{bgfx_get_shader_uniforms(self.handle, std::ptr::null_mut(), 0) as u16}
    }

    pub fn get_uniforms(&mut self, maximum: u16) -> Vec<UniformHandleImpl> {
        let length = if maximum > 0 {
                std::cmp::min(self.get_uniform_count(), maximum)
            } else {
            	maximum
            };

        let mut items: Vec<UniformHandleImpl> = Vec::with_capacity(length as usize);
        unsafe {
            bgfx_get_shader_uniforms(self.handle, items.as_mut_ptr(), length as u16);
        }
        return items
    }

    pub fn set_name(&mut self, name: &str) {
    	let cstring = CString::new(name).unwrap();
    	let cstr    = cstring.as_c_str();
    	let ptr     = cstr.as_ptr();
        unsafe{bgfx_set_shader_name(self.handle, ptr, cstr.to_bytes().len() as i32)}
    }
}

impl Drop for ShaderHandle {
    fn drop(&mut self) {
        unsafe{bgfx_destroy_shader(self.handle)}
    }
}

//---------------------------------------------------------------------
// Indirect buffers
//---------------------------------------------------------------------

impl IndexBufferHandle {
    pub fn with_length(len: u32) -> IndexBufferHandle {
        unsafe {IndexBufferHandle{handle:bgfx_create_indirect_buffer(len)}}
    }
}

impl Drop for IndirectBufferHandle {
    fn drop(&mut self) {
        unsafe {bgfx_destroy_indirect_buffer(self.handle)}
    }
}

impl UniformHandle {
// fn bgfx_create_uniform(name: *const c_char, kind: UniformKind, num: u16) -> UniformHandleImpl;
//    pub fn get_uniform_info(handle: UniformHandleImpl, info: *mut UniformInfo );
//        fn bgfx_get_uniform_info(handle: UniformHandleImpl, info: *mut UniformInfo );
}

impl Drop for UniformHandle {
    fn drop(&mut self) {
        unsafe{bgfx_destroy_uniform(self.handle)}
    }
}

/// *TODO*: Replace `attr` with a hard type.
pub fn debug_text_clear(attr: u8, small: bool) {
    unsafe {bgfx_dbg_text_clear(attr, small)}
}

/// Sends a request to print text throuhg the on-screen debugging
/// facilities. While the C-side accepts variadic arguments and a
/// formatting parameters, the Rust side only accepts a string. Use
/// a Rust native formatting facility to prepare your string.
/// *TODO*: Replace `attr` with a hard type.
pub fn debug_text_print(x: u16, y: u16, attr: u8, text: &str) {
    unsafe {
        let format  = CStr::from_ptr("%\0".as_ptr() as *const i8);
        let cstring = CString::new(text).unwrap();
        let ptr     = cstring.as_ptr();

        bgfx_dbg_text_printf(x, y, attr, format.as_ptr(), ptr)
    }
}

pub unsafe fn debug_text_image(x: u16, y: u16, width: u16, height: u16, data: &[u8], pitch: u16) {
    // TODO check that slice is large enough to hold image
    let ptr = data.as_ptr();
    bgfx_dbg_text_image(x, y, width, height, transmute(ptr), pitch)
}

pub fn is_texture_valid(depth: u16, cube_map: bool, num_layers: u16, format: TextureFormat, flags: u32) -> bool {
    unsafe {bgfx_is_texture_valid(depth, cube_map, num_layers, format, flags)}
}

pub fn calc_texture_size(info: &mut TextureInfo, width: u16, height: u16, depth: u16, cube_map: bool, has_mips: bool, num_layers: u16, format: TextureFormat) {
    unsafe {bgfx_calc_texture_size(info, width, height, depth, cube_map, has_mips, num_layers, format)}
}

//---------------------------------------------------------------------
// Occlusion query
//---------------------------------------------------------------------

impl OcclusionQueryHandle {
    fn new() -> OcclusionQueryHandle {
        unsafe{OcclusionQueryHandle{handle:bgfx_create_occlusion_query()}}
    }

    fn get_result(&mut self, result: &mut i32) -> OcclusionQueryResult {
        unsafe{bgfx_get_result(self.handle, result)}
    }
}

impl Drop for OcclusionQueryHandle {
    fn drop(&mut self) {
        unsafe{bgfx_destroy_occlusion_query(self.handle)}
    }
}

//---------------------------------------------------------------------
// Programs (linked shaders)
//---------------------------------------------------------------------

impl ProgramHandle {
    pub fn with_vertex_fragment(vertex: ShaderHandle, fragment: ShaderHandle) -> ProgramHandle {
        unsafe{ProgramHandle{handle:bgfx_create_program(vertex.handle, fragment.handle, false)}}
    }

    pub fn with_compute(compute: ShaderHandle) -> ProgramHandle {
        unsafe{ProgramHandle{handle:bgfx_create_compute_program(compute.handle, false)}}
    }
}

impl Drop for ProgramHandle {
    fn drop(&mut self) {
        unsafe{bgfx_destroy_program(self.handle)}
    }
}

/// *TODO*: API is subject to change; debug to be replaced with a typed bitfield.
pub fn set_debug(debug: u32) {
    unsafe{bgfx_set_debug(debug)}
}

pub fn vertex_pack(input: [c_float; 4], input_normalized: bool, attr: Attrib, decl: &VertexDecl, data: &mut [u8], index: u32) {
    // TODO check that data vector is large enough to hold decl.size * index
    let ptr = data.as_mut_ptr();
    unsafe{bgfx_vertex_pack(input, input_normalized, attr, decl, transmute(ptr), index)}
}

pub fn vertex_unpack(output: &mut [c_float; 4], attr: Attrib, decl: &VertexDecl, data: &[u8], index: u32) {
    // TODO check that data vector is large enough to hold decl.size * index
    let ptr = data.as_ptr();
    unsafe{bgfx_vertex_unpack(*output, attr, decl, transmute(ptr), index)}
}

pub fn vertex_convert(dest_decl: &VertexDecl, dest_data: &mut [u8], src_decl: &VertexDecl, src_data: &[u8], num: u32) {
    let ptr1 = dest_data.as_mut_ptr();
    let ptr2 = src_data.as_ptr();
    unsafe{bgfx_vertex_convert(dest_decl, transmute(ptr1), src_decl, transmute(ptr2), num)}
}

pub fn weld_vertices(output: &mut [u16], decl: &VertexDecl, data: &[u8], len: u16, epsilon: f32) -> u16 {
    if (len as usize) < output.len() { panic!("Output index vector must be at least as large as `len`.") }
    let ptr = data.as_ptr();
    unsafe{bgfx_weld_vertices(output.as_mut_ptr(), decl, transmute(ptr), len, epsilon as c_float)}
}

pub fn topology_convert16(conversion: TopologyConvert, dst: &mut [u8], indices: &[u16]) -> u32 {
    let ptr1 = dst.as_mut_ptr();
    let ptr2 = indices.as_ptr();
    unsafe{bgfx_topology_convert(conversion, transmute(ptr1), dst.len() as u32, transmute(ptr2), indices.len() as u32, false)}
}

pub fn topology_convert32(conversion: TopologyConvert, dst: &mut [u8], indices: &[u32]) -> u32 {
    let ptr1 = dst.as_mut_ptr();
    let ptr2 = indices.as_ptr();
    // TODO make sure dst has enough space
    unsafe{bgfx_topology_convert(conversion, transmute(ptr1), dst.len() as u32, transmute(ptr2), indices.len() as u32, true)}
}

pub fn topology_sort_tri_list(sort: TopologySort, dst: &mut [u8], dir: [c_float; 3], pos: [c_float; 3], vertices: &[u8], stride: u32, indices: &[u16]) {
    let ptr1 = dst.as_mut_ptr();
    let ptr2 = vertices.as_ptr();
    let ptr3 = indices.as_ptr();
    unsafe{bgfx_topology_sort_tri_list(sort, transmute(ptr1), dst.len() as u32, dir, pos, transmute(ptr2), stride, transmute(ptr3), indices.len() as u32, false)}
}

// XXX yes, bgfx itself asserts these are static lifetime structs
// TODO test that these transmutes actually work as expected

pub fn get_caps() -> &'static Caps {
    unsafe {transmute(bgfx_get_caps())}
}

pub fn get_stats() -> &'static Stats {
    unsafe{transmute(bgfx_get_stats())}
}

impl Memory{
    /// Allocates new memory in C space. Data will be freed inside
    /// bgfx.
    pub fn alloc(size: u32) -> Memory {
        unsafe{Memory{handle:bgfx_alloc(size)}}
    }

    /// Copies the supplied slice over to C memory. Data will be
    /// freed inside bgfx.
    pub fn copy(data: &[u8]) -> Memory {
        let ptr1 = data.as_ptr();
        unsafe{Memory{handle:bgfx_copy(transmute(ptr1), data.len() as u32)}}
    }

    /// Creates a memory handle which refers to the `data` passed
    /// in. `data` must live for at least two calls to `frame`. Unsafe
    /// because this assurance cannot be made by the borrow system.
    pub unsafe fn make_ref_unsafe(data: &[u8]) -> Memory {
        let ptr1 = data.as_ptr();
        Memory{handle:bgfx_make_ref(transmute(ptr1), data.len() as u32)}
    }

    pub unsafe fn make_ref_release_unsafe(data: &[u8], release_fn: ReleaseFn, user_data: *mut c_void) -> Memory {
        let ptr1 = data.as_ptr();
        Memory{handle:bgfx_make_ref_release(transmute(ptr1), data.len() as u32, release_fn, user_data)}
    }
}

