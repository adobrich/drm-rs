/* automatically generated by rust-bindgen */

pub const DRM_NAME: &'static [u8; 4usize] = b"drm\0";
pub const DRM_MIN_ORDER: u32 = 5;
pub const DRM_MAX_ORDER: u32 = 22;
pub const DRM_RAM_PERCENT: u32 = 10;
pub const DRM_CAP_DUMB_BUFFER: u32 = 1;
pub const DRM_CAP_VBLANK_HIGH_CRTC: u32 = 2;
pub const DRM_CAP_DUMB_PREFERRED_DEPTH: u32 = 3;
pub const DRM_CAP_DUMB_PREFER_SHADOW: u32 = 4;
pub const DRM_CAP_PRIME: u32 = 5;
pub const DRM_PRIME_CAP_IMPORT: u32 = 1;
pub const DRM_PRIME_CAP_EXPORT: u32 = 2;
pub const DRM_CAP_TIMESTAMP_MONOTONIC: u32 = 6;
pub const DRM_CAP_ASYNC_PAGE_FLIP: u32 = 7;
pub const DRM_CAP_CURSOR_WIDTH: u32 = 8;
pub const DRM_CAP_CURSOR_HEIGHT: u32 = 9;
pub const DRM_CAP_ADDFB2_MODIFIERS: u32 = 16;
pub const DRM_CAP_PAGE_FLIP_TARGET: u32 = 17;
pub const DRM_CAP_CRTC_IN_VBLANK_EVENT: u32 = 18;
pub const DRM_CAP_SYNCOBJ: u32 = 19;
pub const DRM_CLIENT_CAP_STEREO_3D: u32 = 1;
pub const DRM_CLIENT_CAP_UNIVERSAL_PLANES: u32 = 2;
pub const DRM_CLIENT_CAP_ATOMIC: u32 = 3;
pub const DRM_SYNCOBJ_CREATE_SIGNALED: u32 = 1;
pub const DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_IMPORT_SYNC_FILE: u32 = 1;
pub const DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE: u32 = 1;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL: u32 = 1;
pub const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT: u32 = 2;
pub const DRM_CRTC_SEQUENCE_RELATIVE: u32 = 1;
pub const DRM_CRTC_SEQUENCE_NEXT_ON_MISS: u32 = 2;
pub const DRM_DISPLAY_INFO_LEN: u32 = 32;
pub const DRM_CONNECTOR_NAME_LEN: u32 = 32;
pub const DRM_DISPLAY_MODE_LEN: u32 = 32;
pub const DRM_PROP_NAME_LEN: u32 = 32;
pub const DRM_MODE_TYPE_BUILTIN: u32 = 1;
pub const DRM_MODE_TYPE_CLOCK_C: u32 = 3;
pub const DRM_MODE_TYPE_CRTC_C: u32 = 5;
pub const DRM_MODE_TYPE_PREFERRED: u32 = 8;
pub const DRM_MODE_TYPE_DEFAULT: u32 = 16;
pub const DRM_MODE_TYPE_USERDEF: u32 = 32;
pub const DRM_MODE_TYPE_DRIVER: u32 = 64;
pub const DRM_MODE_FLAG_PHSYNC: u32 = 1;
pub const DRM_MODE_FLAG_NHSYNC: u32 = 2;
pub const DRM_MODE_FLAG_PVSYNC: u32 = 4;
pub const DRM_MODE_FLAG_NVSYNC: u32 = 8;
pub const DRM_MODE_FLAG_INTERLACE: u32 = 16;
pub const DRM_MODE_FLAG_DBLSCAN: u32 = 32;
pub const DRM_MODE_FLAG_CSYNC: u32 = 64;
pub const DRM_MODE_FLAG_PCSYNC: u32 = 128;
pub const DRM_MODE_FLAG_NCSYNC: u32 = 256;
pub const DRM_MODE_FLAG_HSKEW: u32 = 512;
pub const DRM_MODE_FLAG_BCAST: u32 = 1024;
pub const DRM_MODE_FLAG_PIXMUX: u32 = 2048;
pub const DRM_MODE_FLAG_DBLCLK: u32 = 4096;
pub const DRM_MODE_FLAG_CLKDIV2: u32 = 8192;
pub const DRM_MODE_FLAG_3D_MASK: u32 = 507904;
pub const DRM_MODE_FLAG_3D_NONE: u32 = 0;
pub const DRM_MODE_FLAG_3D_FRAME_PACKING: u32 = 16384;
pub const DRM_MODE_FLAG_3D_FIELD_ALTERNATIVE: u32 = 32768;
pub const DRM_MODE_FLAG_3D_LINE_ALTERNATIVE: u32 = 49152;
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_FULL: u32 = 65536;
pub const DRM_MODE_FLAG_3D_L_DEPTH: u32 = 81920;
pub const DRM_MODE_FLAG_3D_L_DEPTH_GFX_GFX_DEPTH: u32 = 98304;
pub const DRM_MODE_FLAG_3D_TOP_AND_BOTTOM: u32 = 114688;
pub const DRM_MODE_FLAG_3D_SIDE_BY_SIDE_HALF: u32 = 131072;
pub const DRM_MODE_PICTURE_ASPECT_NONE: u32 = 0;
pub const DRM_MODE_PICTURE_ASPECT_4_3: u32 = 1;
pub const DRM_MODE_PICTURE_ASPECT_16_9: u32 = 2;
pub const DRM_MODE_FLAG_PIC_AR_MASK: u32 = 7864320;
pub const DRM_MODE_FLAG_PIC_AR_NONE: u32 = 0;
pub const DRM_MODE_FLAG_PIC_AR_4_3: u32 = 524288;
pub const DRM_MODE_FLAG_PIC_AR_16_9: u32 = 1048576;
pub const DRM_MODE_DPMS_ON: u32 = 0;
pub const DRM_MODE_DPMS_STANDBY: u32 = 1;
pub const DRM_MODE_DPMS_SUSPEND: u32 = 2;
pub const DRM_MODE_DPMS_OFF: u32 = 3;
pub const DRM_MODE_SCALE_NONE: u32 = 0;
pub const DRM_MODE_SCALE_FULLSCREEN: u32 = 1;
pub const DRM_MODE_SCALE_CENTER: u32 = 2;
pub const DRM_MODE_SCALE_ASPECT: u32 = 3;
pub const DRM_MODE_DITHERING_OFF: u32 = 0;
pub const DRM_MODE_DITHERING_ON: u32 = 1;
pub const DRM_MODE_DITHERING_AUTO: u32 = 2;
pub const DRM_MODE_DIRTY_OFF: u32 = 0;
pub const DRM_MODE_DIRTY_ON: u32 = 1;
pub const DRM_MODE_DIRTY_ANNOTATE: u32 = 2;
pub const DRM_MODE_LINK_STATUS_GOOD: u32 = 0;
pub const DRM_MODE_LINK_STATUS_BAD: u32 = 1;
pub const DRM_MODE_ROTATE_0: u32 = 1;
pub const DRM_MODE_ROTATE_90: u32 = 2;
pub const DRM_MODE_ROTATE_180: u32 = 4;
pub const DRM_MODE_ROTATE_270: u32 = 8;
pub const DRM_MODE_ROTATE_MASK: u32 = 15;
pub const DRM_MODE_REFLECT_X: u32 = 16;
pub const DRM_MODE_REFLECT_Y: u32 = 32;
pub const DRM_MODE_REFLECT_MASK: u32 = 48;
pub const DRM_MODE_PRESENT_TOP_FIELD: u32 = 1;
pub const DRM_MODE_PRESENT_BOTTOM_FIELD: u32 = 2;
pub const DRM_MODE_ENCODER_NONE: u32 = 0;
pub const DRM_MODE_ENCODER_DAC: u32 = 1;
pub const DRM_MODE_ENCODER_TMDS: u32 = 2;
pub const DRM_MODE_ENCODER_LVDS: u32 = 3;
pub const DRM_MODE_ENCODER_TVDAC: u32 = 4;
pub const DRM_MODE_ENCODER_VIRTUAL: u32 = 5;
pub const DRM_MODE_ENCODER_DSI: u32 = 6;
pub const DRM_MODE_ENCODER_DPMST: u32 = 7;
pub const DRM_MODE_ENCODER_DPI: u32 = 8;
pub const DRM_MODE_CONNECTOR_Unknown: u32 = 0;
pub const DRM_MODE_CONNECTOR_VGA: u32 = 1;
pub const DRM_MODE_CONNECTOR_DVII: u32 = 2;
pub const DRM_MODE_CONNECTOR_DVID: u32 = 3;
pub const DRM_MODE_CONNECTOR_DVIA: u32 = 4;
pub const DRM_MODE_CONNECTOR_Composite: u32 = 5;
pub const DRM_MODE_CONNECTOR_SVIDEO: u32 = 6;
pub const DRM_MODE_CONNECTOR_LVDS: u32 = 7;
pub const DRM_MODE_CONNECTOR_Component: u32 = 8;
pub const DRM_MODE_CONNECTOR_9PinDIN: u32 = 9;
pub const DRM_MODE_CONNECTOR_DisplayPort: u32 = 10;
pub const DRM_MODE_CONNECTOR_HDMIA: u32 = 11;
pub const DRM_MODE_CONNECTOR_HDMIB: u32 = 12;
pub const DRM_MODE_CONNECTOR_TV: u32 = 13;
pub const DRM_MODE_CONNECTOR_eDP: u32 = 14;
pub const DRM_MODE_CONNECTOR_VIRTUAL: u32 = 15;
pub const DRM_MODE_CONNECTOR_DSI: u32 = 16;
pub const DRM_MODE_CONNECTOR_DPI: u32 = 17;
pub const DRM_MODE_PROP_PENDING: u32 = 1;
pub const DRM_MODE_PROP_RANGE: u32 = 2;
pub const DRM_MODE_PROP_IMMUTABLE: u32 = 4;
pub const DRM_MODE_PROP_ENUM: u32 = 8;
pub const DRM_MODE_PROP_BLOB: u32 = 16;
pub const DRM_MODE_PROP_BITMASK: u32 = 32;
pub const DRM_MODE_PROP_LEGACY_TYPE: u32 = 58;
pub const DRM_MODE_PROP_EXTENDED_TYPE: u32 = 65472;
pub const DRM_MODE_PROP_ATOMIC: u32 = 2147483648;
pub const DRM_MODE_OBJECT_CRTC: u32 = 3435973836;
pub const DRM_MODE_OBJECT_CONNECTOR: u32 = 3233857728;
pub const DRM_MODE_OBJECT_ENCODER: u32 = 3772834016;
pub const DRM_MODE_OBJECT_MODE: u32 = 3739147998;
pub const DRM_MODE_OBJECT_PROPERTY: u32 = 2964369584;
pub const DRM_MODE_OBJECT_FB: u32 = 4227595259;
pub const DRM_MODE_OBJECT_BLOB: u32 = 3149642683;
pub const DRM_MODE_OBJECT_PLANE: u32 = 4008636142;
pub const DRM_MODE_OBJECT_ANY: u32 = 0;
pub const DRM_MODE_FB_INTERLACED: u32 = 1;
pub const DRM_MODE_FB_MODIFIERS: u32 = 2;
pub const DRM_MODE_FB_DIRTY_ANNOTATE_COPY: u32 = 1;
pub const DRM_MODE_FB_DIRTY_ANNOTATE_FILL: u32 = 2;
pub const DRM_MODE_FB_DIRTY_FLAGS: u32 = 3;
pub const DRM_MODE_FB_DIRTY_MAX_CLIPS: u32 = 256;
pub const DRM_MODE_CURSOR_BO: u32 = 1;
pub const DRM_MODE_CURSOR_MOVE: u32 = 2;
pub const DRM_MODE_CURSOR_FLAGS: u32 = 3;
pub const DRM_MODE_PAGE_FLIP_EVENT: u32 = 1;
pub const DRM_MODE_PAGE_FLIP_ASYNC: u32 = 2;
pub const DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE: u32 = 4;
pub const DRM_MODE_PAGE_FLIP_TARGET_RELATIVE: u32 = 8;
pub const DRM_MODE_PAGE_FLIP_TARGET: u32 = 12;
pub const DRM_MODE_PAGE_FLIP_FLAGS: u32 = 15;
pub const DRM_MODE_ATOMIC_TEST_ONLY: u32 = 256;
pub const DRM_MODE_ATOMIC_NONBLOCK: u32 = 512;
pub const DRM_MODE_ATOMIC_ALLOW_MODESET: u32 = 1024;
pub const DRM_MODE_ATOMIC_FLAGS: u32 = 1795;
pub const DRM_IOCTL_BASE: u8 = 100u8;
pub const DRM_COMMAND_BASE: u32 = 64;
pub const DRM_COMMAND_END: u32 = 160;
pub const DRM_EVENT_VBLANK: u32 = 1;
pub const DRM_EVENT_FLIP_COMPLETE: u32 = 2;
pub const DRM_EVENT_CRTC_SEQUENCE: u32 = 3;
pub type __u16 = libc::c_ushort;
pub type __s32 = libc::c_int;
pub type __u32 = libc::c_uint;
pub type __s64 = libc::c_longlong;
pub type __u64 = libc::c_ulonglong;
pub type __kernel_ulong_t = libc::c_ulong;
pub type __kernel_size_t = __kernel_ulong_t;
pub type drm_handle_t = libc::c_uint;
pub type drm_context_t = libc::c_uint;
pub type drm_drawable_t = libc::c_uint;
pub type drm_magic_t = libc::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_clip_rect {
    pub x1: libc::c_ushort,
    pub y1: libc::c_ushort,
    pub x2: libc::c_ushort,
    pub y2: libc::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_drawable_info {
    pub num_rects: libc::c_uint,
    pub rects: *mut drm_clip_rect,
}
impl Default for drm_drawable_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_tex_region {
    pub next: libc::c_uchar,
    pub prev: libc::c_uchar,
    pub in_use: libc::c_uchar,
    pub padding: libc::c_uchar,
    pub age: libc::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_hw_lock {
    pub lock: libc::c_uint,
    pub padding: [libc::c_char; 60usize],
}
impl Default for drm_hw_lock {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_version {
    pub version_major: libc::c_int,
    pub version_minor: libc::c_int,
    pub version_patchlevel: libc::c_int,
    pub name_len: __kernel_size_t,
    pub name: *mut libc::c_char,
    pub date_len: __kernel_size_t,
    pub date: *mut libc::c_char,
    pub desc_len: __kernel_size_t,
    pub desc: *mut libc::c_char,
}
impl Default for drm_version {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_unique {
    pub unique_len: __kernel_size_t,
    pub unique: *mut libc::c_char,
}
impl Default for drm_unique {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_list {
    pub count: libc::c_int,
    pub version: *mut drm_version,
}
impl Default for drm_list {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_block {
    pub unused: libc::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_control {
    pub func: drm_control__bindgen_ty_1::Type,
    pub irq: libc::c_int,
}
pub mod drm_control__bindgen_ty_1 {
    pub type Type = u32;
    pub const DRM_ADD_COMMAND: Type = 0;
    pub const DRM_RM_COMMAND: Type = 1;
    pub const DRM_INST_HANDLER: Type = 2;
    pub const DRM_UNINST_HANDLER: Type = 3;
}
impl Default for drm_control {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub mod drm_map_type {
    pub type Type = u32;
    pub const _DRM_FRAME_BUFFER: Type = 0;
    pub const _DRM_REGISTERS: Type = 1;
    pub const _DRM_SHM: Type = 2;
    pub const _DRM_AGP: Type = 3;
    pub const _DRM_SCATTER_GATHER: Type = 4;
    pub const _DRM_CONSISTENT: Type = 5;
}
pub mod drm_map_flags {
    pub type Type = u32;
    pub const _DRM_RESTRICTED: Type = 1;
    pub const _DRM_READ_ONLY: Type = 2;
    pub const _DRM_LOCKED: Type = 4;
    pub const _DRM_KERNEL: Type = 8;
    pub const _DRM_WRITE_COMBINING: Type = 16;
    pub const _DRM_CONTAINS_LOCK: Type = 32;
    pub const _DRM_REMOVABLE: Type = 64;
    pub const _DRM_DRIVER: Type = 128;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_ctx_priv_map {
    pub ctx_id: libc::c_uint,
    pub handle: *mut libc::c_void,
}
impl Default for drm_ctx_priv_map {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_map {
    pub offset: libc::c_ulong,
    pub size: libc::c_ulong,
    pub type_: drm_map_type::Type,
    pub flags: drm_map_flags::Type,
    pub handle: *mut libc::c_void,
    pub mtrr: libc::c_int,
}
impl Default for drm_map {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_client {
    pub idx: libc::c_int,
    pub auth: libc::c_int,
    pub pid: libc::c_ulong,
    pub uid: libc::c_ulong,
    pub magic: libc::c_ulong,
    pub iocs: libc::c_ulong,
}
pub mod drm_stat_type {
    pub type Type = u32;
    pub const _DRM_STAT_LOCK: Type = 0;
    pub const _DRM_STAT_OPENS: Type = 1;
    pub const _DRM_STAT_CLOSES: Type = 2;
    pub const _DRM_STAT_IOCTLS: Type = 3;
    pub const _DRM_STAT_LOCKS: Type = 4;
    pub const _DRM_STAT_UNLOCKS: Type = 5;
    pub const _DRM_STAT_VALUE: Type = 6;
    pub const _DRM_STAT_BYTE: Type = 7;
    pub const _DRM_STAT_COUNT: Type = 8;
    pub const _DRM_STAT_IRQ: Type = 9;
    pub const _DRM_STAT_PRIMARY: Type = 10;
    pub const _DRM_STAT_SECONDARY: Type = 11;
    pub const _DRM_STAT_DMA: Type = 12;
    pub const _DRM_STAT_SPECIAL: Type = 13;
    pub const _DRM_STAT_MISSED: Type = 14;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_stats {
    pub count: libc::c_ulong,
    pub data: [drm_stats__bindgen_ty_1; 15usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_stats__bindgen_ty_1 {
    pub value: libc::c_ulong,
    pub type_: drm_stat_type::Type,
}
impl Default for drm_stats__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for drm_stats {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub mod drm_lock_flags {
    pub type Type = u32;
    pub const _DRM_LOCK_READY: Type = 1;
    pub const _DRM_LOCK_QUIESCENT: Type = 2;
    pub const _DRM_LOCK_FLUSH: Type = 4;
    pub const _DRM_LOCK_FLUSH_ALL: Type = 8;
    pub const _DRM_HALT_ALL_QUEUES: Type = 16;
    pub const _DRM_HALT_CUR_QUEUES: Type = 32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_lock {
    pub context: libc::c_int,
    pub flags: drm_lock_flags::Type,
}
impl Default for drm_lock {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub mod drm_dma_flags {
    pub type Type = u32;
    pub const _DRM_DMA_BLOCK: Type = 1;
    pub const _DRM_DMA_WHILE_LOCKED: Type = 2;
    pub const _DRM_DMA_PRIORITY: Type = 4;
    pub const _DRM_DMA_WAIT: Type = 16;
    pub const _DRM_DMA_SMALLER_OK: Type = 32;
    pub const _DRM_DMA_LARGER_OK: Type = 64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_buf_desc {
    pub count: libc::c_int,
    pub size: libc::c_int,
    pub low_mark: libc::c_int,
    pub high_mark: libc::c_int,
    pub flags: drm_buf_desc__bindgen_ty_1::Type,
    pub agp_start: libc::c_ulong,
}
pub mod drm_buf_desc__bindgen_ty_1 {
    pub type Type = u32;
    pub const _DRM_PAGE_ALIGN: Type = 1;
    pub const _DRM_AGP_BUFFER: Type = 2;
    pub const _DRM_SG_BUFFER: Type = 4;
    pub const _DRM_FB_BUFFER: Type = 8;
    pub const _DRM_PCI_BUFFER_RO: Type = 16;
}
impl Default for drm_buf_desc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_buf_info {
    pub count: libc::c_int,
    pub list: *mut drm_buf_desc,
}
impl Default for drm_buf_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_buf_free {
    pub count: libc::c_int,
    pub list: *mut libc::c_int,
}
impl Default for drm_buf_free {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_buf_pub {
    pub idx: libc::c_int,
    pub total: libc::c_int,
    pub used: libc::c_int,
    pub address: *mut libc::c_void,
}
impl Default for drm_buf_pub {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_buf_map {
    pub count: libc::c_int,
    pub virtual_: *mut libc::c_void,
    pub list: *mut drm_buf_pub,
}
impl Default for drm_buf_map {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_dma {
    pub context: libc::c_int,
    pub send_count: libc::c_int,
    pub send_indices: *mut libc::c_int,
    pub send_sizes: *mut libc::c_int,
    pub flags: drm_dma_flags::Type,
    pub request_count: libc::c_int,
    pub request_size: libc::c_int,
    pub request_indices: *mut libc::c_int,
    pub request_sizes: *mut libc::c_int,
    pub granted_count: libc::c_int,
}
impl Default for drm_dma {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub mod drm_ctx_flags {
    pub type Type = u32;
    pub const _DRM_CONTEXT_PRESERVED: Type = 1;
    pub const _DRM_CONTEXT_2DONLY: Type = 2;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_ctx {
    pub handle: drm_context_t,
    pub flags: drm_ctx_flags::Type,
}
impl Default for drm_ctx {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_ctx_res {
    pub count: libc::c_int,
    pub contexts: *mut drm_ctx,
}
impl Default for drm_ctx_res {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_draw {
    pub handle: drm_drawable_t,
}
pub mod drm_drawable_info_type_t {
    pub type Type = u32;
    pub const DRM_DRAWABLE_CLIPRECTS: Type = 0;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_update_draw {
    pub handle: drm_drawable_t,
    pub type_: libc::c_uint,
    pub num: libc::c_uint,
    pub data: libc::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_auth {
    pub magic: drm_magic_t,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_irq_busid {
    pub irq: libc::c_int,
    pub busnum: libc::c_int,
    pub devnum: libc::c_int,
    pub funcnum: libc::c_int,
}
pub mod drm_vblank_seq_type {
    pub type Type = u32;
    pub const _DRM_VBLANK_ABSOLUTE: Type = 0;
    pub const _DRM_VBLANK_RELATIVE: Type = 1;
    pub const _DRM_VBLANK_HIGH_CRTC_MASK: Type = 62;
    pub const _DRM_VBLANK_EVENT: Type = 67108864;
    pub const _DRM_VBLANK_FLIP: Type = 134217728;
    pub const _DRM_VBLANK_NEXTONMISS: Type = 268435456;
    pub const _DRM_VBLANK_SECONDARY: Type = 536870912;
    pub const _DRM_VBLANK_SIGNAL: Type = 1073741824;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_wait_vblank_request {
    pub type_: drm_vblank_seq_type::Type,
    pub sequence: libc::c_uint,
    pub signal: libc::c_ulong,
}
impl Default for drm_wait_vblank_request {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_wait_vblank_reply {
    pub type_: drm_vblank_seq_type::Type,
    pub sequence: libc::c_uint,
    pub tval_sec: libc::c_long,
    pub tval_usec: libc::c_long,
}
impl Default for drm_wait_vblank_reply {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_wait_vblank {
    pub request: drm_wait_vblank_request,
    pub reply: drm_wait_vblank_reply,
    _bindgen_union_align: [u64; 3usize],
}
impl Default for drm_wait_vblank {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_modeset_ctl {
    pub crtc: __u32,
    pub cmd: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_agp_mode {
    pub mode: libc::c_ulong,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_agp_buffer {
    pub size: libc::c_ulong,
    pub handle: libc::c_ulong,
    pub type_: libc::c_ulong,
    pub physical: libc::c_ulong,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_agp_binding {
    pub handle: libc::c_ulong,
    pub offset: libc::c_ulong,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_agp_info {
    pub agp_version_major: libc::c_int,
    pub agp_version_minor: libc::c_int,
    pub mode: libc::c_ulong,
    pub aperture_base: libc::c_ulong,
    pub aperture_size: libc::c_ulong,
    pub memory_allowed: libc::c_ulong,
    pub memory_used: libc::c_ulong,
    pub id_vendor: libc::c_ushort,
    pub id_device: libc::c_ushort,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_scatter_gather {
    pub size: libc::c_ulong,
    pub handle: libc::c_ulong,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_set_version {
    pub drm_di_major: libc::c_int,
    pub drm_di_minor: libc::c_int,
    pub drm_dd_major: libc::c_int,
    pub drm_dd_minor: libc::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_gem_close {
    pub handle: __u32,
    pub pad: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_gem_flink {
    pub handle: __u32,
    pub name: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_gem_open {
    pub name: __u32,
    pub handle: __u32,
    pub size: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_get_cap {
    pub capability: __u64,
    pub value: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_set_client_cap {
    pub capability: __u64,
    pub value: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_prime_handle {
    pub handle: __u32,
    pub flags: __u32,
    pub fd: __s32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_syncobj_create {
    pub handle: __u32,
    pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_syncobj_destroy {
    pub handle: __u32,
    pub pad: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_syncobj_handle {
    pub handle: __u32,
    pub flags: __u32,
    pub fd: __s32,
    pub pad: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_syncobj_wait {
    pub handles: __u64,
    pub timeout_nsec: __s64,
    pub count_handles: __u32,
    pub flags: __u32,
    pub first_signaled: __u32,
    pub pad: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_syncobj_array {
    pub handles: __u64,
    pub count_handles: __u32,
    pub pad: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_crtc_get_sequence {
    pub crtc_id: __u32,
    pub active: __u32,
    pub sequence: __u64,
    pub sequence_ns: __s64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_crtc_queue_sequence {
    pub crtc_id: __u32,
    pub flags: __u32,
    pub sequence: __u64,
    pub user_data: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_modeinfo {
    pub clock: __u32,
    pub hdisplay: __u16,
    pub hsync_start: __u16,
    pub hsync_end: __u16,
    pub htotal: __u16,
    pub hskew: __u16,
    pub vdisplay: __u16,
    pub vsync_start: __u16,
    pub vsync_end: __u16,
    pub vtotal: __u16,
    pub vscan: __u16,
    pub vrefresh: __u32,
    pub flags: __u32,
    pub type_: __u32,
    pub name: [libc::c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_card_res {
    pub fb_id_ptr: __u64,
    pub crtc_id_ptr: __u64,
    pub connector_id_ptr: __u64,
    pub encoder_id_ptr: __u64,
    pub count_fbs: __u32,
    pub count_crtcs: __u32,
    pub count_connectors: __u32,
    pub count_encoders: __u32,
    pub min_width: __u32,
    pub max_width: __u32,
    pub min_height: __u32,
    pub max_height: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_crtc {
    pub set_connectors_ptr: __u64,
    pub count_connectors: __u32,
    pub crtc_id: __u32,
    pub fb_id: __u32,
    pub x: __u32,
    pub y: __u32,
    pub gamma_size: __u32,
    pub mode_valid: __u32,
    pub mode: drm_mode_modeinfo,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_set_plane {
    pub plane_id: __u32,
    pub crtc_id: __u32,
    pub fb_id: __u32,
    pub flags: __u32,
    pub crtc_x: __s32,
    pub crtc_y: __s32,
    pub crtc_w: __u32,
    pub crtc_h: __u32,
    pub src_x: __u32,
    pub src_y: __u32,
    pub src_h: __u32,
    pub src_w: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_get_plane {
    pub plane_id: __u32,
    pub crtc_id: __u32,
    pub fb_id: __u32,
    pub possible_crtcs: __u32,
    pub gamma_size: __u32,
    pub count_format_types: __u32,
    pub format_type_ptr: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_get_plane_res {
    pub plane_id_ptr: __u64,
    pub count_planes: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_get_encoder {
    pub encoder_id: __u32,
    pub encoder_type: __u32,
    pub crtc_id: __u32,
    pub possible_crtcs: __u32,
    pub possible_clones: __u32,
}
pub mod drm_mode_subconnector {
    pub type Type = u32;
    pub const DRM_MODE_SUBCONNECTOR_Automatic: Type = 0;
    pub const DRM_MODE_SUBCONNECTOR_Unknown: Type = 0;
    pub const DRM_MODE_SUBCONNECTOR_DVID: Type = 3;
    pub const DRM_MODE_SUBCONNECTOR_DVIA: Type = 4;
    pub const DRM_MODE_SUBCONNECTOR_Composite: Type = 5;
    pub const DRM_MODE_SUBCONNECTOR_SVIDEO: Type = 6;
    pub const DRM_MODE_SUBCONNECTOR_Component: Type = 8;
    pub const DRM_MODE_SUBCONNECTOR_SCART: Type = 9;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_get_connector {
    pub encoders_ptr: __u64,
    pub modes_ptr: __u64,
    pub props_ptr: __u64,
    pub prop_values_ptr: __u64,
    pub count_modes: __u32,
    pub count_props: __u32,
    pub count_encoders: __u32,
    pub encoder_id: __u32,
    pub connector_id: __u32,
    pub connector_type: __u32,
    pub connector_type_id: __u32,
    pub connection: __u32,
    pub mm_width: __u32,
    pub mm_height: __u32,
    pub subpixel: __u32,
    pub pad: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_property_enum {
    pub value: __u64,
    pub name: [libc::c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_get_property {
    pub values_ptr: __u64,
    pub enum_blob_ptr: __u64,
    pub prop_id: __u32,
    pub flags: __u32,
    pub name: [libc::c_char; 32usize],
    pub count_values: __u32,
    pub count_enum_blobs: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_connector_set_property {
    pub value: __u64,
    pub prop_id: __u32,
    pub connector_id: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_obj_get_properties {
    pub props_ptr: __u64,
    pub prop_values_ptr: __u64,
    pub count_props: __u32,
    pub obj_id: __u32,
    pub obj_type: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_obj_set_property {
    pub value: __u64,
    pub prop_id: __u32,
    pub obj_id: __u32,
    pub obj_type: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_get_blob {
    pub blob_id: __u32,
    pub length: __u32,
    pub data: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_fb_cmd {
    pub fb_id: __u32,
    pub width: __u32,
    pub height: __u32,
    pub pitch: __u32,
    pub bpp: __u32,
    pub depth: __u32,
    pub handle: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_fb_cmd2 {
    pub fb_id: __u32,
    pub width: __u32,
    pub height: __u32,
    pub pixel_format: __u32,
    pub flags: __u32,
    pub handles: [__u32; 4usize],
    pub pitches: [__u32; 4usize],
    pub offsets: [__u32; 4usize],
    pub modifier: [__u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_fb_dirty_cmd {
    pub fb_id: __u32,
    pub flags: __u32,
    pub color: __u32,
    pub num_clips: __u32,
    pub clips_ptr: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_mode_cmd {
    pub connector_id: __u32,
    pub mode: drm_mode_modeinfo,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_cursor {
    pub flags: __u32,
    pub crtc_id: __u32,
    pub x: __s32,
    pub y: __s32,
    pub width: __u32,
    pub height: __u32,
    pub handle: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_cursor2 {
    pub flags: __u32,
    pub crtc_id: __u32,
    pub x: __s32,
    pub y: __s32,
    pub width: __u32,
    pub height: __u32,
    pub handle: __u32,
    pub hot_x: __s32,
    pub hot_y: __s32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_crtc_lut {
    pub crtc_id: __u32,
    pub gamma_size: __u32,
    pub red: __u64,
    pub green: __u64,
    pub blue: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_color_ctm {
    pub matrix: [__s64; 9usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_color_lut {
    pub red: __u16,
    pub green: __u16,
    pub blue: __u16,
    pub reserved: __u16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_crtc_page_flip {
    pub crtc_id: __u32,
    pub fb_id: __u32,
    pub flags: __u32,
    pub reserved: __u32,
    pub user_data: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_crtc_page_flip_target {
    pub crtc_id: __u32,
    pub fb_id: __u32,
    pub flags: __u32,
    pub sequence: __u32,
    pub user_data: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_create_dumb {
    pub height: __u32,
    pub width: __u32,
    pub bpp: __u32,
    pub flags: __u32,
    pub handle: __u32,
    pub pitch: __u32,
    pub size: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_map_dumb {
    pub handle: __u32,
    pub pad: __u32,
    pub offset: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_destroy_dumb {
    pub handle: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_atomic {
    pub flags: __u32,
    pub count_objs: __u32,
    pub objs_ptr: __u64,
    pub count_props_ptr: __u64,
    pub props_ptr: __u64,
    pub prop_values_ptr: __u64,
    pub reserved: __u64,
    pub user_data: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_format_modifier_blob {
    pub version: __u32,
    pub flags: __u32,
    pub count_formats: __u32,
    pub formats_offset: __u32,
    pub count_modifiers: __u32,
    pub modifiers_offset: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_format_modifier {
    pub formats: __u64,
    pub offset: __u32,
    pub pad: __u32,
    pub modifier: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_create_blob {
    pub data: __u64,
    pub length: __u32,
    pub blob_id: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_destroy_blob {
    pub blob_id: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_create_lease {
    pub object_ids: __u64,
    pub object_count: __u32,
    pub flags: __u32,
    pub lessee_id: __u32,
    pub fd: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_list_lessees {
    pub count_lessees: __u32,
    pub pad: __u32,
    pub lessees_ptr: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_get_lease {
    pub count_objects: __u32,
    pub pad: __u32,
    pub objects_ptr: __u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_mode_revoke_lease {
    pub lessee_id: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_event {
    pub type_: __u32,
    pub length: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_event_vblank {
    pub base: drm_event,
    pub user_data: __u64,
    pub tv_sec: __u32,
    pub tv_usec: __u32,
    pub sequence: __u32,
    pub crtc_id: __u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_event_crtc_sequence {
    pub base: drm_event,
    pub user_data: __u64,
    pub time_ns: __s64,
    pub sequence: __u64,
}
pub type drm_clip_rect_t = drm_clip_rect;
pub type drm_drawable_info_t = drm_drawable_info;
pub type drm_tex_region_t = drm_tex_region;
pub type drm_hw_lock_t = drm_hw_lock;
pub type drm_version_t = drm_version;
pub type drm_unique_t = drm_unique;
pub type drm_list_t = drm_list;
pub type drm_block_t = drm_block;
pub type drm_control_t = drm_control;
pub use self::drm_map_type::Type as drm_map_type_t;
pub use self::drm_map_flags::Type as drm_map_flags_t;
pub type drm_ctx_priv_map_t = drm_ctx_priv_map;
pub type drm_map_t = drm_map;
pub type drm_client_t = drm_client;
pub use self::drm_stat_type::Type as drm_stat_type_t;
pub type drm_stats_t = drm_stats;
pub use self::drm_lock_flags::Type as drm_lock_flags_t;
pub type drm_lock_t = drm_lock;
pub use self::drm_dma_flags::Type as drm_dma_flags_t;
pub type drm_buf_desc_t = drm_buf_desc;
pub type drm_buf_info_t = drm_buf_info;
pub type drm_buf_free_t = drm_buf_free;
pub type drm_buf_pub_t = drm_buf_pub;
pub type drm_buf_map_t = drm_buf_map;
pub type drm_dma_t = drm_dma;
pub type drm_wait_vblank_t = drm_wait_vblank;
pub type drm_agp_mode_t = drm_agp_mode;
pub use self::drm_ctx_flags::Type as drm_ctx_flags_t;
pub type drm_ctx_t = drm_ctx;
pub type drm_ctx_res_t = drm_ctx_res;
pub type drm_draw_t = drm_draw;
pub type drm_update_draw_t = drm_update_draw;
pub type drm_auth_t = drm_auth;
pub type drm_irq_busid_t = drm_irq_busid;
pub use self::drm_vblank_seq_type::Type as drm_vblank_seq_type_t;
pub type drm_agp_buffer_t = drm_agp_buffer;
pub type drm_agp_binding_t = drm_agp_binding;
pub type drm_agp_info_t = drm_agp_info;
pub type drm_scatter_gather_t = drm_scatter_gather;
pub type drm_set_version_t = drm_set_version;
pub const DRM_MODE_PROP_SIGNED_RANGE: libc::c_uint = 128;
pub const DRM_MODE_PROP_OBJECT: libc::c_uint = 64;