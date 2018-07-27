#![allow(non_snake_case)]

pub const API_VERSION: u32 = 76;

// Color RGB/alpha/depth write. When it's not specified write will be disabled.
pub const STATE_WRITE_R: u64 = 0x0000000000000001; // Enable R write.
pub const STATE_WRITE_G: u64 = 0x0000000000000002; // Enable G write.
pub const STATE_WRITE_B: u64 = 0x0000000000000004; // Enable B write.
pub const STATE_WRITE_A: u64 = 0x0000000000000008; // Enable alpha write.
pub const STATE_WRITE_Z: u64 = 0x0000004000000000; // Enable depth write.

// Enable RGB write.
pub const STATE_WRITE_RGB: u64 = (0 | STATE_WRITE_R | STATE_WRITE_G | STATE_WRITE_B );

// Write all channels mask.
pub const STATE_WRITE_MASK: u64 = (0 | STATE_WRITE_RGB | STATE_WRITE_A | STATE_WRITE_Z );

// Depth test state. When `STATE_DEPTH_` is not specified depth test will be disabled.
pub const STATE_DEPTH_TEST_LESS:u64     = 0x0000000000000010; // Enable depth test, less.
pub const STATE_DEPTH_TEST_LEQUAL:u64   = 0x0000000000000020; // Enable depth test, less or equal.
pub const STATE_DEPTH_TEST_EQUAL:u64    = 0x0000000000000030; // Enable depth test, equal.
pub const STATE_DEPTH_TEST_GEQUAL:u64   = 0x0000000000000040; // Enable depth test, greater or equal.
pub const STATE_DEPTH_TEST_GREATER:u64  = 0x0000000000000050; // Enable depth test, greater.
pub const STATE_DEPTH_TEST_NOTEQUAL:u64 = 0x0000000000000060; // Enable depth test, not equal.
pub const STATE_DEPTH_TEST_NEVER:u64    = 0x0000000000000070; // Enable depth test, never.
pub const STATE_DEPTH_TEST_ALWAYS:u64   = 0x0000000000000080; // Enable depth test, always.
pub const STATE_DEPTH_TEST_SHIFT:u64    = 4;                  // Depth test state bit shift.
pub const STATE_DEPTH_TEST_MASK:u64     = 0x00000000000000f0; // Depth test state bit mask.

// Use STATE_BLEND_FUNC(_src, _dst) or STATE_BLEND_FUNC_SEPARATE(_srcRGB, _dstRGB, _srcA, _dstA)
// helper macros.
pub const STATE_BLEND_ZERO: u64              = 0x0000000000001000; // 0, 0, 0, 0
pub const STATE_BLEND_ONE: u64               = 0x0000000000002000; // 1, 1, 1, 1
pub const STATE_BLEND_SRC_COLOR: u64         = 0x0000000000003000; // Rs, Gs, Bs, As
pub const STATE_BLEND_INV_SRC_COLOR: u64     = 0x0000000000004000; // 1-Rs, 1-Gs, 1-Bs, 1-As
pub const STATE_BLEND_SRC_ALPHA: u64         = 0x0000000000005000; // As, As, As, As
pub const STATE_BLEND_INV_SRC_ALPHA: u64     = 0x0000000000006000; // 1-As, 1-As, 1-As, 1-As
pub const STATE_BLEND_DST_ALPHA: u64         = 0x0000000000007000; // Ad, Ad, Ad, Ad
pub const STATE_BLEND_INV_DST_ALPHA: u64     = 0x0000000000008000; // 1-Ad, 1-Ad, 1-Ad ,1-Ad
pub const STATE_BLEND_DST_COLOR: u64         = 0x0000000000009000; // Rd, Gd, Bd, Ad
pub const STATE_BLEND_INV_DST_COLOR: u64     = 0x000000000000a000; // 1-Rd, 1-Gd, 1-Bd, 1-Ad
pub const STATE_BLEND_SRC_ALPHA_SAT: u64     = 0x000000000000b000; // f, f, f, 1; f = min(As, 1-Ad)
pub const STATE_BLEND_FACTOR: u64            = 0x000000000000c000; // Blend factor
pub const STATE_BLEND_INV_FACTOR: u64        = 0x000000000000d000; // 1-Blend factor
pub const STATE_BLEND_SHIFT: u64             = 12;                 // Blend state bit shift.
pub const STATE_BLEND_MASK: u64              = 0x000000000ffff000; // Blend state bit mask.

// Use STATE_BLEND_EQUATION(_equation) or STATE_BLEND_EQUATION_SEPARATE(_equationRGB, _equationA)
// helper macros.
pub const STATE_BLEND_EQUATION_ADD: u64      = 0x0000000000000000; // Blend add: src + dst.
pub const STATE_BLEND_EQUATION_SUB: u64      = 0x0000000010000000; // Blend subtract: src - dst.
pub const STATE_BLEND_EQUATION_REVSUB: u64   = 0x0000000020000000; // Blend reverse subtract: dst - src.
pub const STATE_BLEND_EQUATION_MIN: u64      = 0x0000000030000000; // Blend min: min(src, dst).
pub const STATE_BLEND_EQUATION_MAX: u64      = 0x0000000040000000; // Blend max: max(src, dst).
pub const STATE_BLEND_EQUATION_SHIFT: u64    = 28;                 // Blend equation bit shift.
pub const STATE_BLEND_EQUATION_MASK: u64     = 0x00000003f0000000; // Blend equation bit mask.

