#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern "C" {
    pub fn bgzf_tell_func(fp: *mut BGZF) -> int64_t;
}

/* manually added */
// bgzf.h
pub enum Struct_BGZF { }
// the true struct cannot safely be reimplemented here, because we cannot foresee what the
// compiler does to the bitfields at the beginning.
/*
    unsigned errcode:16, is_write:2, is_be:2;
    signed compress_level:9;
    unsigned is_compressed:2, is_gzip:1;
    int cache_size;
    int block_length, block_offset;
    int64_t block_address, uncompressed_address;
    void *uncompressed_block, *compressed_block;
    void *cache; // a pointer to a hash table
    struct hFILE *fp; // actual file handle
    struct bgzf_mtaux_t *mt; // only used for multi-threading
    bgzidx_t *idx;      // BGZF index
    int idx_build_otf;  // build index on the fly, set by bgzf_index_build_init()
    z_stream *gz_stream;// for gzip-compressed files
*/

pub type BGZF = Struct_BGZF;

extern "C" {
    pub fn bgzf_close(fp: *mut BGZF) -> ::libc::c_int;
    pub fn bgzf_mt(fp: *mut BGZF, n_threads: ::libc::c_int, n_sub_blks: ::libc::c_int) -> ::libc::c_int;
}

// hts.h
pub const HTS_FMT_BAI: ::libc::c_int = 1;


/* automatically generated by rust-bindgen */

pub type int8_t = ::libc::c_char;
pub type int16_t = ::libc::c_short;
pub type int32_t = ::libc::c_int;
pub type int64_t = ::libc::c_long;
pub type uint8_t = ::libc::c_uchar;
pub type uint16_t = ::libc::c_ushort;
pub type uint32_t = ::libc::c_uint;
pub type uint64_t = ::libc::c_ulong;
pub type int_least8_t = ::libc::c_char;
pub type int_least16_t = ::libc::c_short;
pub type int_least32_t = ::libc::c_int;
pub type int_least64_t = ::libc::c_long;
pub type uint_least8_t = ::libc::c_uchar;
pub type uint_least16_t = ::libc::c_ushort;
pub type uint_least32_t = ::libc::c_uint;
pub type uint_least64_t = ::libc::c_ulong;
pub type int_fast8_t = ::libc::c_char;
pub type int_fast16_t = ::libc::c_long;
pub type int_fast32_t = ::libc::c_long;
pub type int_fast64_t = ::libc::c_long;
pub type uint_fast8_t = ::libc::c_uchar;
pub type uint_fast16_t = ::libc::c_ulong;
pub type uint_fast32_t = ::libc::c_ulong;
pub type uint_fast64_t = ::libc::c_ulong;
pub type intptr_t = ::libc::c_long;
pub type uintptr_t = ::libc::c_ulong;
pub type intmax_t = ::libc::c_long;
pub type uintmax_t = ::libc::c_ulong;
pub type ptrdiff_t = ::libc::c_long;
pub type size_t = ::libc::c_ulong;
pub type wchar_t = ::libc::c_int;

