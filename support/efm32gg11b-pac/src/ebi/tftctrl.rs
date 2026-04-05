#[doc = "Register `TFTCTRL` reader"]
pub type R = crate::R<TftctrlSpec>;
#[doc = "Register `TFTCTRL` writer"]
pub type W = crate::W<TftctrlSpec>;
#[doc = "TFT Direct Drive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dd {
    #[doc = "0: Direct Drive is disabled."]
    Disabled = 0,
    #[doc = "1: Direct Drive from internal memory enabled and started."]
    Internal = 1,
    #[doc = "2: Direct Drive from external memory enabled and started."]
    External = 2,
}
impl From<Dd> for u8 {
    #[inline(always)]
    fn from(variant: Dd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dd {
    type Ux = u8;
}
impl crate::IsEnum for Dd {}
#[doc = "Field `DD` reader - TFT Direct Drive Mode"]
pub type DdR = crate::FieldReader<Dd>;
impl DdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dd> {
        match self.bits {
            0 => Some(Dd::Disabled),
            1 => Some(Dd::Internal),
            2 => Some(Dd::External),
            _ => None,
        }
    }
    #[doc = "Direct Drive is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dd::Disabled
    }
    #[doc = "Direct Drive from internal memory enabled and started."]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Dd::Internal
    }
    #[doc = "Direct Drive from external memory enabled and started."]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Dd::External
    }
}
#[doc = "Field `DD` writer - TFT Direct Drive Mode"]
pub type DdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dd>;
impl<'a, REG> DdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct Drive is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dd::Disabled)
    }
    #[doc = "Direct Drive from internal memory enabled and started."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Dd::Internal)
    }
    #[doc = "Direct Drive from external memory enabled and started."]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Dd::External)
    }
}
#[doc = "TFT Mask and Blend Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maskblend {
    #[doc = "0: Masking and Blending are disabled."]
    Disabled = 0,
    #[doc = "1: Internal Masking is enabled."]
    Imask = 1,
    #[doc = "2: Internal Alpha Blending is enabled."]
    Ialpha = 2,
    #[doc = "3: Internal Masking and Alpha Blending are enabled."]
    Imaskalpha = 3,
    #[doc = "4: External Frame Buffer Masking is enabled."]
    Efbmask = 4,
    #[doc = "5: External Frame Buffer Alpha Blending is enabled."]
    Efbalpha = 5,
    #[doc = "6: External Frame Buffer Masking and Alpha Blending are enabled."]
    Efbmaskalpha = 6,
    #[doc = "7: Internal Frame Buffer Masking is enabled."]
    Ifbmask = 7,
    #[doc = "8: Internal Frame Buffer Alpha Blending is enabled."]
    Ifbalpha = 8,
    #[doc = "9: Internal Frame Buffer Masking and Alpha Blending are enabled."]
    Ifbmaskalpha = 9,
}
impl From<Maskblend> for u8 {
    #[inline(always)]
    fn from(variant: Maskblend) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maskblend {
    type Ux = u8;
}
impl crate::IsEnum for Maskblend {}
#[doc = "Field `MASKBLEND` reader - TFT Mask and Blend Mode"]
pub type MaskblendR = crate::FieldReader<Maskblend>;
impl MaskblendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maskblend> {
        match self.bits {
            0 => Some(Maskblend::Disabled),
            1 => Some(Maskblend::Imask),
            2 => Some(Maskblend::Ialpha),
            3 => Some(Maskblend::Imaskalpha),
            4 => Some(Maskblend::Efbmask),
            5 => Some(Maskblend::Efbalpha),
            6 => Some(Maskblend::Efbmaskalpha),
            7 => Some(Maskblend::Ifbmask),
            8 => Some(Maskblend::Ifbalpha),
            9 => Some(Maskblend::Ifbmaskalpha),
            _ => None,
        }
    }
    #[doc = "Masking and Blending are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Maskblend::Disabled
    }
    #[doc = "Internal Masking is enabled."]
    #[inline(always)]
    pub fn is_imask(&self) -> bool {
        *self == Maskblend::Imask
    }
    #[doc = "Internal Alpha Blending is enabled."]
    #[inline(always)]
    pub fn is_ialpha(&self) -> bool {
        *self == Maskblend::Ialpha
    }
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn is_imaskalpha(&self) -> bool {
        *self == Maskblend::Imaskalpha
    }
    #[doc = "External Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn is_efbmask(&self) -> bool {
        *self == Maskblend::Efbmask
    }
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn is_efbalpha(&self) -> bool {
        *self == Maskblend::Efbalpha
    }
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn is_efbmaskalpha(&self) -> bool {
        *self == Maskblend::Efbmaskalpha
    }
    #[doc = "Internal Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn is_ifbmask(&self) -> bool {
        *self == Maskblend::Ifbmask
    }
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn is_ifbalpha(&self) -> bool {
        *self == Maskblend::Ifbalpha
    }
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn is_ifbmaskalpha(&self) -> bool {
        *self == Maskblend::Ifbmaskalpha
    }
}
#[doc = "Field `MASKBLEND` writer - TFT Mask and Blend Mode"]
pub type MaskblendW<'a, REG> = crate::FieldWriter<'a, REG, 4, Maskblend>;
impl<'a, REG> MaskblendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masking and Blending are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Disabled)
    }
    #[doc = "Internal Masking is enabled."]
    #[inline(always)]
    pub fn imask(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Imask)
    }
    #[doc = "Internal Alpha Blending is enabled."]
    #[inline(always)]
    pub fn ialpha(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Ialpha)
    }
    #[doc = "Internal Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn imaskalpha(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Imaskalpha)
    }
    #[doc = "External Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn efbmask(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Efbmask)
    }
    #[doc = "External Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn efbalpha(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Efbalpha)
    }
    #[doc = "External Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn efbmaskalpha(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Efbmaskalpha)
    }
    #[doc = "Internal Frame Buffer Masking is enabled."]
    #[inline(always)]
    pub fn ifbmask(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Ifbmask)
    }
    #[doc = "Internal Frame Buffer Alpha Blending is enabled."]
    #[inline(always)]
    pub fn ifbalpha(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Ifbalpha)
    }
    #[doc = "Internal Frame Buffer Masking and Alpha Blending are enabled."]
    #[inline(always)]
    pub fn ifbmaskalpha(self) -> &'a mut crate::W<REG> {
        self.variant(Maskblend::Ifbmaskalpha)
    }
}
#[doc = "Field `SHIFTDCLKEN` reader - TFT EBI_DCLK Shift Enable"]
pub type ShiftdclkenR = crate::BitReader;
#[doc = "Field `SHIFTDCLKEN` writer - TFT EBI_DCLK Shift Enable"]
pub type ShiftdclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBCTRIG` reader - TFT Frame Base Copy Trigger"]
pub type FbctrigR = crate::BitReader;
#[doc = "Field `FBCTRIG` writer - TFT Frame Base Copy Trigger"]
pub type FbctrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interleave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Interleave {
    #[doc = "0: Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    Unlimited = 0,
    #[doc = "1: Allow 1 interleaved EBI access per EBI_DCLK period."]
    Oneperdclk = 1,
    #[doc = "2: Only allow EBI accesses during TFT porches."]
    Porch = 2,
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(variant: Interleave) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Interleave {
    type Ux = u8;
}
impl crate::IsEnum for Interleave {}
#[doc = "Field `INTERLEAVE` reader - Interleave Mode"]
pub type InterleaveR = crate::FieldReader<Interleave>;
impl InterleaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Interleave> {
        match self.bits {
            0 => Some(Interleave::Unlimited),
            1 => Some(Interleave::Oneperdclk),
            2 => Some(Interleave::Porch),
            _ => None,
        }
    }
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    #[inline(always)]
    pub fn is_unlimited(&self) -> bool {
        *self == Interleave::Unlimited
    }
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    #[inline(always)]
    pub fn is_oneperdclk(&self) -> bool {
        *self == Interleave::Oneperdclk
    }
    #[doc = "Only allow EBI accesses during TFT porches."]
    #[inline(always)]
    pub fn is_porch(&self) -> bool {
        *self == Interleave::Porch
    }
}
#[doc = "Field `INTERLEAVE` writer - Interleave Mode"]
pub type InterleaveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Interleave>;
impl<'a, REG> InterleaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Allow unlimited interleaved EBI accesses per EBI_DCLK period. This can cause jitter on the EBI_DCLK"]
    #[inline(always)]
    pub fn unlimited(self) -> &'a mut crate::W<REG> {
        self.variant(Interleave::Unlimited)
    }
    #[doc = "Allow 1 interleaved EBI access per EBI_DCLK period."]
    #[inline(always)]
    pub fn oneperdclk(self) -> &'a mut crate::W<REG> {
        self.variant(Interleave::Oneperdclk)
    }
    #[doc = "Only allow EBI accesses during TFT porches."]
    #[inline(always)]
    pub fn porch(self) -> &'a mut crate::W<REG> {
        self.variant(Interleave::Porch)
    }
}
#[doc = "Field `COLOR1SRC` reader - Masking/Alpha Blending Color1 Source"]
pub type Color1srcR = crate::BitReader;
#[doc = "Field `COLOR1SRC` writer - Masking/Alpha Blending Color1 Source"]
pub type Color1srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TFT Transaction Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Width {
    #[doc = "0: TFT Data is 8 bit wide."]
    Byte = 0,
    #[doc = "1: TFT Data is 16 bit wide."]
    Halfword = 1,
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(variant: Width) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Width {
    type Ux = u8;
}
impl crate::IsEnum for Width {}
#[doc = "Field `WIDTH` reader - TFT Transaction Width"]
pub type WidthR = crate::FieldReader<Width>;
impl WidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Width> {
        match self.bits {
            0 => Some(Width::Byte),
            1 => Some(Width::Halfword),
            _ => None,
        }
    }
    #[doc = "TFT Data is 8 bit wide."]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Width::Byte
    }
    #[doc = "TFT Data is 16 bit wide."]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == Width::Halfword
    }
}
#[doc = "Field `WIDTH` writer - TFT Transaction Width"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Width>;
impl<'a, REG> WidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TFT Data is 8 bit wide."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Width::Byte)
    }
    #[doc = "TFT Data is 16 bit wide."]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(Width::Halfword)
    }
}
#[doc = "Field `ALIASBANKEN` reader - Alias to Graphics Bank Enable"]
pub type AliasbankenR = crate::BitReader;
#[doc = "Field `ALIASBANKEN` writer - Alias to Graphics Bank Enable"]
pub type AliasbankenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Graphics Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Banksel {
    #[doc = "0: Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank0 = 0,
    #[doc = "1: Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank1 = 1,
    #[doc = "2: Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank2 = 2,
    #[doc = "3: Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    Bank3 = 3,
}
impl From<Banksel> for u8 {
    #[inline(always)]
    fn from(variant: Banksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Banksel {
    type Ux = u8;
}
impl crate::IsEnum for Banksel {}
#[doc = "Field `BANKSEL` reader - Graphics Bank"]
pub type BankselR = crate::FieldReader<Banksel>;
impl BankselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Banksel {
        match self.bits {
            0 => Banksel::Bank0,
            1 => Banksel::Bank1,
            2 => Banksel::Bank2,
            3 => Banksel::Bank3,
            _ => unreachable!(),
        }
    }
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == Banksel::Bank0
    }
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == Banksel::Bank1
    }
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Banksel::Bank2
    }
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn is_bank3(&self) -> bool {
        *self == Banksel::Bank3
    }
}
#[doc = "Field `BANKSEL` writer - Graphics Bank"]
pub type BankselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Banksel, crate::Safe>;
impl<'a, REG> BankselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Memory bank 0 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank0(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank0)
    }
    #[doc = "Memory bank 1 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank1)
    }
    #[doc = "Memory bank 2 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank2)
    }
    #[doc = "Memory bank 3 is used for Direct Drive, Masking, and Alpha Blending."]
    #[inline(always)]
    pub fn bank3(self) -> &'a mut crate::W<REG> {
        self.variant(Banksel::Bank3)
    }
}
#[doc = "Graphic Bank Select Aliasing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aliasbank {
    #[doc = "0: Graphic Bank Select is alias to Bank Select 0"]
    Aliasbank0 = 0,
    #[doc = "1: Graphic Bank Select is alias to Bank Select 1"]
    Aliasbank1 = 1,
    #[doc = "2: Graphic Bank Select is alias to Bank Select 2"]
    Aliasbank2 = 2,
    #[doc = "3: Graphic Bank Select is alias to Bank Select 3"]
    Aliasbank3 = 3,
}
impl From<Aliasbank> for u8 {
    #[inline(always)]
    fn from(variant: Aliasbank) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aliasbank {
    type Ux = u8;
}
impl crate::IsEnum for Aliasbank {}
#[doc = "Field `ALIASBANK` reader - Graphic Bank Select Aliasing"]
pub type AliasbankR = crate::FieldReader<Aliasbank>;
impl AliasbankR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aliasbank {
        match self.bits {
            0 => Aliasbank::Aliasbank0,
            1 => Aliasbank::Aliasbank1,
            2 => Aliasbank::Aliasbank2,
            3 => Aliasbank::Aliasbank3,
            _ => unreachable!(),
        }
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    #[inline(always)]
    pub fn is_aliasbank0(&self) -> bool {
        *self == Aliasbank::Aliasbank0
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    #[inline(always)]
    pub fn is_aliasbank1(&self) -> bool {
        *self == Aliasbank::Aliasbank1
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    #[inline(always)]
    pub fn is_aliasbank2(&self) -> bool {
        *self == Aliasbank::Aliasbank2
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    #[inline(always)]
    pub fn is_aliasbank3(&self) -> bool {
        *self == Aliasbank::Aliasbank3
    }
}
#[doc = "Field `ALIASBANK` writer - Graphic Bank Select Aliasing"]
pub type AliasbankW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aliasbank, crate::Safe>;
impl<'a, REG> AliasbankW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Graphic Bank Select is alias to Bank Select 0"]
    #[inline(always)]
    pub fn aliasbank0(self) -> &'a mut crate::W<REG> {
        self.variant(Aliasbank::Aliasbank0)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 1"]
    #[inline(always)]
    pub fn aliasbank1(self) -> &'a mut crate::W<REG> {
        self.variant(Aliasbank::Aliasbank1)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 2"]
    #[inline(always)]
    pub fn aliasbank2(self) -> &'a mut crate::W<REG> {
        self.variant(Aliasbank::Aliasbank2)
    }
    #[doc = "Graphic Bank Select is alias to Bank Select 3"]
    #[inline(always)]
    pub fn aliasbank3(self) -> &'a mut crate::W<REG> {
        self.variant(Aliasbank::Aliasbank3)
    }
}
impl R {
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline(always)]
    pub fn dd(&self) -> DdR {
        DdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline(always)]
    pub fn maskblend(&self) -> MaskblendR {
        MaskblendR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline(always)]
    pub fn shiftdclken(&self) -> ShiftdclkenR {
        ShiftdclkenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline(always)]
    pub fn fbctrig(&self) -> FbctrigR {
        FbctrigR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline(always)]
    pub fn interleave(&self) -> InterleaveR {
        InterleaveR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline(always)]
    pub fn color1src(&self) -> Color1srcR {
        Color1srcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline(always)]
    pub fn aliasbanken(&self) -> AliasbankenR {
        AliasbankenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline(always)]
    pub fn banksel(&self) -> BankselR {
        BankselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline(always)]
    pub fn aliasbank(&self) -> AliasbankR {
        AliasbankR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TFT Direct Drive Mode"]
    #[inline(always)]
    pub fn dd(&mut self) -> DdW<'_, TftctrlSpec> {
        DdW::new(self, 0)
    }
    #[doc = "Bits 2:5 - TFT Mask and Blend Mode"]
    #[inline(always)]
    pub fn maskblend(&mut self) -> MaskblendW<'_, TftctrlSpec> {
        MaskblendW::new(self, 2)
    }
    #[doc = "Bit 8 - TFT EBI_DCLK Shift Enable"]
    #[inline(always)]
    pub fn shiftdclken(&mut self) -> ShiftdclkenW<'_, TftctrlSpec> {
        ShiftdclkenW::new(self, 8)
    }
    #[doc = "Bit 9 - TFT Frame Base Copy Trigger"]
    #[inline(always)]
    pub fn fbctrig(&mut self) -> FbctrigW<'_, TftctrlSpec> {
        FbctrigW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Interleave Mode"]
    #[inline(always)]
    pub fn interleave(&mut self) -> InterleaveW<'_, TftctrlSpec> {
        InterleaveW::new(self, 10)
    }
    #[doc = "Bit 12 - Masking/Alpha Blending Color1 Source"]
    #[inline(always)]
    pub fn color1src(&mut self) -> Color1srcW<'_, TftctrlSpec> {
        Color1srcW::new(self, 12)
    }
    #[doc = "Bits 16:17 - TFT Transaction Width"]
    #[inline(always)]
    pub fn width(&mut self) -> WidthW<'_, TftctrlSpec> {
        WidthW::new(self, 16)
    }
    #[doc = "Bit 19 - Alias to Graphics Bank Enable"]
    #[inline(always)]
    pub fn aliasbanken(&mut self) -> AliasbankenW<'_, TftctrlSpec> {
        AliasbankenW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Graphics Bank"]
    #[inline(always)]
    pub fn banksel(&mut self) -> BankselW<'_, TftctrlSpec> {
        BankselW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Graphic Bank Select Aliasing"]
    #[inline(always)]
    pub fn aliasbank(&mut self) -> AliasbankW<'_, TftctrlSpec> {
        AliasbankW::new(self, 22)
    }
}
#[doc = "TFT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftctrlSpec;
impl crate::RegisterSpec for TftctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftctrl::R`](R) reader structure"]
impl crate::Readable for TftctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tftctrl::W`](W) writer structure"]
impl crate::Writable for TftctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTCTRL to value 0"]
impl crate::Resettable for TftctrlSpec {}
