// Copyright © 2015, Jordan Miner
// Licensed under the MIT License <LICENSE.md>
//! Uniscribe structure declarations and constant definitions

pub const SCRIPT_UNDEFINED: ::WORD = 0;

pub const USP_E_SCRIPT_NOT_IN_FONT: ::HRESULT = MAKE_HRESULT!(
    ::SEVERITY_ERROR, ::FACILITY_ITF, 0x200
);

DECLARE_HANDLE!(SCRIPT_CACHE, SCRIPT_CACHE__);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_CONTROL {
    bit_fields: ::DWORD,
}
BITFIELD!(SCRIPT_CONTROL bit_fields: ::DWORD [
    uDefaultLanguage set_uDefaultLanguage[0..16],
    fContextDigits set_fContextDigits[16..17],
    fInvertPreBoundDir set_fInvertPreBoundDir[17..18],
    fInvertPostBoundDir set_fInvertPostBoundDir[18..19],
    fLinkStringBefore set_fLinkStringBefore[19..20],
    fLinkStringAfter set_fLinkStringAfter[20..21],
    fNeutralOverride set_fNeutralOverride[21..22],
    fNumericOverride set_fNumericOverride[22..23],
    fLegacyBidiClass set_fLegacyBidiClass[23..24],
    fMergeNeutralItems set_fMergeNeutralItems[24..25],
    fReserved set_fReserved[25..32],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_STATE {
    bit_fields: ::WORD,
}
BITFIELD!(SCRIPT_STATE bit_fields: ::WORD [
    uBidiLevel set_uBidiLevel[0..5],
    fOverrideDirection set_fOverrideDirection[5..6],
    fInhibitSymSwap set_fInhibitSymSwap[6..7],
    fCharShape set_fCharShape[7..8],
    fDigitSubstitute set_fDigitSubstitute[8..9],
    fInhibitLigate set_fInhibitLigate[9..10],
    fDisplayZWG set_fDisplayZWG[10..11],
    fArabicNumContext set_fArabicNumContext[11..12],
    fGcpClusters set_fGcpClusters[12..13],
    fReserved set_fReserved[13..14],
    fEngineReserved set_fEngineReserved[14..16],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_ANALYSIS {
    bit_fields: ::WORD,
    pub s: SCRIPT_STATE,
}
BITFIELD!(SCRIPT_ANALYSIS bit_fields: ::WORD [
    eScript set_eScript[0..10],
    fRTL set_fRTL[10..11],
    fLayoutRTL set_fLayoutRTL[11..12],
    fLinkBefore set_fLinkBefore[12..13],
    fLinkAfter set_fLinkAfter[13..14],
    fLogicalOrder set_fLogicalOrder[14..15],
    fNoGlyphIndex set_fNoGlyphIndex[15..16],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_ITEM {
    pub iCharPos: ::c_int,
    pub a: SCRIPT_ANALYSIS,
}

//490
pub const SCRIPT_JUSTIFY_NONE: ::WORD = 0;
pub const SCRIPT_JUSTIFY_ARABIC_BLANK: ::WORD = 1;
pub const SCRIPT_JUSTIFY_CHARACTER: ::WORD = 2;
pub const SCRIPT_JUSTIFY_RESERVED1: ::WORD = 3;
pub const SCRIPT_JUSTIFY_BLANK: ::WORD = 4;
pub const SCRIPT_JUSTIFY_RESERVED2: ::WORD = 5;
pub const SCRIPT_JUSTIFY_RESERVED3: ::WORD = 6;
pub const SCRIPT_JUSTIFY_ARABIC_NORMAL: ::WORD = 7;
pub const SCRIPT_JUSTIFY_ARABIC_KASHIDA: ::WORD = 8;
pub const SCRIPT_JUSTIFY_ARABIC_ALEF: ::WORD = 9;
pub const SCRIPT_JUSTIFY_ARABIC_HA: ::WORD = 10;
pub const SCRIPT_JUSTIFY_ARABIC_RA: ::WORD = 11;
pub const SCRIPT_JUSTIFY_ARABIC_BA: ::WORD = 12;
pub const SCRIPT_JUSTIFY_ARABIC_BARA: ::WORD = 13;
pub const SCRIPT_JUSTIFY_ARABIC_SEEN: ::WORD = 14;
pub const SCRIPT_JUSTIFY_ARABIC_SEEN_M: ::WORD = 15;


#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_VISATTR {
    bit_fields: ::WORD,
}
BITFIELD!(SCRIPT_VISATTR bit_fields: ::WORD [
    uJustification set_uJustification[0..4],
    fClusterStart set_fClusterStart[4..5],
    fDiacritic set_fDiacritic[5..6],
    fZeroWidth set_fZeroWidth[6..7],
    fReserved set_fReserved[7..8],
    fShapeReserved set_fShapeReserved[8..16],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct GOFFSET {
    pub du: ::LONG,
    pub dv: ::LONG,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_LOGATTR {
    bit_fields: ::BYTE,

}
BITFIELD!(SCRIPT_LOGATTR bit_fields: ::BYTE [
    fSoftBreak set_fSoftBreak[0..1],
    fWhiteSpace set_fWhiteSpace[1..2],
    fCharStop set_fCharStop[2..3],
    fWordStop set_fWordStop[3..4],
    fInvalid set_fInvalid[4..5],
    fReserved set_fReserved[5..8],
]);

pub const SGCM_RTL: ::DWORD = 0x00000001;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_PROPERTIES {
    bit_fields1: ::DWORD,
    bit_fields2: ::DWORD,
}
BITFIELD!(SCRIPT_PROPERTIES bit_fields1: ::DWORD [
    langid set_langid[0..16],
    fNumeric set_fNumeric[16..17],
    fComplex set_fComplex[17..18],
    fNeedsWordBreaking set_fNeedsWordBreaking[18..19],
    fNeedsCaretInfo set_fNeedsCaretInfo[19..20],
    bCharSet set_bCharSet[20..28],
    fControl set_fControl[28..29],
    fPrivateUseArea set_fPrivateUseArea[29..30],
    fNeedsCharacterJustify set_fNeedsCharacterJustify[30..31],
    fInvalidGlyph set_fInvalidGlyph[31..32],
]);
BITFIELD!(SCRIPT_PROPERTIES bit_fields2: ::DWORD [
    fInvalidLogAttr set_fInvalidLogAttr[0..1],
    fCDM set_fCDM[1..2],
    fAmbiguousCharSet set_fAmbiguousCharSet[2..3],
    fClusterSizeVaries set_fClusterSizeVaries[3..4],
    fRejectInvalid set_fRejectInvalid[4..5],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_FONTPROPERTIES {
    pub cBytes: ::c_int,
    pub wgBlank: ::WORD,
    pub wgDefault: ::WORD,
    pub wgInvalid: ::WORD,
    pub wgKashida: ::WORD,
    pub iKashidaWidth: ::c_int,
}

//1440
pub const SSA_PASSWORD: ::DWORD = 0x00000001;
pub const SSA_TAB: ::DWORD = 0x00000002;
pub const SSA_CLIP: ::DWORD = 0x00000004;
pub const SSA_FIT: ::DWORD = 0x00000008;
pub const SSA_DZWG: ::DWORD = 0x00000010;
pub const SSA_FALLBACK: ::DWORD = 0x00000020;
pub const SSA_BREAK: ::DWORD = 0x00000040;
pub const SSA_GLYPHS: ::DWORD = 0x00000080;
pub const SSA_RTL: ::DWORD = 0x00000100;
pub const SSA_GCP: ::DWORD = 0x00000200;
pub const SSA_HOTKEY: ::DWORD = 0x00000400;
pub const SSA_METAFILE: ::DWORD = 0x00000800;
pub const SSA_LINK: ::DWORD = 0x00001000;
pub const SSA_HIDEHOTKEY: ::DWORD = 0x00002000;
pub const SSA_HOTKEYONLY: ::DWORD = 0x00002400;

pub const SSA_FULLMEASURE: ::DWORD = 0x04000000;
pub const SSA_LPKANSIFALLBACK: ::DWORD = 0x08000000;
pub const SSA_PIDX: ::DWORD = 0x10000000;
pub const SSA_LAYOUTRTL: ::DWORD = 0x20000000;
pub const SSA_DONTGLYPH: ::DWORD = 0x40000000;
pub const SSA_NOKASHIDA: ::DWORD = 0x80000000;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_TABDEF {
    pub cTabStops: ::c_int,
    pub iScale: ::c_int,
    pub pTabStops: *mut ::c_int,
    pub iTabOrigin: ::c_int,
}

DECLARE_HANDLE!(SCRIPT_STRING_ANALYSIS, SCRIPT_STRING_ANALYSIS__);

pub const SIC_COMPLEX: ::DWORD = 1;
pub const SIC_ASCIIDIGIT: ::DWORD = 2;
pub const SIC_NEUTRAL: ::DWORD = 4;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_DIGITSUBSTITUTE {
    bit_fields1: ::DWORD,
    bit_fields2: ::DWORD,
    pub dwReserved: ::DWORD,
}
BITFIELD!(SCRIPT_DIGITSUBSTITUTE bit_fields1: ::DWORD [
    NationalDigitLanguage set_NationalDigitLanguage[0..16],
    TraditionalDigitLanguage set_TraditionalDigitLanguage[16..32],
]);
BITFIELD!(SCRIPT_DIGITSUBSTITUTE bit_fields2: ::DWORD [
    DigitSubstitute set_DigitSubstitute[0..8],
]);

pub const SCRIPT_DIGITSUBSTITUTE_CONTEXT: ::BYTE = 0;
pub const SCRIPT_DIGITSUBSTITUTE_NONE: ::BYTE = 1;
pub const SCRIPT_DIGITSUBSTITUTE_NATIONAL: ::BYTE = 2;
pub const SCRIPT_DIGITSUBSTITUTE_TRADITIONAL: ::BYTE = 3;

pub type OPENTYPE_TAG = ::ULONG;

pub const SCRIPT_TAG_UNKNOWN: OPENTYPE_TAG = 0x00000000;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct OPENTYPE_FEATURE_RECORD {
    pub tagFeature: OPENTYPE_TAG,
    pub lParameter: ::LONG,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TEXTRANGE_PROPERTIES {
    pub potfRecords: *mut OPENTYPE_FEATURE_RECORD,
    pub cotfRecords: ::c_int,
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_CHARPROP {
    bit_fields: ::WORD,
}
BITFIELD!(SCRIPT_CHARPROP bit_fields: ::WORD [
    fCanGlyphAlone set_fCanGlyphAlone[0..1],
    reserved set_reserved[1..16],
]);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct SCRIPT_GLYPHPROP {
    pub sva: SCRIPT_VISATTR,
    pub reserved: ::WORD,
}
