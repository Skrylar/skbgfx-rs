
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

pub type release_fn = extern fn(ptr: *mut c_void, userData: *mut c_void);

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
    pub name: [u8; 256],
    pub view: ViewId,
    pub cpuTimeElapsed: i64,
    pub gpuTimeElapsed: i64,
}

#[repr(C)]
pub struct EncoderStats {
    pub cpuTimeBegin: i64,
    pub cpuTimeEnd: i64,
}

#[repr(C)]
pub struct Stats {
    pub cpuTimeFrame           : i64,
    pub cpuTimeBegin           : i64,
    pub cpuTimeEnd             : i64,
    pub cpuTimerFreq           : i64,
    pub gpuTimeBegin           : i64,
    pub gpuTimeEnd             : i64,
    pub gpuTimerFreq           : i64,
    pub waitRender             : i64,
    pub waitSubmit             : i64,
    pub numDraw                : u32,
    pub numCompute             : u32,
    pub maxGpuLatency          : u32,
    pub numDynamicIndexBuffers : u16,
    pub numDynamicVertexBuffers: u16,
    pub numFrameBuffers        : u16,
    pub numIndexBuffers        : u16,
    pub numOcclusionQueries    : u16,
    pub numPrograms            : u16,
    pub numShaders             : u16,
    pub numTextures            : u16,
    pub numUniforms            : u16,
    pub numVertexBuffers       : u16,
    pub numVertexDecls         : u16,
    pub textureMemoryUsed      : i64,
    pub rtMemoryUsed           : i64,
    pub transientVbUsed        : i32,
    pub transientIbUsed        : i32,
    pub numPrims               : [u32; TOPOLOGY_COUNT],
    pub gpuMemoryMax           : i64,
    pub gpuMemoryUsed          : i64,
    pub width                  : u16,
    pub height                 : u16,
    pub textWidth              : u16,
    pub textHeight             : u16,
    pub numViews               : u16,
    pub viewStats              : *mut ViewStats,
    pub numEncoders            : u8,
    pub encoderStats           : *mut EncoderStats,
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
    pub data      : *mut u8,
    pub size      : u32,
    pub handle    : IndexBufferHandle,
    pub startIndex: u32,
}

#[repr(C)]
pub struct TransientVertexBuffer {
    pub data       : *mut u8,
    pub size       : u32,
    pub startVertex: u32,
    pub stride     : u16,
    pub handle     : VertexBufferHandle,
    pub decl       : VertexDeclHandle,
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
    pub format      : TextureFormat,
    pub storageSize : u32,
    pub width       : u16,
    pub height      : u16,
    pub depth       : u16,
    pub numLayers   : u16,
    pub numMips     : u8,
    pub bitsPerPixel: u8,
    pub cubeMap     : bool,
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
    pub vendorId: u16,
    pub deviceId: u16,
}

#[repr(C)]
pub struct CapsLimits {
    pub maxDrawCalls           : u32,
    pub maxBlits               : u32,
    pub maxTextureSize         : u32,
    pub maxTextureLayers       : u32,
    pub maxViews               : u32,
    pub maxFrameBuffers        : u32,
    pub maxFBAttachments       : u32,
    pub maxPrograms            : u32,
    pub maxShaders             : u32,
    pub maxTextures            : u32,
    pub maxTextureSamplers     : u32,
    pub maxVertexDecls         : u32,
    pub maxVertexStreams       : u32,
    pub maxIndexBuffers        : u32,
    pub maxVertexBuffers       : u32,
    pub maxDynamicIndexBuffers : u32,
    pub maxDynamicVertexBuffers: u32,
    pub maxUniforms            : u32,
    pub maxOcclusionQueries    : u32,
    pub maxEncoders            : u32,
    pub transientVbSize        : u32,
    pub transientIbSize        : u32,
}

