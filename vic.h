#include <stdint.h>

typedef struct _OutputConfig {
    uint64_t AlphaFillMode       : 3;    // 2..0
    uint64_t AlphaFillSlot       : 3;    // 5..3
    uint64_t BackgroundAlpha     : 10;   // 15..6
    uint64_t BackgroundR         : 10;   // 25..16
    uint64_t BackgroundG         : 10;   // 35..26
    uint64_t BackgroundB         : 10;   // 45..36
    uint64_t RegammaMode         : 2;    // 47..46
    uint64_t OutputFlipX         : 1;    // 48
    uint64_t OutputFlipY         : 1;    // 49
    uint64_t OutputTranspose     : 1;    // 50
    uint64_t reserved1           : 1;    // 51
    uint64_t reserved2           : 12;   // 63..52
    uint64_t TargetRectLeft      : 14;   // 77..64
    uint64_t reserved3           : 2;    // 79..78
    uint64_t TargetRectRight     : 14;   // 93..80
    uint64_t reserved4           : 2;    // 95..94
    uint64_t TargetRectTop       : 14;   // 109..96
    uint64_t reserved5           : 2;    // 111..110
    uint64_t TargetRectBottom    : 14;   // 125..112
    uint64_t reserved6           : 2;    // 127..126
} OutputConfig;

typedef struct _OutputSurfaceConfig {
    uint64_t OutPixelFormat      : 7;    // 6..0
    uint64_t OutChromaLocHoriz   : 2;    // 8..7
    uint64_t OutChromaLocVert    : 2;    // 10..9
    uint64_t OutBlkKind          : 4;    // 14..11
    uint64_t OutBlkHeight        : 4;    // 18..15
    uint64_t reserved0           : 3;    // 21..19
    uint64_t reserved1           : 10;   // 31..22
    uint64_t OutSurfaceWidth     : 14;   // 45..32
    uint64_t OutSurfaceHeight    : 14;   // 59..46
    uint64_t reserved2           : 4;    // 63..60
    uint64_t OutLumaWidth        : 14;   // 77..64
    uint64_t OutLumaHeight       : 14;   // 91..78
    uint64_t reserved3           : 4;    // 95..92
    uint64_t OutChromaWidth      : 14;   // 109..96
    uint64_t OutChromaHeight     : 14;   // 123..110
    uint64_t reserved4           : 4;    // 127..124
} OutputSurfaceConfig;
