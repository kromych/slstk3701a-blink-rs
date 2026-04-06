#[doc = "Register `EXTIFCTRL` reader"]
pub type R = crate::R<ExtifctrlSpec>;
#[doc = "Register `EXTIFCTRL` writer"]
pub type W = crate::W<ExtifctrlSpec>;
#[doc = "Field `EN` reader - Enable External Interface"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable External Interface"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "APORT Selection for External Interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aportsel {
    #[doc = "0: APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    Aport0x = 0,
    #[doc = "1: APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    Aport0y = 1,
    #[doc = "2: APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    Aport1x = 2,
    #[doc = "3: APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    Aport1y = 3,
    #[doc = "4: APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    Aport1xy = 4,
    #[doc = "5: APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    Aport2x = 5,
    #[doc = "6: APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    Aport2y = 6,
    #[doc = "7: APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    Aport2yx = 7,
    #[doc = "8: APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    Aport3x = 8,
    #[doc = "9: APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    Aport3y = 9,
    #[doc = "10: APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    Aport3xy = 10,
    #[doc = "11: APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    Aport4x = 11,
    #[doc = "12: APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    Aport4y = 12,
    #[doc = "13: APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    Aport4yx = 13,
}
impl From<Aportsel> for u8 {
    #[inline(always)]
    fn from(variant: Aportsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aportsel {
    type Ux = u8;
}
impl crate::IsEnum for Aportsel {}
#[doc = "Field `APORTSEL` reader - APORT Selection for External Interface"]
pub type AportselR = crate::FieldReader<Aportsel>;
impl AportselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aportsel> {
        match self.bits {
            0 => Some(Aportsel::Aport0x),
            1 => Some(Aportsel::Aport0y),
            2 => Some(Aportsel::Aport1x),
            3 => Some(Aportsel::Aport1y),
            4 => Some(Aportsel::Aport1xy),
            5 => Some(Aportsel::Aport2x),
            6 => Some(Aportsel::Aport2y),
            7 => Some(Aportsel::Aport2yx),
            8 => Some(Aportsel::Aport3x),
            9 => Some(Aportsel::Aport3y),
            10 => Some(Aportsel::Aport3xy),
            11 => Some(Aportsel::Aport4x),
            12 => Some(Aportsel::Aport4y),
            13 => Some(Aportsel::Aport4yx),
            _ => None,
        }
    }
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    #[inline(always)]
    pub fn is_aport0x(&self) -> bool {
        *self == Aportsel::Aport0x
    }
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    #[inline(always)]
    pub fn is_aport0y(&self) -> bool {
        *self == Aportsel::Aport0y
    }
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn is_aport1x(&self) -> bool {
        *self == Aportsel::Aport1x
    }
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn is_aport1y(&self) -> bool {
        *self == Aportsel::Aport1y
    }
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn is_aport1xy(&self) -> bool {
        *self == Aportsel::Aport1xy
    }
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn is_aport2x(&self) -> bool {
        *self == Aportsel::Aport2x
    }
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn is_aport2y(&self) -> bool {
        *self == Aportsel::Aport2y
    }
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn is_aport2yx(&self) -> bool {
        *self == Aportsel::Aport2yx
    }
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn is_aport3x(&self) -> bool {
        *self == Aportsel::Aport3x
    }
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn is_aport3y(&self) -> bool {
        *self == Aportsel::Aport3y
    }
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn is_aport3xy(&self) -> bool {
        *self == Aportsel::Aport3xy
    }
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn is_aport4x(&self) -> bool {
        *self == Aportsel::Aport4x
    }
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn is_aport4y(&self) -> bool {
        *self == Aportsel::Aport4y
    }
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn is_aport4yx(&self) -> bool {
        *self == Aportsel::Aport4yx
    }
}
#[doc = "Field `APORTSEL` writer - APORT Selection for External Interface"]
pub type AportselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Aportsel>;
impl<'a, REG> AportselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    #[inline(always)]
    pub fn aport0x(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport0x)
    }
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    #[inline(always)]
    pub fn aport0y(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport0y)
    }
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1x(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport1x)
    }
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1y(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport1y)
    }
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1xy(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport1xy)
    }
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2x(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport2x)
    }
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2y(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport2y)
    }
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2yx(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport2yx)
    }
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3x(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport3x)
    }
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3y(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport3y)
    }
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3xy(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport3xy)
    }
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4x(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport4x)
    }
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4y(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport4y)
    }
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4yx(self) -> &'a mut crate::W<REG> {
        self.variant(Aportsel::Aport4yx)
    }
}
impl R {
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline(always)]
    pub fn aportsel(&self) -> AportselR {
        AportselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, ExtifctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline(always)]
    pub fn aportsel(&mut self) -> AportselW<'_, ExtifctrlSpec> {
        AportselW::new(self, 4)
    }
}
#[doc = "External Override Interface Control\n\nYou can [`read`](crate::Reg::read) this register and get [`extifctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extifctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtifctrlSpec;
impl crate::RegisterSpec for ExtifctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extifctrl::R`](R) reader structure"]
impl crate::Readable for ExtifctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`extifctrl::W`](W) writer structure"]
impl crate::Writable for ExtifctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIFCTRL to value 0"]
impl crate::Resettable for ExtifctrlSpec {}