pub const STATE_BLEND_INDEPENDENT: u64       = 0x0000000400000000; // Enable blend independent.
pub const STATE_BLEND_ALPHA_TO_COVERAGE: u64 = 0x0000000800000000; // Enable alpha to coverage.

// Cull state. When `STATE_CULL_*` is not specified culling will be disabled.
pub const STATE_CULL_CW: u64                 = 0x0000001000000000; // Cull clockwise triangles.
pub const STATE_CULL_CCW: u64                = 0x0000002000000000; // Cull counter-clockwise triangles.
pub const STATE_CULL_SHIFT: u64              = 36;                 // Culling mode bit shift.
pub const STATE_CULL_MASK: u64               = 0x0000003000000000; // Culling mode bit mask.

// See STATE_ALPHA_REF(_ref) helper macro.
pub const STATE_ALPHA_REF_SHIFT: u64         = 40;                 // Alpha reference bit shift.
pub const STATE_ALPHA_REF_MASK: u64          = 0x0000ff0000000000; // Alpha reference bit mask.

pub const STATE_PT_TRISTRIP: u64             = 0x0001000000000000; // Tristrip.
pub const STATE_PT_LINES: u64                = 0x0002000000000000; // Lines.
pub const STATE_PT_LINESTRIP: u64            = 0x0003000000000000; // Line strip.
pub const STATE_PT_POINTS: u64               = 0x0004000000000000; // Points.
pub const STATE_PT_SHIFT: u64                = 48;                 // Primitive type bit shift.
pub const STATE_PT_MASK: u64                 = 0x0007000000000000; // Primitive type bit mask.

// See STATE_POINT_SIZE(_size) helper macro.
pub const STATE_POINT_SIZE_SHIFT: u64        = 52;                 // Point size bit shift.
pub const STATE_POINT_SIZE_MASK: u64         = 0x00f0000000000000; // Point size bit mask.

// Enable MSAA write when writing into MSAA frame buffer.
// This flag is ignored when not writing into MSAA frame buffer.
pub const STATE_MSAA: u64                    = 0x0100000000000000; // Enable MSAA rasterization.
pub const STATE_LINEAA: u64                  = 0x0200000000000000; // Enable line AA rasterization.
pub const STATE_CONSERVATIVE_RASTER: u64     = 0x0400000000000000; // Enable conservative rasterization.

// Do not use!
pub const STATE_RESERVED_SHIFT: u64          = 61;                 // Internal bits shift.
pub const STATE_RESERVED_MASK: u64           = 0xe000000000000000; // Internal bits mask.

pub const STATE_NONE: u64                    = 0x0000000000000000; // No state.
pub const STATE_MASK: u64                    = 0xffffffffffffffff; // State mask.

// Default state is write to RGB, alpha, and depth with depth test less enabled, with clockwise
// culling and MSAA (when writing into MSAA frame buffer, otherwise this flag is ignored).
pub const STATE_DEFAULT: u64 = (0 | STATE_WRITE_RGB | STATE_WRITE_A | STATE_WRITE_Z | STATE_DEPTH_TEST_LESS | STATE_CULL_CW | STATE_MSAA);

// Alpha reference value.
pub const fn STATE_ALPHA_REF(reff: u64) -> u64 {
    return (reff << STATE_ALPHA_REF_SHIFT) & STATE_ALPHA_REF_MASK;
}

// Point size value.
pub const fn STATE_POINT_SIZE(size: u64) -> u64 {
    return (size << STATE_POINT_SIZE_SHIFT) & STATE_POINT_SIZE_MASK
}

// Blend function separate.
pub const fn STATE_BLEND_FUNC_SEPARATE(srcRGB: u64, dstRGB: u64, srcA: u64, dstA: u64) -> u64 {
    return 0 | ((srcRGB | (dstRGB << 4)) | ((srcA | (dstA << 4)) << 8));
}

// Blend equation separate.
pub const fn STATE_BLEND_EQUATION_SEPARATE(equationRGB: u64, equationA: u64) -> u64 {
    return equationRGB | (equationA) << 3;
}

// Blend function.
pub const fn STATE_BLEND_FUNC(src: u64, dst: u64) -> u64 {
    return STATE_BLEND_FUNC_SEPARATE(src, dst, src, dst);
}

// Blend equation.
pub const fn STATE_BLEND_EQUATION(equation: u64) -> u64 {
    return STATE_BLEND_EQUATION_SEPARATE(equation, equation);
}

// Utility predefined blend modes.