pub enum Struct_cram_fd { }
pub enum Struct_hFILE { }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct___kstring_t {
    pub l: size_t,
    pub m: size_t,
    pub s: *mut ::libc::c_char,
}
impl ::std::default::Default for Struct___kstring_t {
    fn default() -> Struct___kstring_t { unsafe { ::std::mem::zeroed() } }
}
pub type kstring_t = Struct___kstring_t;
pub type Enum_htsFormatCategory = ::libc::c_uint;
pub const unknown_category: ::libc::c_uint = 0;
pub const sequence_data: ::libc::c_uint = 1;
pub const variant_data: ::libc::c_uint = 2;
pub const index_file: ::libc::c_uint = 3;
pub const region_list: ::libc::c_uint = 4;
pub const category_maximum: ::libc::c_uint = 32767;
pub type Enum_htsExactFormat = ::libc::c_uint;
pub const unknown_format: ::libc::c_uint = 0;
pub const binary_format: ::libc::c_uint = 1;
pub const text_format: ::libc::c_uint = 2;
pub const sam: ::libc::c_uint = 3;
pub const bam: ::libc::c_uint = 4;
pub const bai: ::libc::c_uint = 5;
pub const cram: ::libc::c_uint = 6;
pub const crai: ::libc::c_uint = 7;
pub const vcf: ::libc::c_uint = 8;
pub const bcf: ::libc::c_uint = 9;
pub const csi: ::libc::c_uint = 10;
pub const gzi: ::libc::c_uint = 11;
pub const tbi: ::libc::c_uint = 12;
pub const bed: ::libc::c_uint = 13;
pub const format_maximum: ::libc::c_uint = 32767;
pub type Enum_htsCompression = ::libc::c_uint;
pub const no_compression: ::libc::c_uint = 0;
pub const gzip: ::libc::c_uint = 1;
pub const bgzf: ::libc::c_uint = 2;
pub const custom: ::libc::c_uint = 3;
pub const compression_maximum: ::libc::c_uint = 32767;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_htsFormat {
    pub category: Enum_htsFormatCategory,
    pub format: Enum_htsExactFormat,
    pub version: Struct_Unnamed1,
    pub compression: Enum_htsCompression,
    pub compression_level: ::libc::c_short,
    pub specific: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_htsFormat {
    fn default() -> Struct_htsFormat { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed1 {
    pub major: ::libc::c_short,
    pub minor: ::libc::c_short,
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type htsFormat = Struct_htsFormat;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed2 {
    pub isbin_isright_isbe_iscram_dummy : uint32_t,
    pub lineno: int64_t,
    pub line: kstring_t,
    pub _fn: *mut ::libc::c_char,
    pub fn_aux: *mut ::libc::c_char,
    pub fp: Union_Unnamed3,
    pub format: htsFormat,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Struct_Unnamed2 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed3 {
    pub _bindgen_data_: [u64; 1],
}
impl Union_Unnamed3 {
    pub unsafe fn bgzf(&mut self) -> *mut *mut BGZF {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn cram(&mut self) -> *mut *mut Struct_cram_fd {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn hfile(&mut self) -> *mut *mut Struct_hFILE {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn voidp(&mut self) -> *mut *mut ::libc::c_void {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed3 {
    fn default() -> Union_Unnamed3 { unsafe { ::std::mem::zeroed() } }
}
pub type htsFile = Struct_Unnamed2;
pub type Enum_sam_fields = ::libc::c_uint;
pub const SAM_QNAME: ::libc::c_uint = 1;
pub const SAM_FLAG: ::libc::c_uint = 2;
pub const SAM_RNAME: ::libc::c_uint = 4;
pub const SAM_POS: ::libc::c_uint = 8;
pub const SAM_MAPQ: ::libc::c_uint = 16;
pub const SAM_CIGAR: ::libc::c_uint = 32;
pub const SAM_RNEXT: ::libc::c_uint = 64;
pub const SAM_PNEXT: ::libc::c_uint = 128;
pub const SAM_TLEN: ::libc::c_uint = 256;
pub const SAM_SEQ: ::libc::c_uint = 512;
pub const SAM_QUAL: ::libc::c_uint = 1024;
pub const SAM_AUX: ::libc::c_uint = 2048;
pub const SAM_RGAUX: ::libc::c_uint = 4096;
pub type Enum_cram_option = ::libc::c_uint;
pub const CRAM_OPT_DECODE_MD: ::libc::c_uint = 0;
pub const CRAM_OPT_PREFIX: ::libc::c_uint = 1;
pub const CRAM_OPT_VERBOSITY: ::libc::c_uint = 2;
pub const CRAM_OPT_SEQS_PER_SLICE: ::libc::c_uint = 3;
pub const CRAM_OPT_SLICES_PER_CONTAINER: ::libc::c_uint = 4;
pub const CRAM_OPT_RANGE: ::libc::c_uint = 5;
pub const CRAM_OPT_VERSION: ::libc::c_uint = 6;
pub const CRAM_OPT_EMBED_REF: ::libc::c_uint = 7;
pub const CRAM_OPT_IGNORE_MD5: ::libc::c_uint = 8;
pub const CRAM_OPT_REFERENCE: ::libc::c_uint = 9;
pub const CRAM_OPT_MULTI_SEQ_PER_SLICE: ::libc::c_uint = 10;
pub const CRAM_OPT_NO_REF: ::libc::c_uint = 11;
pub const CRAM_OPT_USE_BZIP2: ::libc::c_uint = 12;
pub const CRAM_OPT_SHARED_REF: ::libc::c_uint = 13;
pub const CRAM_OPT_NTHREADS: ::libc::c_uint = 14;
pub const CRAM_OPT_THREAD_POOL: ::libc::c_uint = 15;
pub const CRAM_OPT_USE_LZMA: ::libc::c_uint = 16;
pub const CRAM_OPT_USE_RANS: ::libc::c_uint = 17;
pub const CRAM_OPT_REQUIRED_FIELDS: ::libc::c_uint = 18;
pub enum Struct___hts_idx_t { }
pub type hts_idx_t = Struct___hts_idx_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed4 {
    pub u: uint64_t,
    pub v: uint64_t,
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Struct_Unnamed4 { unsafe { ::std::mem::zeroed() } }
}
pub type hts_pair64_t = Struct_Unnamed4;
pub type hts_readrec_func =
    extern "C" fn
        (fp: *mut BGZF, data: *mut ::libc::c_void, r: *mut ::libc::c_void,
         tid: *mut ::libc::c_int, beg: *mut ::libc::c_int,
         end: *mut ::libc::c_int) -> ::libc::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed5 {
    pub readres_finished_dummy: uint32_t,
    pub tid: ::libc::c_int,
    pub beg: ::libc::c_int,
    pub end: ::libc::c_int,
    pub n_off: ::libc::c_int,
    pub i: ::libc::c_int,
    pub curr_tid: ::libc::c_int,
    pub curr_beg: ::libc::c_int,
    pub curr_end: ::libc::c_int,
    pub curr_off: uint64_t,
    pub off: *mut hts_pair64_t,
    pub readrec: *mut ::std::option::Option<extern "C" fn() -> ::libc::c_int>,
    pub bins: Struct_Unnamed6,
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Struct_Unnamed5 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed6 {
    pub n: ::libc::c_int,
    pub m: ::libc::c_int,
    pub a: *mut ::libc::c_int,
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Struct_Unnamed6 { unsafe { ::std::mem::zeroed() } }
}
pub type hts_itr_t = Struct_Unnamed5;
pub type hts_name2id_f =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut ::libc::c_void,
                               arg2: *const ::libc::c_char) -> ::libc::c_int>;
pub type hts_id2name_f =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut ::libc::c_void, arg2: ::libc::c_int)
                              -> *const ::libc::c_char>;
pub type hts_itr_query_func =
    extern "C" fn
        (idx: *const hts_idx_t, tid: ::libc::c_int, beg: ::libc::c_int,
         end: ::libc::c_int,
         readrec:
             *mut ::std::option::Option<extern "C" fn() -> ::libc::c_int>)
        -> *mut hts_itr_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed7 {
    pub n_targets: int32_t,
    pub ignore_sam_err: int32_t,
    pub l_text: uint32_t,
    pub target_len: *mut uint32_t,
    pub cigar_tab: *mut int8_t,
    pub target_name: *mut *mut ::libc::c_char,
    pub text: *mut ::libc::c_char,
    pub sdict: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Struct_Unnamed7 { unsafe { ::std::mem::zeroed() } }
}
pub type bam_hdr_t = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed8 {
    pub tid: int32_t,
    pub pos: int32_t,
    pub bin: uint16_t,
    pub qual: uint8_t,
    pub l_qname: uint8_t,
    pub flag: uint16_t,
    pub n_cigar: uint16_t,
    pub l_qseq: int32_t,
    pub mtid: int32_t,
    pub mpos: int32_t,
    pub isize: int32_t,
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Struct_Unnamed8 { unsafe { ::std::mem::zeroed() } }
}
pub type bam1_core_t = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed9 {
    pub core: bam1_core_t,
    pub l_data: ::libc::c_int,
    pub m_data: ::libc::c_int,
    pub data: *mut uint8_t,
    pub id: uint64_t,
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Struct_Unnamed9 { unsafe { ::std::mem::zeroed() } }
}
pub type bam1_t = Struct_Unnamed9;
pub type samFile = htsFile;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bam_pileup_cd {
    pub data: [u64; 1usize] // this can be a pointer, an int64 or a double
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Struct_Unnamed10 {
    pub b: *mut bam1_t,
    pub qpos: int32_t,
    pub indel: ::libc::c_int,
    pub level: ::libc::c_int,
    pub isdel_ishead_istail_isrefskip_isaux: uint32_t
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Struct_Unnamed10 { unsafe { ::std::mem::zeroed() } }
}
pub type bam_pileup1_t = Struct_Unnamed10;
pub type bam_plp_auto_f =
    ::std::option::Option<extern "C" fn
                              (data: *mut ::libc::c_void, b: *mut bam1_t)
                              -> ::libc::c_int>;
pub enum Struct___bam_plp_t { }
pub type bam_plp_t = *mut Struct___bam_plp_t;
pub enum Struct___bam_mplp_t { }
pub type bam_mplp_t = *mut Struct___bam_mplp_t;
extern "C" {
    pub static mut hts_verbose: ::libc::c_int;
    pub static mut seq_nt16_table: [::libc::c_uchar; 256];
    pub static mut seq_nt16_str: *const ::libc::c_char;
    pub static mut seq_nt16_int: *const ::libc::c_int;
}

extern "C" {
    pub fn bgzf_open(_fn: *const ::libc::c_char, mode: *const ::libc::c_char) -> *mut BGZF;
    pub fn bgzf_seek(fp: *mut BGZF, pos: int64_t, relative: ::libc::c_int) -> int64_t;
    pub fn hts_version() -> *const ::libc::c_char;
    pub fn hts_detect_format(fp: *mut Struct_hFILE, fmt: *mut htsFormat)
     -> ::libc::c_int;
    pub fn hts_format_description(format: *const htsFormat)
     -> *mut ::libc::c_char;
    pub fn hts_open(_fn: *const ::libc::c_char, mode: *const ::libc::c_char)
     -> *mut htsFile;
    pub fn hts_hopen(fp: *mut Struct_hFILE, _fn: *const ::libc::c_char,
                     mode: *const ::libc::c_char) -> *mut htsFile;
    pub fn hts_close(fp: *mut htsFile) -> ::libc::c_int;
    pub fn hts_get_format(fp: *mut htsFile) -> *const htsFormat;
    pub fn hts_set_opt(fp: *mut htsFile, opt: Enum_cram_option, ...)
     -> ::libc::c_int;
    pub fn hts_getline(fp: *mut htsFile, delimiter: ::libc::c_int,
                       str: *mut kstring_t) -> ::libc::c_int;
    pub fn hts_readlines(_fn: *const ::libc::c_char, _n: *mut ::libc::c_int)
     -> *mut *mut ::libc::c_char;
    pub fn hts_readlist(_fn: *const ::libc::c_char, is_file: ::libc::c_int,
                        _n: *mut ::libc::c_int) -> *mut *mut ::libc::c_char;
    pub fn hts_set_threads(fp: *mut htsFile, n: ::libc::c_int)
     -> ::libc::c_int;
    pub fn hts_set_fai_filename(fp: *mut htsFile,
                                fn_aux: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn hts_idx_init(n: ::libc::c_int, fmt: ::libc::c_int,
                        offset0: uint64_t, min_shift: ::libc::c_int,
                        n_lvls: ::libc::c_int) -> *mut hts_idx_t;
    pub fn hts_idx_destroy(idx: *mut hts_idx_t) -> ();
    pub fn hts_idx_push(idx: *mut hts_idx_t, tid: ::libc::c_int,
                        beg: ::libc::c_int, end: ::libc::c_int,
                        offset: uint64_t, is_mapped: ::libc::c_int)
     -> ::libc::c_int;
    pub fn hts_idx_finish(idx: *mut hts_idx_t, final_offset: uint64_t) -> ();
    pub fn hts_idx_save(idx: *const hts_idx_t, _fn: *const ::libc::c_char,
                        fmt: ::libc::c_int) -> ();
    pub fn hts_idx_load(_fn: *const ::libc::c_char, fmt: ::libc::c_int)
     -> *mut hts_idx_t;
    pub fn hts_idx_get_meta(idx: *mut hts_idx_t, l_meta: *mut ::libc::c_int)
     -> *mut uint8_t;
    pub fn hts_idx_set_meta(idx: *mut hts_idx_t, l_meta: ::libc::c_int,
                            meta: *mut uint8_t, is_copy: ::libc::c_int) -> ();
    pub fn hts_idx_get_stat(idx: *const hts_idx_t, tid: ::libc::c_int,
                            mapped: *mut uint64_t, unmapped: *mut uint64_t)
     -> ::libc::c_int;
    pub fn hts_idx_get_n_no_coor(idx: *const hts_idx_t) -> uint64_t;
    pub fn hts_parse_reg(s: *const ::libc::c_char, beg: *mut ::libc::c_int,
                         end: *mut ::libc::c_int) -> *const ::libc::c_char;
    pub fn hts_itr_query(idx: *const hts_idx_t, tid: ::libc::c_int,
                         beg: ::libc::c_int, end: ::libc::c_int,
                         readrec:
                             *mut ::std::option::Option<extern "C" fn()
                                                            -> ::libc::c_int>)
     -> *mut hts_itr_t;
    pub fn hts_itr_destroy(iter: *mut hts_itr_t) -> ();
    pub fn hts_itr_querys(idx: *const hts_idx_t, reg: *const ::libc::c_char,
                          getid: hts_name2id_f, hdr: *mut ::libc::c_void,
                          itr_query:
                              *mut ::std::option::Option<extern "C" fn()
                                                             ->
                                                                 *mut hts_itr_t>,
                          readrec:
                              *mut ::std::option::Option<extern "C" fn()
                                                             ->
                                                                 ::libc::c_int>)
     -> *mut hts_itr_t;
    pub fn hts_itr_next(fp: *mut BGZF, iter: *mut hts_itr_t,
                        r: *mut ::libc::c_void, data: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn hts_idx_seqnames(idx: *const hts_idx_t, n: *mut ::libc::c_int,
                            getid: hts_id2name_f, hdr: *mut ::libc::c_void)
     -> *mut *const ::libc::c_char;
    pub fn hts_file_type(fname: *const ::libc::c_char) -> ::libc::c_int;
    pub fn bam_hdr_init() -> *mut bam_hdr_t;
    pub fn bam_hdr_read(fp: *mut BGZF) -> *mut bam_hdr_t;
    pub fn bam_hdr_write(fp: *mut BGZF, h: *const bam_hdr_t) -> ::libc::c_int;
    pub fn bam_hdr_destroy(h: *mut bam_hdr_t) -> ();
    pub fn bam_name2id(h: *mut bam_hdr_t, _ref: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn bam_hdr_dup(h0: *const bam_hdr_t) -> *mut bam_hdr_t;
    pub fn bam_init1() -> *mut bam1_t;
    pub fn bam_destroy1(b: *mut bam1_t) -> ();
    pub fn bam_read1(fp: *mut BGZF, b: *mut bam1_t) -> ::libc::c_int;
    pub fn bam_write1(fp: *mut BGZF, b: *const bam1_t) -> ::libc::c_int;
    pub fn bam_copy1(bdst: *mut bam1_t, bsrc: *const bam1_t) -> *mut bam1_t;
    pub fn bam_dup1(bsrc: *const bam1_t) -> *mut bam1_t;
    pub fn bam_cigar2qlen(n_cigar: ::libc::c_int, cigar: *const uint32_t)
     -> ::libc::c_int;
    pub fn bam_cigar2rlen(n_cigar: ::libc::c_int, cigar: *const uint32_t)
     -> ::libc::c_int;
    pub fn bam_endpos(b: *const bam1_t) -> int32_t;
    pub fn bam_str2flag(str: *const ::libc::c_char) -> ::libc::c_int;
    pub fn bam_flag2str(flag: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn bam_index_build(_fn: *const ::libc::c_char,
                           min_shift: ::libc::c_int) -> ::libc::c_int;
    pub fn sam_index_load(fp: *mut htsFile, _fn: *const ::libc::c_char)
     -> *mut hts_idx_t;
    pub fn sam_itr_queryi(idx: *const hts_idx_t, tid: ::libc::c_int,
                          beg: ::libc::c_int, end: ::libc::c_int)
     -> *mut hts_itr_t;
    pub fn sam_itr_querys(idx: *const hts_idx_t, hdr: *mut bam_hdr_t,
                          region: *const ::libc::c_char) -> *mut hts_itr_t;
    pub fn sam_open_mode(mode: *mut ::libc::c_char,
                         _fn: *const ::libc::c_char,
                         format: *const ::libc::c_char) -> ::libc::c_int;
    pub fn sam_hdr_parse(l_text: ::libc::c_int, text: *const ::libc::c_char)
     -> *mut bam_hdr_t;
    pub fn sam_hdr_read(fp: *mut samFile) -> *mut bam_hdr_t;
    pub fn sam_hdr_write(fp: *mut samFile, h: *const bam_hdr_t)
     -> ::libc::c_int;
    pub fn sam_parse1(s: *mut kstring_t, h: *mut bam_hdr_t, b: *mut bam1_t)
     -> ::libc::c_int;
    pub fn sam_format1(h: *const bam_hdr_t, b: *const bam1_t,
                       str: *mut kstring_t) -> ::libc::c_int;
    pub fn sam_read1(fp: *mut samFile, h: *mut bam_hdr_t, b: *mut bam1_t)
     -> ::libc::c_int;
    pub fn sam_write1(fp: *mut samFile, h: *const bam_hdr_t, b: *const bam1_t)
     -> ::libc::c_int;
    pub fn bam_aux_get(b: *const bam1_t, tag: *mut ::libc::c_char)
     -> *mut uint8_t;
    pub fn bam_aux2i(s: *const uint8_t) -> int64_t;
    pub fn bam_aux2f(s: *const uint8_t) -> ::libc::c_double;
    pub fn bam_aux2A(s: *const uint8_t) -> ::libc::c_char;
    pub fn bam_aux2Z(s: *const uint8_t) -> *mut ::libc::c_char;
    pub fn bam_aux_append(b: *mut bam1_t, tag: *mut ::libc::c_char,
                          _type: ::libc::c_char, len: ::libc::c_int,
                          data: *mut uint8_t) -> ();
    pub fn bam_aux_del(b: *mut bam1_t, s: *mut uint8_t) -> ::libc::c_int;
    pub fn bam_plp_init(func: bam_plp_auto_f, data: *mut ::libc::c_void)
     -> bam_plp_t;
    pub fn bam_plp_destroy(iter: bam_plp_t) -> ();
    pub fn bam_plp_push(iter: bam_plp_t, b: *const bam1_t) -> ::libc::c_int;
    pub fn bam_plp_next(iter: bam_plp_t, _tid: *mut ::libc::c_int,
                        _pos: *mut ::libc::c_int, _n_plp: *mut ::libc::c_int)
     -> *const bam_pileup1_t;
    pub fn bam_plp_auto(iter: bam_plp_t, _tid: *mut ::libc::c_int,
                        _pos: *mut ::libc::c_int, _n_plp: *mut ::libc::c_int)
     -> *const bam_pileup1_t;
    pub fn bam_plp_set_maxcnt(iter: bam_plp_t, maxcnt: ::libc::c_int) -> ();
    pub fn bam_plp_reset(iter: bam_plp_t) -> ();
    pub fn bam_mplp_init(n: ::libc::c_int, func: bam_plp_auto_f,
                         data: *mut *mut ::libc::c_void) -> bam_mplp_t;
    pub fn bam_mplp_init_overlaps(iter: bam_mplp_t) -> ();
    pub fn bam_mplp_destroy(iter: bam_mplp_t) -> ();
    pub fn bam_mplp_set_maxcnt(iter: bam_mplp_t, maxcnt: ::libc::c_int) -> ();
    pub fn bam_mplp_auto(iter: bam_mplp_t, _tid: *mut ::libc::c_int,
                         _pos: *mut ::libc::c_int, n_plp: *mut ::libc::c_int,
                         plp: *mut *const bam_pileup1_t) -> ::libc::c_int;
}
