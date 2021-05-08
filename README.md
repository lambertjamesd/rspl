# Reality Signal Processor Language

This document is a design for RSPL, or Reality Signal Processor Language. A language made specifically for the RSP fo the Nintendo 64. Its main design goals are to stay low level enough to have good performance while hiding quirks of tasks, such as vector multiplies, behind easy to understand abstractions.

# Compatiblity with C

The language will be C like. Type definitions such as struct, union, enum, typedef, and global variables will be compatible with C meaning you can share structure definitions and global variables with C code running on the CPU.

```C
struct Point {
    u32 x;
    u32 y;
};

enum EnumExample {
    ENUM_EXAMPLE_0 = 0,
    ENUM_EXAMPLE_1 = 1,
};

extern struct Point gPointGlobal;

```

Global variables by default are assumed to reside in main system memory. RSPL cannot directly access these variables but it can know the physical address of these variables. To use them RSPL will need to use `dma` to transfer memory into DMEM first.

```C

extern struct Point gPointGlobal;
struct Point DMEM gPointDMEM;

void main() {
    // copies from gPointGlobal from main memory into DMEM
    dmaRead(&gPointDMEM, &gPointGlobal, sizeof(struct Point));

    // wait for current DMAs to finish
    dmaWait();

    u32 tmp = gPointDMEM.x;
    gPointDMEM.x = gPointDMEM.y;
    gPointDMEM.y = tmp;

    // copy back into main memory
    dmaWrite(&gPointGlobal, &gPointDMEM, sizeof(struct Point));
}

```

## C Preprocessor

RSPL will not implement a preprocessor. If you wish to use one you can just use GCC. You should define use the following definiton when running the proprocessor for RSPL
```C
#define __RSPL__ 1
``` 

## Linkable with C and ASM

RSPL will be linkable with C so addresses to global variables residing in main memory will be accessable from RSPL.

RSPL will be linkable with other RSPL files and even RSP assembly. This will let you mix RSPL with assembly routines. 

# Data Types

Data types will be limited to what is native to the RSP. Any data types that cannot be used natively, such as floating point numbers, will not be supported by RSPL. The following primitive data types will be suppported

| Type Name | Bytes | Description                    |
|:----------|:-----:|:-------------------------------|
| u64       | 8     | Unsigned 64 bit integer        |
| u32       | 4     | Unsigned 32 bit integer        |
| u16       | 2     | Unsigned 16 bit integer        |
| u8        | 1     | Unsigned 8 bit integer         |
| s64       | 8     | Signed 64 bit integer          |
| s32       | 4     | Signed 32 bit integer          |
| s16       | 2     | Signed 16 bit integer          |
| s8        | 1     | Signed 8 bit integer           |
| bool      | 1     | Has the value of true or false |
| type~     | 2     | An address in DMEM             |
| type*     | 4     | An address in main memory      |

There will also be vector types. All vector types are 8 elements wide.

| Type Name | Bytes | Description                    |
|:----------|:-----:|:-------------------------------|
| vu16      | 16    | Vector of unsigned 16 bit integers |
| vu32      | 32    | Vector of unsigned 32 bit integers |
| vs16      | 16    | Vector of signed 16 bit integers |
| vs32      | 32    | Vector of signed 32 bit integers |
| vf16      | 16    | Fixed point number with 16 fractional bits |
| vs16f16   | 16    | Fixed point number with 16 integer and 16 fractional bits |
| vbool     | 16    | Vector of booleans |

## Operator overloading

Arethmetic and bit operators will be overloaded for vector types.

```
u32 vectorDotProduct(vu16 a, vu16 b) {
    vu16 multiplyResult = a * b;
    return a[0] + a[1] + a[2];
}
```

## RDP Commands

RSPL will have built in functions to generate RDP commands

```
void generateRDPCommands(u32~ dl) {
    gDPPipeSync(dl++);
    gDPTileSync(dl++);
    gDPLoadSync(dl++);
    gDPSetColorImage(dl++, G_IM_FMT_RGBA, G_IM_SIZ_16b, SCREEN_WD, OS_K0_TO_PHYSICAL(colorBuffer));
    gDPPipeSync(dl++);
}

```