// Additive blending.
pub const STATE_BLEND_ADD: u64         = (0 | STATE_BLEND_FUNC(STATE_BLEND_ONE, STATE_BLEND_ONE));                                                                           // Alpha blend.
pub const STATE_BLEND_ALPHA: u64       = (0 | STATE_BLEND_FUNC(STATE_BLEND_SRC_ALPHA, STATE_BLEND_INV_SRC_ALPHA));                                                           // Selects darker color of blend.
pub const STATE_BLEND_DARKEN: u64      = (0 | STATE_BLEND_FUNC(STATE_BLEND_ONE, STATE_BLEND_ONE)| STATE_BLEND_EQUATION(STATE_BLEND_EQUATION_MIN));                 // Selects lighter color of blend.
pub const STATE_BLEND_LIGHTEN: u64     = (0 | STATE_BLEND_FUNC(STATE_BLEND_ONE, STATE_BLEND_ONE)| STATE_BLEND_EQUATION(STATE_BLEND_EQUATION_MAX));                 // Multiplies colors.
pub const STATE_BLEND_MULTIPLY: u64    = (0 | STATE_BLEND_FUNC(STATE_BLEND_DST_COLOR, STATE_BLEND_ZERO));                                                                    // Opaque pixels will cover the pixels directly below them without any math or algorithm applied to them.
pub const STATE_BLEND_NORMAL: u64      = (0 | STATE_BLEND_FUNC(STATE_BLEND_ONE, STATE_BLEND_INV_SRC_ALPHA));                                                                 // Multiplies the inverse of the blend and base colors.
pub const STATE_BLEND_SCREEN: u64      = (0 | STATE_BLEND_FUNC(STATE_BLEND_ONE, STATE_BLEND_INV_SRC_COLOR));                                                                 // Decreases the brightness of the base color based on the value of the blend color.
pub const STATE_BLEND_LINEAR_BURN: u64 = (0 | STATE_BLEND_FUNC(STATE_BLEND_DST_COLOR, STATE_BLEND_INV_DST_COLOR)| STATE_BLEND_EQUATION(STATE_BLEND_EQUATION_SUB)); //

pub const fn STATE_BLEND_FUNC_RT_x(src: u64, dst: u64) -> u64 {
    return 0 | ((src >> STATE_BLEND_SHIFT) | ((dst >> STATE_BLEND_SHIFT) << 4));
}

pub const fn STATE_BLEND_FUNC_RT_xE(src: u64, dst: u64, equation: u64) -> u64 {
    return 0 | STATE_BLEND_FUNC_RT_x(src, dst) | ((equation >> STATE_BLEND_EQUATION_SHIFT) << 8);
}

pub const fn STATE_BLEND_FUNC_RT_1(src: u64, dst: u64) -> u64 {
    return STATE_BLEND_FUNC_RT_x(src, dst) <<  0;
}

pub const fn STATE_BLEND_FUNC_RT_2(src: u64, dst: u64) -> u64 {
    return STATE_BLEND_FUNC_RT_x(src, dst) << 11;
}

pub const fn STATE_BLEND_FUNC_RT_3(src: u64, dst: u64) -> u64 {
    return STATE_BLEND_FUNC_RT_x(src, dst) << 22;
}

pub const fn STATE_BLEND_FUNC_RT_1E(src: u64, dst: u64, equation: u64) -> u64 {
    return STATE_BLEND_FUNC_RT_xE(src, dst, equation) <<  0;
}

pub const fn STATE_BLEND_FUNC_RT_2E(src: u64, dst: u64, equation: u64) -> u64 {
    return STATE_BLEND_FUNC_RT_xE(src, dst, equation) << 11;
}

pub const fn STATE_BLEND_FUNC_RT_3E(src: u64, dst: u64, equation: u64) -> u64 {
    return STATE_BLEND_FUNC_RT_xE(src, dst, equation) << 22;
}

//
pub const STENCIL_FUNC_REF_SHIFT: u32 =      0;                    //
pub const STENCIL_FUNC_REF_MASK: u32 =       0x000000ff; //
pub const STENCIL_FUNC_RMASK_SHIFT: u32 =    8;                    //
pub const STENCIL_FUNC_RMASK_MASK: u32 =     0x0000ff00; //

pub const STENCIL_TEST_LESS: u32 =           0x00010000; // Enable stencil test, less.
pub const STENCIL_TEST_LEQUAL: u32 =         0x00020000; // Enable stencil test, less or equal.
pub const STENCIL_TEST_EQUAL: u32 =          0x00030000; // Enable stencil test, equal.
pub const STENCIL_TEST_GEQUAL: u32 =         0x00040000; // Enable stencil test, greater or equal.
pub const STENCIL_TEST_GREATER: u32 =        0x00050000; // Enable stencil test, greater.
pub const STENCIL_TEST_NOTEQUAL: u32 =       0x00060000; // Enable stencil test, not equal.
pub const STENCIL_TEST_NEVER: u32 =          0x00070000; // Enable stencil test, never.
pub const STENCIL_TEST_ALWAYS: u32 =         0x00080000; // Enable stencil test, always.
pub const STENCIL_TEST_SHIFT: u32 =          16;                   // Stencil test bit shift.
pub const STENCIL_TEST_MASK: u32 =           0x000f0000; // Stencil test bit mask.

