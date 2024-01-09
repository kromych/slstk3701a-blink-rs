#[doc = "Register `TFTCTRL` reader"]
pub type R = crate::R<TFTCTRL_SPEC>;
#[doc = "Register `TFTCTRL` writer"]
pub type W = crate::W<TFTCTRL_SPEC>;
#[doc = "Field `DD` reader - TFT Direct Drive Mode"]
pub type DD_R = crate::FieldReader<DD_A>;
#[doc = "TFT Direct Drive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DD_A {
    #[doc = "0: Direct Drive is disabled."]
    DISABLED = 0,
    #[doc = "1: Direct Drive from internal memory enabled and started."]
    INTERNAL = 1,
    #[doc = "2: Direct Drive from external memory enabled and started."]
    EXTERNAL = 2,
}
impl From<DD_A> for u8 {
    #[inline(always)]
    fn from(variant: DD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DD_A {
    type Ux = u8;
}
impl DD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DD_A> {
        match self.bits {
            0 => Some(DD_A::DISABLED),
            1 => Some(DD_A::INTERNAL),
            2 => Some(DD_A::EXTERNAL),
            _ => None,
        }
    }
    #[doc = "Direct Drive is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DD_A::DISABLED
    }
    #[doc = "Direct Drive from internal memory enabled and started."]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == DD_A::INTERNAL
    }
    #[doc = "Direct Drive from external memory enabled and started."]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == DD_A::EXTERNAL
    }
}
#[doc = "Field `DD` writer - TFT Direct Drive Mode"]
pub type DD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DD_A>;
impl<'a, REG> DD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct Drive is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DD_A::DISABLED)
    }
    #[doc = "Direct Drive from internal memory enabled and started."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(DD_A::INTERNAL)
    }
    #[doc = "Direct Drive from external memory enabled and started."]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(DD_A::EXTERNAL)
    }
}
#[doc = "Field `MASKBLEND` reader - TFT Mask and Blend Mode"]
pub type MASKBLEND_R = crate::FieldReader<MASKBLEND_A>;
#[doc = "TFT Mask and Blend Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKBLEND_A {
    #[doc = "0: Masking and Blending are disabled."]
    DISABLED = 0,
    #[doc = "1: Internal Masking is enabled."]
    IMASK = 1,
    #[doc = "2: Internal Alpha Blending is enabled."]
    IALPHA = 2,
    #[doc = "3: Internal Masking and Alpha Blending are enabled."]
    IMASKALPHA = 3,
    #[doc = "4: External Frame Buffer Masking is enabled."]
    EFBMASK = 4,
    #[doc = "5: External Frame Buffer Alpha Blending is enabled."]
    EFBALPHA = 5,
    #[doc = "6: External Frame Buffer Masking and Alpha Blending are enabled."]
    EFBMASKALPHA = 6,
    #[doc = "7: Internal Frame Buffer Masking is enabled."]
    IFBMASK = 7,
    #[doc = "8: Internal Frame Buffer Alpha Blending is enabled."]
    IFBALPHA = 8,
    #[doc = "9: Internal Frame Buffer Masking and Alpha Blending are enabled."]
    IFBMASKALPHA = 9,
}
impl From<MASKBLEND_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKBLEND_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MASKBLEND_A {
    type Ux = u8;
}
impl MASKBLEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MASKBLEND_A> {
        match self.bits {
            0 => Some(MASKBLEND_A::DISABLED),
            1 => Some(MASKBLEND_A::IMASK),
            2 => Some(MASKBLEND_A::IALPHA),
            3 => Some(MASKBLEND_A::IMASKALPHA),
            4 => Some(MASKBLEND_A::EFBMASK),
            5 => Some(MASKBLEND_A::EFBALPHA),
            6 => Some(MASKBLEND_A::EFBMASKALPHA),
            7 => Some(MASKBLEND_A::IFBMASK),
            8 => Some(MASKBLEND_A::IFBALPHA),
            9 => Some(MASKBLEND_A::IFBMASKALPHA),
            _ => None,
        }
    }
    #[doc = "Masking and Blending are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MASKBLEND_A::DISABLED
    }
    #[doc = "Internal Masking is enabled."]
    #[inline(always)]
    pub fn is_imask(&self) -> bool {
        *self == MASKBLEND_A::IMASK
    }
    #[doc = "Internal Alpha Blending is enabled."]
    #[inline(always)]
    pub fn is_ialpha(&self) -> bool {
        *self == MASKBLEND_A::IALPHA
    }
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn is_imaskalpha(&self) -> bool {
        *self == MASKBLEND_A::IMASKALPHA
    }
    #[doc = "External Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn is_efbmask(&self) -> bool {
        *self == MASKBLEND_A::EFBMASK
    }
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn is_efbalpha(&self) -> bool {
        *self == MASKBLEND_A::EFBALPHA
    }
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn is_efbmaskalpha(&self) -> bool {
        *self == MASKBLEND_A::EFBMASKALPHA
    }
    #[doc = "Internal Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn is_ifbmask(&self) -> bool {
        *self == MASKBLEND_A::IFBMASK
    }
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn is_ifbalpha(&self) -> bool {
        *self == MASKBLEND_A::IFBALPHA
    }
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn is_ifbmaskalpha(&self) -> bool {
        *self == MASKBLEND_A::IFBMASKALPHA
    }
}
#[doc = "Field `MASKBLEND` writer - TFT Mask and Blend Mode"]
pub type MASKBLEND_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MASKBLEND_A>;
impl<'a, REG> MASKBLEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masking and Blending are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::DISABLED)
    }
    #[doc = "Internal Masking is enabled."]
    #[inline(always)]
    pub fn imask(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::IMASK)
    }
    #[doc = "Internal Alpha Blending is enabled."]
    #[inline(always)]
    pub fn ialpha(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::IALPHA)
    }
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn imaskalpha(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::IMASKALPHA)
    }
    #[doc = "External Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn efbmask(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::EFBMASK)
    }
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn efbalpha(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::EFBALPHA)
    }
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn efbmaskalpha(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::EFBMASKALPHA)
    }
    #[doc = "Internal Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn ifbmask(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::IFBMASK)
    }
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn ifbalpha(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::IFBALPHA)
    }
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn ifbmaskalpha(self) -> &'a mut crate::W<REG> {
        self.variant(MASKBLEND_A::IFBMASKALPHA)
    }
}
#[doc = "Field `SHIFTDCLKEN` reader - TFT EBI_DCLK Shift Enable"]
pub type SHIFTDCLKEN_R = crate::BitReader;
#[doc = "Field `SHIFTDCLKEN` writer - TFT EBI_DCLK Shift Enable"]
pub type SHIFTDCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBCTRIG` reader - TFT Frame Base Copy Trigger"]
pub type FBCTRIG_R = crate::BitReader;
#[doc = "Field `FBCTRIG` writer - TFT Frame Base Copy Trigger"]
pub type FBCTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERLEAVE` reader - Interleave Mode"]
pub type INTERLEAVE_R = crate::FieldReader<INTERLEAVE_A>;
#[doc = "Interleave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTERLEAVE_A {
    #[doc = "0: Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    UNLIMITED = 0,
    #[doc = "1: Allow 1 interleaved EBI access per EBI_DCLK period."]
    ONEPERDCLK = 1,
    #[doc = "2: Only allow EBI accesses during TFT porches."]
    PORCH = 2,
}
impl From<INTERLEAVE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERLEAVE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTERLEAVE_A {
    type Ux = u8;
}
impl INTERLEAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INTERLEAVE_A> {
        match self.bits {
            0 => Some(INTERLEAVE_A::UNLIMITED),
            1 => Some(INTERLEAVE_A::ONEPERDCLK),
            2 => Some(INTERLEAVE_A::PORCH),
            _ => None,
        }
    }
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    #[inline(always)]
    pub fn is_unlimited(&self) -> bool {
        *self == INTERLEAVE_A::UNLIMITED
    }
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    #[inline(always)]
    pub fn is_oneperdclk(&self) -> bool {
        *self == INTERLEAVE_A::ONEPERDCLK
    }
    #[doc = "Only allow EBI accesses during TFT porches."]
    #[inline(always)]
    pub fn is_porch(&self) -> bool {
        *self == INTERLEAVE_A::PORCH
    }
}
#[doc = "Field `INTERLEAVE` writer - Interleave Mode"]
pub type INTERLEAVE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INTERLEAVE_A>;
impl<'a, REG> INTERLEAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    #[inline(always)]
    pub fn unlimited(self) -> &'a mut crate::W<REG> {
        self.variant(INTERLEAVE_A::UNLIMITED)
    }
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    #[inline(always)]
    pub fn oneperdclk(self) -> &'a mut crate::W<REG> {
        self.variant(INTERLEAVE_A::ONEPERDCLK)
    }
    #[doc = "Only allow EBI accesses during TFT porches."]
    #[inline(always)]
    pub fn porch(self) -> &'a mut crate::W<REG> {
        self.variant(INTERLEAVE_A::PORCH)
    }
}
#[doc = "Field `COLOR1SRC` reader - Masking/Alpha Blending Color1 Source"]
pub type COLOR1SRC_R = crate::BitReader;
#[doc = "Field `COLOR1SRC` writer - Masking/Alpha Blending Color1 Source"]
pub type COLOR1SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIDTH` reader - TFT Transaction Width"]
pub type WIDTH_R = crate::FieldReader<WIDTH_A>;
#[doc = "TFT Transaction Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WIDTH_A {
    #[doc = "0: TFT Data is 8 bit wide."]
    BYTE = 0,
    #[doc = "1: TFT Data is 16 bit wide."]
    HALFWORD = 1,
}
impl From<WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WIDTH_A {
    type Ux = u8;
}
impl WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WIDTH_A> {
        match self.bits {
            0 => Some(WIDTH_A::BYTE),
            1 => Some(WIDTH_A::HALFWORD),
            _ => None,
        }
    }
    #[doc = "TFT Data is 8 bit wide."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == WIDTH_A::BYTE
    }
    #[doc = "TFT Data is 16 bit wide."]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == WIDTH_A::HALFWORD
    }
}
#[doc = "Field `WIDTH` writer - TFT Transaction Width"]
pub type WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WIDTH_A>;
impl<'a, REG> WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TFT Data is 8 bit wide."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTH_A::BYTE)
    }
    #[doc = "TFT Data is 16 bit wide."]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(WIDTH_A::HALFWORD)
    }
}
#[doc = "Field `ALIASBANKEN` reader - Alias to Graphics Bank Enable"]
pub type ALIASBANKEN_R = crate::BitReader;
#[doc = "Field `ALIASBANKEN` writer - Alias to Graphics Bank Enable"]
pub type ALIASBANKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BANKSEL` reader - Graphics Bank"]
pub type BANKSEL_R = crate::FieldReader<BANKSEL_A>;
#[doc = "Graphics Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKSEL_A {
    #[doc = "0: Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK0 = 0,
    #[doc = "1: Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK1 = 1,
    #[doc = "2: Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK2 = 2,
    #[doc = "3: Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    BANK3 = 3,
}
impl From<BANKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BANKSEL_A {
    type Ux = u8;
}
impl BANKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BANKSEL_A {
        match self.bits {
            0 => BANKSEL_A::BANK0,
            1 => BANKSEL_A::BANK1,
            2 => BANKSEL_A::BANK2,
            3 => BANKSEL_A::BANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == BANKSEL_A::BANK0
    }
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == BANKSEL_A::BANK1
    }
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == BANKSEL_A::BANK2
    }
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == BANKSEL_A::BANK3
    }
}
#[doc = "Field `BANKSEL` writer - Graphics Bank"]
pub type BANKSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BANKSEL_A>;
impl<'a, REG> BANKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK0)
    }
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK1)
    }
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK2)
    }
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSEL_A::BANK3)
    }
}
#[doc = "Field `ALIASBANK` reader - Graphic Bank Select Aliasing"]
pub type ALIASBANK_R = crate::FieldReader<ALIASBANK_A>;
#[doc = "Graphic Bank Select Aliasing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALIASBANK_A {
    #[doc = "0: Graphic Bank Select is alias to Bank Select 0"]
    ALIASBANK0 = 0,
    #[doc = "1: Graphic Bank Select is alias to Bank Select 1"]
    ALIASBANK1 = 1,
    #[doc = "2: Graphic Bank Select is alias to Bank Select 2"]
    ALIASBANK2 = 2,
    #[doc = "3: Graphic Bank Select is alias to Bank Select 3"]
    ALIASBANK3 = 3,
}
impl From<ALIASBANK_A> for u8 {
    #[inline(always)]
    fn from(variant: ALIASBANK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALIASBANK_A {
    type Ux = u8;
}
impl ALIASBANK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALIASBANK_A {
        match self.bits {
            0 => ALIASBANK_A::ALIASBANK0,
            1 => ALIASBANK_A::ALIASBANK1,
            2 => ALIASBANK_A::ALIASBANK2,
            3 => ALIASBANK_A::ALIASBANK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    #[inline(always)]
    pub fn is_aliasbank0(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK0
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    #[inline(always)]
    pub fn is_aliasbank1(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK1
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    #[inline(always)]
    pub fn is_aliasbank2(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK2
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    #[inline(always)]
    pub fn is_aliasbank3(&self) -> bool {
        *self == ALIASBANK_A::ALIASBANK3
    }
}
#[doc = "Field `ALIASBANK` writer - Graphic Bank Select Aliasing"]
pub type ALIASBANK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ALIASBANK_A>;
impl<'a, REG> ALIASBANK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    #[inline(always)]
    pub fn aliasbank0(self) -> &'a mut crate::W<REG> {
        self.variant(ALIASBANK_A::ALIASBANK0)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    #[inline(always)]
    pub fn aliasbank1(self) -> &'a mut crate::W<REG> {
        self.variant(ALIASBANK_A::ALIASBANK1)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    #[inline(always)]
    pub fn aliasbank2(self) -> &'a mut crate::W<REG> {
        self.variant(ALIASBANK_A::ALIASBANK2)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    #[inline(always)]
    pub fn aliasbank3(self) -> &'a mut crate::W<REG> {
        self.variant(ALIASBANK_A::ALIASBANK3)
    }
}
impl R {
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline(always)]
    pub fn dd(&self) -> DD_R {
        DD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline(always)]
    pub fn maskblend(&self) -> MASKBLEND_R {
        MASKBLEND_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline(always)]
    pub fn shiftdclken(&self) -> SHIFTDCLKEN_R {
        SHIFTDCLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline(always)]
    pub fn fbctrig(&self) -> FBCTRIG_R {
        FBCTRIG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline(always)]
    pub fn interleave(&self) -> INTERLEAVE_R {
        INTERLEAVE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline(always)]
    pub fn color1src(&self) -> COLOR1SRC_R {
        COLOR1SRC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline(always)]
    pub fn aliasbanken(&self) -> ALIASBANKEN_R {
        ALIASBANKEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline(always)]
    pub fn banksel(&self) -> BANKSEL_R {
        BANKSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline(always)]
    pub fn aliasbank(&self) -> ALIASBANK_R {
        ALIASBANK_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dd(&mut self) -> DD_W<TFTCTRL_SPEC> {
        DD_W::new(self, 0)
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline(always)]
    #[must_use]
    pub fn maskblend(&mut self) -> MASKBLEND_W<TFTCTRL_SPEC> {
        MASKBLEND_W::new(self, 2)
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shiftdclken(&mut self) -> SHIFTDCLKEN_W<TFTCTRL_SPEC> {
        SHIFTDCLKEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn fbctrig(&mut self) -> FBCTRIG_W<TFTCTRL_SPEC> {
        FBCTRIG_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline(always)]
    #[must_use]
    pub fn interleave(&mut self) -> INTERLEAVE_W<TFTCTRL_SPEC> {
        INTERLEAVE_W::new(self, 10)
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline(always)]
    #[must_use]
    pub fn color1src(&mut self) -> COLOR1SRC_W<TFTCTRL_SPEC> {
        COLOR1SRC_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<TFTCTRL_SPEC> {
        WIDTH_W::new(self, 16)
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aliasbanken(&mut self) -> ALIASBANKEN_W<TFTCTRL_SPEC> {
        ALIASBANKEN_W::new(self, 19)
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline(always)]
    #[must_use]
    pub fn banksel(&mut self) -> BANKSEL_W<TFTCTRL_SPEC> {
        BANKSEL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline(always)]
    #[must_use]
    pub fn aliasbank(&mut self) -> ALIASBANK_W<TFTCTRL_SPEC> {
        ALIASBANK_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TFT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tftctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tftctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTCTRL_SPEC;
impl crate::RegisterSpec for TFTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftctrl::R`](R) reader structure"]
impl crate::Readable for TFTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tftctrl::W`](W) writer structure"]
impl crate::Writable for TFTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFTCTRL to value 0"]
impl crate::Resettable for TFTCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
