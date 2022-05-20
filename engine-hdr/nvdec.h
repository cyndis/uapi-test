#include <stdint.h>

typedef struct _nvdec_display_param_s {
    uint32_t enableTFOutput : 1;
    uint32_t VC1MapYFlag : 1;
    uint32_t MapYValue : 3;
    uint32_t VC1MapUVFlag : 1;
    uint32_t MapUVValue : 3;
    uint32_t OutStride : 8;
    uint32_t TilingFormat : 3;
    uint32_t OutputStructure : 1;
    uint32_t reserved0 : 11;
    int32_t OutputTop[2];
    int32_t OutputBottom[2];
    uint32_t enableHistogram : 1;
    uint32_t HistogramStartX : 12;
    uint32_t HistogramStartY : 12;
    uint32_t reserved1 : 7;
    uint32_t HistogramEndX : 12;
    uint32_t HistogramEndY : 12;
    uint32_t reserved2 : 8;
} nvdec_display_param_s;

typedef struct _nvdec_mpeg2_pic_s {
    uint32_t reserved0[13];
    uint8_t eos[16];
    uint8_t explicitEOSPresentFlag;
    uint8_t reserved1[3];
    uint32_t stream_len;
    uint32_t slice_count;
    uint32_t gptimer_timeout_value;

    // Fields from vld_mpeg2_seq_pic_info_s
    uint16_t FrameWidth; // actual frame width
    uint16_t FrameHeight; // actual frame height
    uint8_t picture_structure; // 0 => Reserved, 1 => Top field, 2 => Bottom field, 3 => Frame picture. Table 6-14.
    uint8_t picture_coding_type; // 0 => Forbidden, 1 => I, 2 => P, 3 => B, 4 => D (for MPEG-2). Table 6-12.
    uint8_t intra_dc_precision; // 0 => 8 bits, 1=> 9 bits, 2 => 10 bits, 3 => 11 bits. Table 6-13.
    int8_t frame_pred_frame_dct; // as in section 6.3.10
    int8_t concealment_motion_vectors; // as in section 6.3.10
    int8_t intra_vlc_format; // as in section 6.3.10
    uint8_t tileFormat : 2; // 0: TBL; 1: KBL;
    uint8_t gob_height : 3; // Set GOB height, 0: GOB_2, 1: GOB_4, 2: GOB_8, 3: GOB_16, 4: GOB_32
    uint8_t reserverd_surface_format : 3;

    int8_t reserved2; // always 0
    int8_t f_code[4]; // as in section 6.3.10

    uint16_t PicWidthInMbs;
    uint16_t FrameHeightInMbs;
    uint32_t pitch_luma;
    uint32_t pitch_chroma;
    uint32_t luma_top_offset;
    uint32_t luma_bot_offset;
    uint32_t luma_frame_offset;
    uint32_t chroma_top_offset;
    uint32_t chroma_bot_offset;
    uint32_t chroma_frame_offset;
    uint32_t HistBufferSize;
    uint16_t output_memory_layout;
    uint16_t alternate_scan;
    uint16_t secondfield;
    uint16_t rounding_type;
    uint32_t MbInfoSizeInBytes;
    uint32_t q_scale_type;
    uint32_t top_field_first;
    uint32_t full_pel_fwd_vector;
    uint32_t full_pel_bwd_vector;
    uint8_t quant_mat_8x8intra[64];
    uint8_t quant_mat_8x8nonintra[64];
    uint32_t ref_memory_layout[2]; //0:for fwd; 1:for bwd

    nvdec_display_param_s displayPara;
    uint32_t reserved3[3];
} nvdec_mpeg2_pic_s;