pub const STENCIL_OP_FAIL_S_ZERO: u32 =      0x00000000; // Zero.
pub const STENCIL_OP_FAIL_S_KEEP: u32 =      0x00100000; // Keep.
pub const STENCIL_OP_FAIL_S_REPLACE: u32 =   0x00200000; // Replace.
pub const STENCIL_OP_FAIL_S_INCR: u32 =      0x00300000; // Increment and wrap.
pub const STENCIL_OP_FAIL_S_INCRSAT: u32 =   0x00400000; // Increment and clamp.
pub const STENCIL_OP_FAIL_S_DECR: u32 =      0x00500000; // Decrement and wrap.
pub const STENCIL_OP_FAIL_S_DECRSAT: u32 =   0x00600000; // Decrement and clamp.
pub const STENCIL_OP_FAIL_S_INVERT: u32 =    0x00700000; // Invert.
pub const STENCIL_OP_FAIL_S_SHIFT: u32 =     20;                   // Stencil operation fail bit shift.
pub const STENCIL_OP_FAIL_S_MASK: u32 =      0x00f00000; // Stencil operation fail bit mask.

pub const STENCIL_OP_FAIL_Z_ZERO: u32 =      0x00000000; // Zero.
pub const STENCIL_OP_FAIL_Z_KEEP: u32 =      0x01000000; // Keep.
pub const STENCIL_OP_FAIL_Z_REPLACE: u32 =   0x02000000; // Replace.
pub const STENCIL_OP_FAIL_Z_INCR: u32 =      0x03000000; // Increment and wrap.
pub const STENCIL_OP_FAIL_Z_INCRSAT: u32 =   0x04000000; // Increment and clamp.
pub const STENCIL_OP_FAIL_Z_DECR: u32 =      0x05000000; // Decrement and wrap.
pub const STENCIL_OP_FAIL_Z_DECRSAT: u32 =   0x06000000; // Decrement and clamp.
pub const STENCIL_OP_FAIL_Z_INVERT: u32 =    0x07000000; // Invert.
pub const STENCIL_OP_FAIL_Z_SHIFT: u32 =     24;                   // Stencil operation depth fail bit shift
pub const STENCIL_OP_FAIL_Z_MASK: u32 =      0x0f000000; // Stencil operation depth fail bit mask.

pub const STENCIL_OP_PASS_Z_ZERO: u32 =      0x00000000; // Zero.
pub const STENCIL_OP_PASS_Z_KEEP: u32 =      0x10000000; // Keep.
pub const STENCIL_OP_PASS_Z_REPLACE: u32 =   0x20000000; // Replace.
pub const STENCIL_OP_PASS_Z_INCR: u32 =      0x30000000; // Increment and wrap.
pub const STENCIL_OP_PASS_Z_INCRSAT: u32 =   0x40000000; // Increment and clamp.
pub const STENCIL_OP_PASS_Z_DECR: u32 =      0x50000000; // Decrement and wrap.
pub const STENCIL_OP_PASS_Z_DECRSAT: u32 =   0x60000000; // Decrement and clamp.
pub const STENCIL_OP_PASS_Z_INVERT: u32 =    0x70000000; // Invert.
pub const STENCIL_OP_PASS_Z_SHIFT: u32 =     28;                   // Stencil operation depth pass bit shift
pub const STENCIL_OP_PASS_Z_MASK: u32 =      0xf0000000; // Stencil operation depth pass bit mask.

pub const STENCIL_NONE: u32 =                0x00000000; //
pub const STENCIL_MASK: u32 =                0xffffffff; //
pub const STENCIL_DEFAULT: u32 =             0x00000000; //

// Set stencil ref value.
pub const fn STENCIL_FUNC_REF(reff: u32) -> u32 {
    return (reff << STENCIL_FUNC_REF_SHIFT) & STENCIL_FUNC_REF_MASK;
}

// Set stencil rmask value.
pub const fn STENCIL_FUNC_RMASK(mask: u32) -> u32 {
    return (mask << STENCIL_FUNC_RMASK_SHIFT) & STENCIL_FUNC_RMASK_MASK;
}

//
pub const CLEAR_NONE: u32 =                  0x0000; // No clear flags.
pub const CLEAR_COLOR: u32 =                 0x0001; // Clear color.
pub const CLEAR_DEPTH: u32 =                 0x0002; // Clear depth.
pub const CLEAR_STENCIL: u32 =               0x0004; // Clear stencil.
pub const CLEAR_DISCARD_COLOR_0: u32 =       0x0008; // Discard frame buffer attachment 0.
pub const CLEAR_DISCARD_COLOR_1: u32 =       0x0010; // Discard frame buffer attachment 1.
pub const CLEAR_DISCARD_COLOR_2: u32 =       0x0020; // Discard frame buffer attachment 2.
pub const CLEAR_DISCARD_COLOR_3: u32 =       0x0040; // Discard frame buffer attachment 3.
pub const CLEAR_DISCARD_COLOR_4: u32 =       0x0080; // Discard frame buffer attachment 4.
pub const CLEAR_DISCARD_COLOR_5: u32 =       0x0100; // Discard frame buffer attachment 5.
pub const CLEAR_DISCARD_COLOR_6: u32 =       0x0200; // Discard frame buffer attachment 6.
pub const CLEAR_DISCARD_COLOR_7: u32 =       0x0400; // Discard frame buffer attachment 7.
pub const CLEAR_DISCARD_DEPTH: u32 =         0x0800; // Discard frame buffer depth attachment.
pub const CLEAR_DISCARD_STENCIL: u32 =       0x1000; // Discard frame buffer stencil attachment.