#[repr(C)]
pub struct Caps {
    pub rendererKind    : RendererKind,
    pub supported       : u64,
    pub vendorId        : u16,
    pub deviceId        : u16,
    pub homogeneousDepth: bool,
    pub originBottomLeft: bool,
    pub numGPUs         : u8,
    pub gpu             : [CapsGpu; 4],
    pub limits          : CapsLimits,
    pub formats         : [u16; TEXTURE_FORMAT_COUNT],
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
    pub maxEncoders    : u16,
    pub transientVbSize: u32,
    pub transientIbSize: u32,
}

#[repr(C)]
pub struct Init {
    pub kind      : RendererKind,
    pub vendorId  : u16,
    pub deviceId  : u16,
    pub debug     : bool,
    pub profile   : bool,
    pub resolution: Resolution,
    pub limits    : InitLimits,
    pub callback  : *mut CallbackInterface,
    pub allocator : *mut AllocatorInterface,
}

/*
BGFX_C_API void bgfx_vertex_decl_begin(bgfx_vertex_decl_t* _decl, bgfx_renderer_type_t _renderer);
BGFX_C_API void bgfx_vertex_decl_add(bgfx_vertex_decl_t* _decl, bgfx_attrib_t _attrib, u8 _num, bgfx_attrib_type_t _type, bool _normalized, bool _asInt);
BGFX_C_API void bgfx_vertex_decl_decode(const bgfx_vertex_decl_t* _decl, bgfx_attrib_t _attrib, u8* _num, bgfx_attrib_type_t* _type, bool* _normalized, bool* _asInt);
BGFX_C_API bool bgfx_vertex_decl_has(const bgfx_vertex_decl_t* _decl, bgfx_attrib_t _attrib);
BGFX_C_API void bgfx_vertex_decl_skip(bgfx_vertex_decl_t* _decl, u8 _num);
BGFX_C_API void bgfx_vertex_decl_end(bgfx_vertex_decl_t* _decl);
BGFX_C_API void bgfx_vertex_pack(const float _input[4], bool _inputNormalized, bgfx_attrib_t _attr, const bgfx_vertex_decl_t* _decl, void* _data, u32 _index);
BGFX_C_API void bgfx_vertex_unpack(float _output[4], bgfx_attrib_t _attr, const bgfx_vertex_decl_t* _decl, const void* _data, u32 _index);
BGFX_C_API void bgfx_vertex_convert(const bgfx_vertex_decl_t* _destDecl, void* _destData, const bgfx_vertex_decl_t* _srcDecl, const void* _srcData, u32 _num);
BGFX_C_API u16 bgfx_weld_vertices(u16* _output, const bgfx_vertex_decl_t* _decl, const void* _data, u16 _num, float _epsilon);
BGFX_C_API u32 bgfx_topology_convert(bgfx_topology_convert_t _conversion, void* _dst, u32 _dstSize, const void* _indices, u32 _numIndices, bool _index32)BGFX_C_API void bgfx_topology_sort_tri_list(bgfx_topology_sort_t _sort, void* _dst, u32 _dstSize, const float _dir[3], const float _pos[3], const void* _vertices, u32 _stride, const void* _indices, u32 _numIndices, bool _index32);
BGFX_C_API u8 bgfx_get_supported_renderers(u8 _max, bgfx_renderer_type_t* _enum);
BGFX_C_API const char* bgfx_get_renderer_name(bgfx_renderer_type_t _type);
BGFX_C_API void bgfx_init_ctor(bgfx_init_t* _init);
BGFX_C_API bool bgfx_init(const bgfx_init_t* _init);
BGFX_C_API void bgfx_shutdown(void);
BGFX_C_API void bgfx_reset(u32 _width, u32 _height, u32 _flags);
BGFX_C_API struct bgfx_encoder_s* bgfx_begin(void);
BGFX_C_API void bgfx_end(struct bgfx_encoder_s* _encoder);
BGFX_C_API u32 bgfx_frame(bool _capture);
BGFX_C_API bgfx_renderer_type_t bgfx_get_renderer_type(void);
BGFX_C_API const bgfx_caps_t* bgfx_get_caps(void);
BGFX_C_API const bgfx_stats_t* bgfx_get_stats(void);
BGFX_C_API const bgfx_memory_t* bgfx_alloc(u32 _size);
BGFX_C_API const bgfx_memory_t* bgfx_copy(const void* _data, u32 _size);
BGFX_C_API const bgfx_memory_t* bgfx_make_ref(const void* _data, u32 _size);
BGFX_C_API const bgfx_memory_t* bgfx_make_ref_release(const void* _data, u32 _size, bgfx_release_fn_t _releaseFn, void* _userData);
BGFX_C_API void bgfx_set_debug(u32 _debug);
BGFX_C_API void bgfx_dbg_text_clear(u8 _attr, bool _small);
BGFX_C_API void bgfx_dbg_text_printf(u16 _x, u16 _y, u8 _attr, const char* _format, ...);
BGFX_C_API void bgfx_dbg_text_vprintf(u16 _x, u16 _y, u8 _attr, const char* _format, va_list _argList);
BGFX_C_API void bgfx_dbg_text_image(u16 _x, u16 _y, u16 _width, u16 _height, const void* _data, u16 _pitch);
BGFX_C_API bgfx_index_buffer_handle_t bgfx_create_index_buffer(const bgfx_memory_t* _mem, u16 _flags);
BGFX_C_API void bgfx_destroy_index_buffer(bgfx_index_buffer_handle_t _handle);
BGFX_C_API bgfx_vertex_buffer_handle_t bgfx_create_vertex_buffer(const bgfx_memory_t* _mem, const bgfx_vertex_decl_t* _decl, u16 _flags);
BGFX_C_API void bgfx_destroy_vertex_buffer(bgfx_vertex_buffer_handle_t _handle);
BGFX_C_API bgfx_dynamic_index_buffer_handle_t bgfx_create_dynamic_index_buffer(u32 _num, u16 _flags);
BGFX_C_API bgfx_dynamic_index_buffer_handle_t bgfx_create_dynamic_index_buffer_mem(const bgfx_memory_t* _mem, u16 _flags);
BGFX_C_API void bgfx_update_dynamic_index_buffer(bgfx_dynamic_index_buffer_handle_t _handle, u32 _startIndex, const bgfx_memory_t* _mem);
BGFX_C_API void bgfx_destroy_dynamic_index_buffer(bgfx_dynamic_index_buffer_handle_t _handle);
BGFX_C_API bgfx_dynamic_vertex_buffer_handle_t bgfx_create_dynamic_vertex_buffer(u32 _num, const bgfx_vertex_decl_t* _decl, u16 _flags);
BGFX_C_API bgfx_dynamic_vertex_buffer_handle_t bgfx_create_dynamic_vertex_buffer_mem(const bgfx_memory_t* _mem, const bgfx_vertex_decl_t* _decl, u16 _flags);
BGFX_C_API void bgfx_update_dynamic_vertex_buffer(bgfx_dynamic_vertex_buffer_handle_t _handle, u32 _startVertex, const bgfx_memory_t* _mem);
BGFX_C_API void bgfx_destroy_dynamic_vertex_buffer(bgfx_dynamic_vertex_buffer_handle_t _handle);
BGFX_C_API u32 bgfx_get_avail_transient_index_buffer(u32 _num);
BGFX_C_API u32 bgfx_get_avail_transient_vertex_buffer(u32 _num, const bgfx_vertex_decl_t* _decl);
BGFX_C_API u32 bgfx_get_avail_instance_data_buffer(u32 _num, u16 _stride);
BGFX_C_API void bgfx_alloc_transient_index_buffer(bgfx_transient_index_buffer_t* _tib, u32 _num);
BGFX_C_API void bgfx_alloc_transient_vertex_buffer(bgfx_transient_vertex_buffer_t* _tvb, u32 _num, const bgfx_vertex_decl_t* _decl);
BGFX_C_API bool bgfx_alloc_transient_buffers(bgfx_transient_vertex_buffer_t* _tvb, const bgfx_vertex_decl_t* _decl, u32 _numVertices, bgfx_transient_index_buffer_t* _tib, u32 _numIndices);
BGFX_C_API void bgfx_alloc_instance_data_buffer(bgfx_instance_data_buffer_t* _idb, u32 _num, u16 _stride);
BGFX_C_API bgfx_indirect_buffer_handle_t bgfx_create_indirect_buffer(u32 _num);
BGFX_C_API void bgfx_destroy_indirect_buffer(bgfx_indirect_buffer_handle_t _handle);
BGFX_C_API bgfx_shader_handle_t bgfx_create_shader(const bgfx_memory_t* _mem);
BGFX_C_API u16 bgfx_get_shader_uniforms(bgfx_shader_handle_t _handle, bgfx_uniform_handle_t* _uniforms, u16 _max);
BGFX_C_API void bgfx_get_uniform_info(bgfx_uniform_handle_t _handle, bgfx_uniform_info_t* _info);
BGFX_C_API void bgfx_set_shader_name(bgfx_shader_handle_t _handle, const char* _name, i32 _len);
BGFX_C_API void bgfx_destroy_shader(bgfx_shader_handle_t _handle);
BGFX_C_API bgfx_program_handle_t bgfx_create_program(bgfx_shader_handle_t _vsh, bgfx_shader_handle_t _fsh, bool _destroyShaders);
BGFX_C_API bgfx_program_handle_t bgfx_create_compute_program(bgfx_shader_handle_t _csh, bool _destroyShaders);
BGFX_C_API void bgfx_destroy_program(bgfx_program_handle_t _handle);
BGFX_C_API bool bgfx_is_texture_valid(u16 _depth, bool _cubeMap, u16 _numLayers, bgfx_texture_format_t _format, u32 _flags);
BGFX_C_API void bgfx_calc_texture_size(bgfx_texture_info_t* _info, u16 _width, u16 _height, u16 _depth, bool _cubeMap, bool _hasMips, u16 _numLayers, bgfx_texture_format_t _format);
BGFX_C_API bgfx_texture_handle_t bgfx_create_texture(const bgfx_memory_t* _mem, u32 _flags, u8 _skip, bgfx_texture_info_t* _info);
BGFX_C_API bgfx_texture_handle_t bgfx_create_texture_2d(u16 _width, u16 _height, bool _hasMips, u16 _numLayers, bgfx_texture_format_t _format, u32 _flags, const bgfx_memory_t* _mem);
BGFX_C_API bgfx_texture_handle_t bgfx_create_texture_2d_scaled(bgfx_backbuffer_ratio_t _ratio, bool _hasMips, u16 _numLayers, bgfx_texture_format_t _format, u32 _flags);
BGFX_C_API bgfx_texture_handle_t bgfx_create_texture_3d(u16 _width, u16 _height, u16 _depth, bool _hasMips, bgfx_texture_format_t _format, u32 _flags, const bgfx_memory_t* _mem);
BGFX_C_API bgfx_texture_handle_t bgfx_create_texture_cube(u16 _size, bool _hasMips, u16 _numLayers, bgfx_texture_format_t _format, u32 _flags, const bgfx_memory_t* _mem);
BGFX_C_API void bgfx_update_texture_2d(bgfx_texture_handle_t _handle, u16 _layer, u8 _mip, u16 _x, u16 _y, u16 _width, u16 _height, const bgfx_memory_t* _mem, u16 _pitch);
BGFX_C_API void bgfx_update_texture_3d(bgfx_texture_handle_t _handle, u8 _mip, u16 _x, u16 _y, u16 _z, u16 _width, u16 _height, u16 _depth, const bgfx_memory_t* _mem);
BGFX_C_API void bgfx_update_texture_cube(bgfx_texture_handle_t _handle, u16 _layer, u8 _side, u8 _mip, u16 _x, u16 _y, u16 _width, u16 _height, const bgfx_memory_t* _mem, u16 _pitch);
BGFX_C_API u32 bgfx_read_texture(bgfx_texture_handle_t _handle, void* _data, u8 _mip);
BGFX_C_API void bgfx_set_texture_name(bgfx_texture_handle_t _handle, const char* _name, i32 _len);
BGFX_C_API void bgfx_destroy_texture(bgfx_texture_handle_t _handle);
BGFX_C_API bgfx_frame_buffer_handle_t bgfx_create_frame_buffer(u16 _width, u16 _height, bgfx_texture_format_t _format, u32 _textureFlags);
BGFX_C_API bgfx_frame_buffer_handle_t bgfx_create_frame_buffer_scaled(bgfx_backbuffer_ratio_t _ratio, bgfx_texture_format_t _format, u32 _textureFlags);
BGFX_C_API bgfx_frame_buffer_handle_t bgfx_create_frame_buffer_from_handles(u8 _num, const bgfx_texture_handle_t* _handles, bool _destroyTextures);
BGFX_C_API bgfx_frame_buffer_handle_t bgfx_create_frame_buffer_from_attachment(u8 _num, const bgfx_attachment_t* _attachment, bool _destroyTextures);
BGFX_C_API bgfx_frame_buffer_handle_t bgfx_create_frame_buffer_from_nwh(void* _nwh, u16 _width, u16 _height, bgfx_texture_format_t _depthFormat);
BGFX_C_API bgfx_texture_handle_t bgfx_get_texture(bgfx_frame_buffer_handle_t _handle, u8 _attachment);
BGFX_C_API void bgfx_destroy_frame_buffer(bgfx_frame_buffer_handle_t _handle);
BGFX_C_API bgfx_uniform_handle_t bgfx_create_uniform(const char* _name, bgfx_uniform_type_t _type, u16 _num);
BGFX_C_API void bgfx_destroy_uniform(bgfx_uniform_handle_t _handle);
BGFX_C_API bgfx_occlusion_query_handle_t bgfx_create_occlusion_query(void);
BGFX_C_API bgfx_occlusion_query_result_t bgfx_get_result(bgfx_occlusion_query_handle_t _handle, i32* _result);
BGFX_C_API void bgfx_destroy_occlusion_query(bgfx_occlusion_query_handle_t _handle);
BGFX_C_API void bgfx_set_palette_color(u8 _index, const float _rgba[4]);
BGFX_C_API void bgfx_set_view_name(bgfx_view_id_t _id, const char* _name);
BGFX_C_API void bgfx_set_view_rect(bgfx_view_id_t _id, u16 _x, u16 _y, u16 _width, u16 _height);
BGFX_C_API void bgfx_set_view_rect_auto(bgfx_view_id_t _id, u16 _x, u16 _y, bgfx_backbuffer_ratio_t _ratio);
BGFX_C_API void bgfx_set_view_scissor(bgfx_view_id_t _id, u16 _x, u16 _y, u16 _width, u16 _height);
BGFX_C_API void bgfx_set_view_clear(bgfx_view_id_t _id, u16 _flags, u32 _rgba, float _depth, u8 _stencil);
BGFX_C_API void bgfx_set_view_clear_mrt(bgfx_view_id_t _id, u16 _flags, float _depth, u8 _stencil, u8 _0, u8 _1, u8 _2, u8 _3, u8 _4, u8 _5, u8 _6, u8 _7);
BGFX_C_API void bgfx_set_view_mode(bgfx_view_id_t _id, bgfx_view_mode_t _mode);
BGFX_C_API void bgfx_set_view_frame_buffer(bgfx_view_id_t _id, bgfx_frame_buffer_handle_t _handle);
BGFX_C_API void bgfx_set_view_transform(bgfx_view_id_t _id, const void* _view, const void* _proj);
BGFX_C_API void bgfx_set_view_transform_stereo(bgfx_view_id_t _id, const void* _view, const void* _projL, u8 _flags, const void* _projR);
BGFX_C_API void bgfx_set_view_order(bgfx_view_id_t _id, u16 _num, const bgfx_view_id_t* _order);
BGFX_C_API void bgfx_reset_view(bgfx_view_id_t _id);
BGFX_C_API void bgfx_set_marker(const char* _marker);
BGFX_C_API void bgfx_set_state(u64 _state, u32 _rgba);
BGFX_C_API void bgfx_set_condition(bgfx_occlusion_query_handle_t _handle, bool _visible);
BGFX_C_API void bgfx_set_stencil(u32 _fstencil, u32 _bstencil);
BGFX_C_API u16 bgfx_set_scissor(u16 _x, u16 _y, u16 _width, u16 _height);
BGFX_C_API void bgfx_set_scissor_cached(u16 _cache);
BGFX_C_API u32 bgfx_set_transform(const void* _mtx, u16 _num);
BGFX_C_API u32 bgfx_alloc_transform(bgfx_transform_t* _transform, u16 _num);
BGFX_C_API void bgfx_set_transform_cached(u32 _cache, u16 _num);
BGFX_C_API void bgfx_set_uniform(bgfx_uniform_handle_t _handle, const void* _value, u16 _num);
BGFX_C_API void bgfx_set_index_buffer(bgfx_index_buffer_handle_t _handle, u32 _firstIndex, u32 _numIndices);
BGFX_C_API void bgfx_set_dynamic_index_buffer(bgfx_dynamic_index_buffer_handle_t _handle, u32 _firstIndex, u32 _numIndices);
BGFX_C_API void bgfx_set_transient_index_buffer(const bgfx_transient_index_buffer_t* _tib, u32 _firstIndex, u32 _numIndices);
BGFX_C_API void bgfx_set_vertex_buffer(u8 _stream, bgfx_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _numVertices);
BGFX_C_API void bgfx_set_dynamic_vertex_buffer(u8 _stream, bgfx_dynamic_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _numVertices);
BGFX_C_API void bgfx_set_transient_vertex_buffer(u8 _stream, const bgfx_transient_vertex_buffer_t* _tvb, u32 _startVertex, u32 _numVertices);
BGFX_C_API void bgfx_set_vertex_count(u32 _numVertices);
BGFX_C_API void bgfx_set_instance_data_buffer(const bgfx_instance_data_buffer_t* _idb, u32 _start, u32 _num);
BGFX_C_API void bgfx_set_instance_data_from_vertex_buffer(bgfx_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _num);
BGFX_C_API void bgfx_set_instance_data_from_dynamic_vertex_buffer(bgfx_dynamic_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _num);
BGFX_C_API void bgfx_set_texture(u8 _stage, bgfx_uniform_handle_t _sampler, bgfx_texture_handle_t _handle, u32 _flags);
BGFX_C_API void bgfx_touch(bgfx_view_id_t _id);
BGFX_C_API void bgfx_submit(bgfx_view_id_t _id, bgfx_program_handle_t _handle, i32 _depth, bool _preserveState);
BGFX_C_API void bgfx_submit_occlusion_query(bgfx_view_id_t _id, bgfx_program_handle_t _program, bgfx_occlusion_query_handle_t _occlusionQuery, i32 _depth, bool _preserveState);
BGFX_C_API void bgfx_submit_indirect(bgfx_view_id_t _id, bgfx_program_handle_t _handle, bgfx_indirect_buffer_handle_t _indirectHandle, u16 _start, u16 _num, i32 _depth, bool _preserveState);
BGFX_C_API void bgfx_set_image(u8 _stage, bgfx_texture_handle_t _handle, u8 _mip, bgfx_access_t _access, bgfx_texture_format_t _format);
BGFX_C_API void bgfx_set_compute_index_buffer(u8 _stage, bgfx_index_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_set_compute_vertex_buffer(u8 _stage, bgfx_vertex_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_set_compute_dynamic_index_buffer(u8 _stage, bgfx_dynamic_index_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_set_compute_dynamic_vertex_buffer(u8 _stage, bgfx_dynamic_vertex_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_set_compute_indirect_buffer(u8 _stage, bgfx_indirect_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_dispatch(bgfx_view_id_t _id, bgfx_program_handle_t _handle, u32 _numX, u32 _numY, u32 _numZ, u8 _flags);
BGFX_C_API void bgfx_dispatch_indirect(bgfx_view_id_t _id, bgfx_program_handle_t _handle, bgfx_indirect_buffer_handle_t _indirectHandle, u16 _start, u16 _num, u8 _flags);
BGFX_C_API void bgfx_discard(void);
BGFX_C_API void bgfx_blit(bgfx_view_id_t _id, bgfx_texture_handle_t _dst, u8 _dstMip, u16 _dstX, u16 _dstY, u16 _dstZ, bgfx_texture_handle_t _src, u8 _srcMip, u16 _srcX, u16 _srcY, u16 _srcZ, u16 _width, u16 _height, u16 _depth);
BGFX_C_API void bgfx_encoder_set_marker(struct bgfx_encoder_s* _encoder, const char* _marker);
BGFX_C_API void bgfx_encoder_set_state(struct bgfx_encoder_s* _encoder, u64 _state, u32 _rgba);
BGFX_C_API void bgfx_encoder_set_condition(struct bgfx_encoder_s* _encoder, bgfx_occlusion_query_handle_t _handle, bool _visible);
BGFX_C_API void bgfx_encoder_set_stencil(struct bgfx_encoder_s* _encoder, u32 _fstencil, u32 _bstencil);
BGFX_C_API u16 bgfx_encoder_set_scissor(struct bgfx_encoder_s* _encoder, u16 _x, u16 _y, u16 _width, u16 _height);
BGFX_C_API void bgfx_encoder_set_scissor_cached(struct bgfx_encoder_s* _encoder, u16 _cache);
BGFX_C_API u32 bgfx_encoder_set_transform(struct bgfx_encoder_s* _encoder, const void* _mtx, u16 _num);
BGFX_C_API u32 bgfx_encoder_alloc_transform(struct bgfx_encoder_s* _encoder, bgfx_transform_t* _transform, u16 _num);
BGFX_C_API void bgfx_encoder_set_transform_cached(struct bgfx_encoder_s* _encoder, u32 _cache, u16 _num);
BGFX_C_API void bgfx_encoder_set_uniform(struct bgfx_encoder_s* _encoder, bgfx_uniform_handle_t _handle, const void* _value, u16 _num);
BGFX_C_API void bgfx_encoder_set_index_buffer(struct bgfx_encoder_s* _encoder, bgfx_index_buffer_handle_t _handle, u32 _firstIndex, u32 _numIndices);
BGFX_C_API void bgfx_encoder_set_dynamic_index_buffer(struct bgfx_encoder_s* _encoder, bgfx_dynamic_index_buffer_handle_t _handle, u32 _firstIndex, u32 _numIndices);
BGFX_C_API void bgfx_encoder_set_transient_index_buffer(struct bgfx_encoder_s* _encoder, const bgfx_transient_index_buffer_t* _tib, u32 _firstIndex, u32 _numIndices);
BGFX_C_API void bgfx_encoder_set_vertex_buffer(struct bgfx_encoder_s* _encoder, u8 _stream, bgfx_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _numVertices);
BGFX_C_API void bgfx_encoder_set_dynamic_vertex_buffer(struct bgfx_encoder_s* _encoder, u8 _stream, bgfx_dynamic_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _numVertices);
BGFX_C_API void bgfx_encoder_set_transient_vertex_buffer(struct bgfx_encoder_s* _encoder, u8 _stream, const bgfx_transient_vertex_buffer_t* _tvb, u32 _startVertex, u32 _numVertices);
BGFX_C_API void bgfx_encoder_set_vertex_count(struct bgfx_encoder_s* _encoder, u32 _numVertices);
BGFX_C_API void bgfx_encoder_set_instance_data_buffer(struct bgfx_encoder_s* _encoder, const bgfx_instance_data_buffer_t* _idb, u32 _start, u32 _num);
BGFX_C_API void bgfx_encoder_set_instance_data_from_vertex_buffer(struct bgfx_encoder_s* _encoder, bgfx_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _num);
BGFX_C_API void bgfx_encoder_set_instance_data_from_dynamic_vertex_buffer(struct bgfx_encoder_s* _encoder, bgfx_dynamic_vertex_buffer_handle_t _handle, u32 _startVertex, u32 _num);
BGFX_C_API void bgfx_encoder_set_texture(struct bgfx_encoder_s* _encoder, u8 _stage, bgfx_uniform_handle_t _sampler, bgfx_texture_handle_t _handle, u32 _flags);
BGFX_C_API void bgfx_encoder_touch(struct bgfx_encoder_s* _encoder, bgfx_view_id_t _id);
BGFX_C_API void bgfx_encoder_submit(struct bgfx_encoder_s* _encoder, bgfx_view_id_t _id, bgfx_program_handle_t _handle, i32 _depth, bool _preserveState);
BGFX_C_API void bgfx_encoder_submit_occlusion_query(struct bgfx_encoder_s* _encoder, bgfx_view_id_t _id, bgfx_program_handle_t _program, bgfx_occlusion_query_handle_t _occlusionQuery, i32 _depth, bool _preserveState);
BGFX_C_API void bgfx_encoder_submit_indirect(struct bgfx_encoder_s* _encoder, bgfx_view_id_t _id, bgfx_program_handle_t _handle, bgfx_indirect_buffer_handle_t _indirectHandle, u16 _start, u16 _num, i32 _depth, bool _preserveState);
BGFX_C_API void bgfx_encoder_set_image(struct bgfx_encoder_s* _encoder, u8 _stage, bgfx_texture_handle_t _handle, u8 _mip, bgfx_access_t _access, bgfx_texture_format_t _format);
BGFX_C_API void bgfx_encoder_set_compute_index_buffer(struct bgfx_encoder_s* _encoder, u8 _stage, bgfx_index_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_encoder_set_compute_vertex_buffer(struct bgfx_encoder_s* _encoder, u8 _stage, bgfx_vertex_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_encoder_set_compute_dynamic_index_buffer(struct bgfx_encoder_s* _encoder, u8 _stage, bgfx_dynamic_index_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_encoder_set_compute_dynamic_vertex_buffer(struct bgfx_encoder_s* _encoder, u8 _stage, bgfx_dynamic_vertex_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_encoder_set_compute_indirect_buffer(struct bgfx_encoder_s* _encoder, u8 _stage, bgfx_indirect_buffer_handle_t _handle, bgfx_access_t _access);
BGFX_C_API void bgfx_encoder_dispatch(struct bgfx_encoder_s* _encoder, bgfx_view_id_t _id, bgfx_program_handle_t _handle, u32 _numX, u32 _numY, u32 _numZ, u8 _flags);
BGFX_C_API void bgfx_encoder_dispatch_indirect(struct bgfx_encoder_s* _encoder, bgfx_view_id_t _id, bgfx_program_handle_t _handle, bgfx_indirect_buffer_handle_t _indirectHandle, u16 _start, u16 _num, u8 _flags);
BGFX_C_API void bgfx_encoder_discard(struct bgfx_encoder_s* _encoder);
BGFX_C_API void bgfx_encoder_blit(struct bgfx_encoder_s* _encoder, bgfx_view_id_t _id, bgfx_texture_handle_t _dst, u8 _dstMip, u16 _dstX, u16 _dstY, u16 _dstZ, bgfx_texture_handle_t _src, u8 _srcMip, u16 _srcX, u16 _srcY, u16 _srcZ, u16 _width, u16 _height, u16 _depth);
BGFX_C_API void bgfx_request_screen_shot(bgfx_frame_buffer_handle_t _handle, const char* _filePath);
*/