pub const CLEAR_DISCARD_COLOR_MASK: u32 = (0 | CLEAR_DISCARD_COLOR_0    | CLEAR_DISCARD_COLOR_1 | CLEAR_DISCARD_COLOR_2 | CLEAR_DISCARD_COLOR_3 | CLEAR_DISCARD_COLOR_4 | CLEAR_DISCARD_COLOR_5 | CLEAR_DISCARD_COLOR_6 | CLEAR_DISCARD_COLOR_7);
pub const CLEAR_DISCARD_MASK: u32       = (0 | CLEAR_DISCARD_COLOR_MASK | CLEAR_DISCARD_DEPTH   | CLEAR_DISCARD_STENCIL);

//
pub const DEBUG_NONE: u32 =                  0x00000000; // No debug.
pub const DEBUG_WIREFRAME: u32 =             0x00000001; // Enable wireframe for all primitives.
pub const DEBUG_IFH: u32 =                   0x00000002; // Enable infinitely fast hardware test. No draw calls will be submitted to driver. It’s useful when profiling to quickly assess bottleneck between CPU and GPU.
pub const DEBUG_STATS: u32 =                 0x00000004; // Enable statistics display.
pub const DEBUG_TEXT: u32 =                  0x00000008; // Enable debug text display.
pub const DEBUG_PROFILER: u32 =              0x00000010; // Enable profiler.

//
pub const BUFFER_NONE: u16 =                 0x0000; //

pub const BUFFER_COMPUTE_FORMAT_8X1: u16 =   0x0001; // 1 8-bit value
pub const BUFFER_COMPUTE_FORMAT_8X2: u16 =   0x0002; // 2 8-bit values
pub const BUFFER_COMPUTE_FORMAT_8X4: u16 =   0x0003; // 4 8-bit values
pub const BUFFER_COMPUTE_FORMAT_16X1: u16 =  0x0004; // 1 16-bit value
pub const BUFFER_COMPUTE_FORMAT_16X2: u16 =  0x0005; // 2 16-bit values
pub const BUFFER_COMPUTE_FORMAT_16X4: u16 =  0x0006; // 4 16-bit values
pub const BUFFER_COMPUTE_FORMAT_32X1: u16 =  0x0007; // 1 32-bit value
pub const BUFFER_COMPUTE_FORMAT_32X2: u16 =  0x0008; // 2 32-bit values
pub const BUFFER_COMPUTE_FORMAT_32X4: u16 =  0x0009; // 4 32-bit values
pub const BUFFER_COMPUTE_FORMAT_SHIFT: u16 = 0;                //
pub const BUFFER_COMPUTE_FORMAT_MASK: u16 =  0x000f; //

pub const BUFFER_COMPUTE_TYPE_INT: u16 =     0x0010; // Type `int`.
pub const BUFFER_COMPUTE_TYPE_UINT: u16 =    0x0020; // Type `uint`.
pub const BUFFER_COMPUTE_TYPE_FLOAT: u16 =   0x0030; // Type `float`.
pub const BUFFER_COMPUTE_TYPE_SHIFT: u16 =   4;                //
pub const BUFFER_COMPUTE_TYPE_MASK: u16 =    0x0030; //

pub const BUFFER_COMPUTE_READ: u16 =         0x0100; // Buffer will be read by shader.
pub const BUFFER_COMPUTE_WRITE: u16 =        0x0200; // Buffer will be used for writing.
pub const BUFFER_DRAW_INDIRECT: u16 =        0x0400; // Buffer will be used for storing draw indirect commands.
pub const BUFFER_ALLOW_RESIZE: u16 =         0x0800; // Allow dynamic index/vertex buffer resize during update.
pub const BUFFER_INDEX32: u16 =              0x1000; // Index buffer contains 32-bit indices.

pub const GFX_BUFFER_COMPUTE_READ_WRITE: u16 = (0 | BUFFER_COMPUTE_READ | BUFFER_COMPUTE_WRITE);

//
pub const TEXTURE_NONE: u32 =                0x00000000;  //
pub const TEXTURE_U_MIRROR: u32 =            0x00000001;  // Wrap U mode: Mirror
pub const TEXTURE_U_CLAMP: u32 =             0x00000002;  // Wrap U mode: Clamp
pub const TEXTURE_U_BORDER: u32 =            0x00000003;  // Wrap U mode: Border
pub const TEXTURE_U_SHIFT: u32 =             0;           //
pub const TEXTURE_U_MASK: u32 =              0x00000003;  //
pub const TEXTURE_V_MIRROR: u32 =            0x00000004;  // Wrap V mode: Mirror
pub const TEXTURE_V_CLAMP: u32 =             0x00000008;  // Wrap V mode: Clamp
pub const TEXTURE_V_BORDER: u32 =            0x0000000c;  // Wrap V mode: Border
pub const TEXTURE_V_SHIFT: u32 =             2;           //
pub const TEXTURE_V_MASK: u32 =              0x0000000c;  //
pub const TEXTURE_W_MIRROR: u32 =            0x00000010;  // Wrap W mode: Mirror
pub const TEXTURE_W_CLAMP: u32 =             0x00000020;  // Wrap W mode: Clamp
pub const TEXTURE_W_BORDER: u32 =            0x00000030;  // Wrap W mode: Border
pub const TEXTURE_W_SHIFT: u32 =             4;           //
pub const TEXTURE_W_MASK: u32 =              0x00000030;  //
pub const TEXTURE_MIN_POINT: u32 =           0x00000040;  // Min sampling mode: Point
pub const TEXTURE_MIN_ANISOTROPIC: u32 =     0x00000080;  // Min sampling mode: Anisotropic
pub const TEXTURE_MIN_SHIFT: u32 =           6;           //
pub const TEXTURE_MIN_MASK: u32 =            0x000000c0;  //
pub const TEXTURE_MAG_POINT: u32 =           0x00000100;  // Mag sampling mode: Point
pub const TEXTURE_MAG_ANISOTROPIC: u32 =     0x00000200;  // Mag sampling mode: Anisotropic
pub const TEXTURE_MAG_SHIFT: u32 =           8;           //
pub const TEXTURE_MAG_MASK: u32 =            0x00000300;  //
pub const TEXTURE_MIP_POINT: u32 =           0x00000400;  // Mip sampling mode: Point
pub const TEXTURE_MIP_SHIFT: u32 =           10;          //
pub const TEXTURE_MIP_MASK: u32 =            0x00000400;  //
pub const TEXTURE_MSAA_SAMPLE: u32 =         0x00000800;  // Texture will be used for MSAA sampling.
pub const TEXTURE_RT: u32 =                  0x00001000;  //
pub const TEXTURE_RT_MSAA_X2: u32 =          0x00002000;  // Render target MSAAx2 mode.
pub const TEXTURE_RT_MSAA_X4: u32 =          0x00003000;  // Render target MSAAx4 mode.
pub const TEXTURE_RT_MSAA_X8: u32 =          0x00004000;  // Render target MSAAx8 mode.
pub const TEXTURE_RT_MSAA_X16: u32 =         0x00005000;  // Render target MSAAx16 mode.
pub const TEXTURE_RT_MSAA_SHIFT: u32 =       12;          //
pub const TEXTURE_RT_MSAA_MASK: u32 =        0x00007000;  //
pub const TEXTURE_RT_WRITE_ONLY: u32 =       0x00008000;  // Render target will be used for writing only.
pub const TEXTURE_RT_MASK: u32 =             0x0000f000;  //
pub const TEXTURE_COMPARE_LESS: u32 =        0x00010000;  // Compare when sampling depth texture: less.
pub const TEXTURE_COMPARE_LEQUAL: u32 =      0x00020000;  // Compare when sampling depth texture: less or equal.
pub const TEXTURE_COMPARE_EQUAL: u32 =       0x00030000;  // Compare when sampling depth texture: equal.
pub const TEXTURE_COMPARE_GEQUAL: u32 =      0x00040000;  // Compare when sampling depth texture: greater or equal.
pub const TEXTURE_COMPARE_GREATER: u32 =     0x00050000;  // Compare when sampling depth texture: greater.
pub const TEXTURE_COMPARE_NOTEQUAL: u32 =    0x00060000;  // Compare when sampling depth texture: not equal.
pub const TEXTURE_COMPARE_NEVER: u32 =       0x00070000;  // Compare when sampling depth texture: never.
pub const TEXTURE_COMPARE_ALWAYS: u32 =      0x00080000;  // Compare when sampling depth texture: always.
pub const TEXTURE_COMPARE_SHIFT: u32 =       16;          //
pub const TEXTURE_COMPARE_MASK: u32 =        0x000f0000;  //
pub const TEXTURE_COMPUTE_WRITE: u32 =       0x00100000;  // Texture will be used for compute write.
pub const TEXTURE_SRGB: u32 =                0x00200000;  // Sample texture as sRGB.
pub const TEXTURE_BLIT_DST: u32 =            0x00400000;  // Texture will be used as blit destination.
pub const TEXTURE_READ_BACK: u32 =           0x00800000;  // Texture will be used for read back from GPU.
pub const TEXTURE_BORDER_COLOR_SHIFT: u32 =  24;          //
pub const TEXTURE_BORDER_COLOR_MASK: u32 =   0x0f000000;  //
pub const TEXTURE_RESERVED_SHIFT: u32 =      28;          //
pub const TEXTURE_RESERVED_MASK: u32 =       0xf0000000;  //

pub const fn TEXTURE_BORDER_COLOR(index: u32) -> u32 {
    return (index << TEXTURE_BORDER_COLOR_SHIFT) & TEXTURE_BORDER_COLOR_MASK;
}

pub const TEXTURE_SAMPLER_BITS_MASK: u32 = (0 | TEXTURE_U_MASK | TEXTURE_V_MASK | TEXTURE_W_MASK | TEXTURE_MIN_MASK | TEXTURE_MAG_MASK | TEXTURE_MIP_MASK | TEXTURE_COMPARE_MASK);

//
pub const RESET_NONE: u32 =                  0x00000000; // No reset flags.
pub const RESET_FULLSCREEN: u32 =            0x00000001; // Not supported yet.
pub const RESET_FULLSCREEN_SHIFT: u32 =      0;          // Fullscreen bit shift.
pub const RESET_FULLSCREEN_MASK: u32 =       0x00000001; // Fullscreen bit mask.
pub const RESET_MSAA_X2: u32 =               0x00000010; // Enable 2x MSAA.
pub const RESET_MSAA_X4: u32 =               0x00000020; // Enable 4x MSAA.
pub const RESET_MSAA_X8: u32 =               0x00000030; // Enable 8x MSAA.
pub const RESET_MSAA_X16: u32 =              0x00000040; // Enable 16x MSAA.
pub const RESET_MSAA_SHIFT: u32 =            4;          // MSAA mode bit shift.
pub const RESET_MSAA_MASK: u32 =             0x00000070; // MSAA mode bit mask.
pub const RESET_VSYNC: u32 =                 0x00000080; // Enable V-Sync.
pub const RESET_MAXANISOTROPY: u32 =         0x00000100; // Turn on/off max anisotropy.
pub const RESET_CAPTURE: u32 =               0x00000200; // Begin screen capture.
pub const RESET_FLUSH_AFTER_RENDER: u32 =    0x00002000; // Flush rendering after submitting to GPU.
pub const RESET_FLIP_AFTER_RENDER: u32 =     0x00004000; // This flag  specifies where flip occurs. Default behavior is that flip occurs before rendering new frame. This flag only has effect when `CONFIG_MULTITHREADED=0`.
pub const RESET_SRGB_BACKBUFFER: u32 =       0x00008000; // Enable sRGB backbuffer.
pub const RESET_HIDPI: u32 =                 0x00010000; // Enable HiDPI rendering.
pub const RESET_DEPTH_CLAMP: u32 =           0x00020000; // Enable depth clamp.
pub const RESET_SUSPEND: u32 =               0x00040000; // Suspend rendering.

pub const RESET_RESERVED_SHIFT: u32 =        31;         // Internal bits shift.
pub const RESET_RESERVED_MASK: u32 =         0x80000000; // Internal bits mask.

//
pub const CAPS_ALPHA_TO_COVERAGE: u64 =      0x0000000000000001; // Alpha to coverage is supported.
pub const CAPS_BLEND_INDEPENDENT: u64 =      0x0000000000000002; // Blend independent is supported.
pub const CAPS_COMPUTE: u64 =                0x0000000000000004; // Compute shaders are supported.
pub const CAPS_CONSERVATIVE_RASTER: u64 =    0x0000000000000008; // Conservative rasterization is supported.
pub const CAPS_DRAW_INDIRECT: u64 =          0x0000000000000010; // Draw indirect is supported.
pub const CAPS_FRAGMENT_DEPTH: u64 =         0x0000000000000020; // Fragment depth is accessible in fragment shader.
pub const CAPS_FRAGMENT_ORDERING: u64 =      0x0000000000000040; // Fragment ordering is available in fragment shader.
pub const CAPS_GRAPHICS_DEBUGGER: u64 =      0x0000000000000080; // Graphics debugger is present.
pub const CAPS_HIDPI: u64 =                  0x0000000000000100; // HiDPI rendering is supported.
pub const CAPS_INDEX32: u64 =                0x0000000000000400; // 32-bit indices are supported.
pub const CAPS_INSTANCING: u64 =             0x0000000000000800; // Instancing is supported.
pub const CAPS_OCCLUSION_QUERY: u64 =        0x0000000000001000; // Occlusion query is supported.
pub const CAPS_RENDERER_MULTITHREADED: u64 = 0x0000000000002000; // Renderer is on separate thread.
pub const CAPS_SWAP_CHAIN: u64 =             0x0000000000004000; // Multiple windows are supported.
pub const CAPS_TEXTURE_2D_ARRAY: u64 =       0x0000000000008000; // 2D texture array is supported.
pub const CAPS_TEXTURE_3D: u64 =             0x0000000000010000; // 3D textures are supported.
pub const CAPS_TEXTURE_BLIT: u64 =           0x0000000000020000; // Texture blit is supported.
pub const CAPS_TEXTURE_COMPARE_ALL: u64 =    0x00000000000c0000; // All texture compare modes are supported.
pub const CAPS_TEXTURE_COMPARE_LEQUAL: u64 = 0x0000000000080000; // Texture compare less equal mode is supported.
pub const CAPS_TEXTURE_CUBE_ARRAY: u64 =     0x0000000000100000; // Cubemap texture array is supported.
pub const CAPS_TEXTURE_DIRECT_ACCESS: u64 =  0x0000000000200000; // CPU direct access to GPU texture memory.
pub const CAPS_TEXTURE_READ_BACK: u64 =      0x0000000000400000; // Read-back texture is supported.
pub const CAPS_VERTEX_ATTRIB_HALF: u64 =     0x0000000000800000; // Vertex attribute half-float is supported.
pub const CAPS_VERTEX_ATTRIB_UINT10: u64 =   0x0000000000800000; // Vertex attribute 10_10_10_2 is supported.
pub const CAPS_VERTEX_ID: u64 =              0x0000000001000000; // Rendering with VertexID only is supported.

//
pub const CAPS_FORMAT_TEXTURE_NONE: u16 =             0x0000; // Texture format is not supported.
pub const CAPS_FORMAT_TEXTURE_2D: u16 =               0x0001; // Texture format is supported.
pub const CAPS_FORMAT_TEXTURE_2D_SRGB: u16 =          0x0002; // Texture as sRGB format is supported.
pub const CAPS_FORMAT_TEXTURE_2D_EMULATED: u16 =      0x0004; // Texture format is emulated.
pub const CAPS_FORMAT_TEXTURE_3D: u16 =               0x0008; // Texture format is supported.
pub const CAPS_FORMAT_TEXTURE_3D_SRGB: u16 =          0x0010; // Texture as sRGB format is supported.
pub const CAPS_FORMAT_TEXTURE_3D_EMULATED: u16 =      0x0020; // Texture format is emulated.
pub const CAPS_FORMAT_TEXTURE_CUBE: u16 =             0x0040; // Texture format is supported.
pub const CAPS_FORMAT_TEXTURE_CUBE_SRGB: u16 =        0x0080; // Texture as sRGB format is supported.
pub const CAPS_FORMAT_TEXTURE_CUBE_EMULATED: u16 =    0x0100; // Texture format is emulated.
pub const CAPS_FORMAT_TEXTURE_VERTEX: u16 =           0x0200; // Texture format can be used from vertex shader.
pub const CAPS_FORMAT_TEXTURE_IMAGE: u16 =            0x0400; // Texture format can be used as image from compute shader.
pub const CAPS_FORMAT_TEXTURE_FRAMEBUFFER: u16 =      0x0800; // Texture format can be used as frame buffer.
pub const CAPS_FORMAT_TEXTURE_FRAMEBUFFER_MSAA: u16 = 0x1000; // Texture format can be used as MSAA frame buffer.
pub const CAPS_FORMAT_TEXTURE_MSAA: u16 =             0x2000; // Texture can be sampled as MSAA.
pub const CAPS_FORMAT_TEXTURE_MIP_AUTOGEN: u16 =      0x4000; // Texture format supports auto-generated mips.

//
pub const VIEW_NONE: u8 =   0x00; //
pub const VIEW_STEREO: u8 = 0x01; // View will be rendered in stereo mode.

//
pub const SUBMIT_EYE_LEFT: u8 =       0x01; // Submit to left eye.
pub const SUBMIT_EYE_RIGHT: u8 =      0x02; // Submit to right eye.
pub const SUBMIT_EYE_MASK: u8 =       0x03; //
pub const SUBMIT_EYE_FIRST: u8 =      SUBMIT_EYE_LEFT;

pub const SUBMIT_RESERVED_SHIFT: u8 = 7;    // Internal bits shift.
pub const SUBMIT_RESERVED_MASK: u8 =  0x80; // Internal bits mask.

//
pub const PCI_ID_NONE: u16 =                0x0000; // Autoselect adapter.
pub const PCI_ID_SOFTWARE_RASTERIZER: u16 = 0x0001; // Software rasterizer.
pub const PCI_ID_AMD: u16 =                 0x1002; // AMD adapter.
pub const PCI_ID_INTEL: u16 =               0x8086; // Intel adapter.
pub const PCI_ID_NVIDIA: u16 =              0x10de; // nVidia adapter.

//
pub const CUBE_MAP_POSITIVE_X: u8 = 0x00; // Cubemap +x.
pub const CUBE_MAP_NEGATIVE_X: u8 = 0x01; // Cubemap -x.
pub const CUBE_MAP_POSITIVE_Y: u8 = 0x02; // Cubemap +y.
pub const CUBE_MAP_NEGATIVE_Y: u8 = 0x03; // Cubemap -y.
pub const CUBE_MAP_POSITIVE_Z: u8 = 0x04; // Cubemap +z.
pub const CUBE_MAP_NEGATIVE_Z: u8 = 0x05; // Cubemap -z.

