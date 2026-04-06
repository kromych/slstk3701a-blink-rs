#[doc = "Register `DTHRCTL` reader"]
pub type R = crate::R<DthrctlSpec>;
#[doc = "Register `DTHRCTL` writer"]
pub type W = crate::W<DthrctlSpec>;
#[doc = "Field `NONISOTHREN` reader - Non-ISO IN Endpoints Threshold Enable"]
pub type NonisothrenR = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - Non-ISO IN Endpoints Threshold Enable"]
pub type NonisothrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOTHREN` reader - ISO IN Endpoints Threshold Enable"]
pub type IsothrenR = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - ISO IN Endpoints Threshold Enable"]
pub type IsothrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRLEN` reader - Transmit Threshold Length"]
pub type TxthrlenR = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - Transmit Threshold Length"]
pub type TxthrlenW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "AHB Threshold Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ahbthrratio {
    #[doc = "0: AHB threshold = MAC threshold."]
    Div1 = 0,
    #[doc = "1: AHB threshold = MAC threshold / 2."]
    Div2 = 1,
    #[doc = "2: AHB threshold = MAC threshold / 4."]
    Div4 = 2,
    #[doc = "3: AHB threshold = MAC threshold / 8."]
    Div8 = 3,
}
impl From<Ahbthrratio> for u8 {
    #[inline(always)]
    fn from(variant: Ahbthrratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ahbthrratio {
    type Ux = u8;
}
impl crate::IsEnum for Ahbthrratio {}
#[doc = "Field `AHBTHRRATIO` reader - AHB Threshold Ratio"]
pub type AhbthrratioR = crate::FieldReader<Ahbthrratio>;
impl AhbthrratioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahbthrratio {
        match self.bits {
            0 => Ahbthrratio::Div1,
            1 => Ahbthrratio::Div2,
            2 => Ahbthrratio::Div4,
            3 => Ahbthrratio::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "AHB threshold = MAC threshold."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ahbthrratio::Div1
    }
    #[doc = "AHB threshold = MAC threshold / 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ahbthrratio::Div2
    }
    #[doc = "AHB threshold = MAC threshold / 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ahbthrratio::Div4
    }
    #[doc = "AHB threshold = MAC threshold / 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ahbthrratio::Div8
    }
}
#[doc = "Field `AHBTHRRATIO` writer - AHB Threshold Ratio"]
pub type AhbthrratioW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ahbthrratio, crate::Safe>;
impl<'a, REG> AhbthrratioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AHB threshold = MAC threshold."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbthrratio::Div1)
    }
    #[doc = "AHB threshold = MAC threshold / 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbthrratio::Div2)
    }
    #[doc = "AHB threshold = MAC threshold / 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbthrratio::Div4)
    }
    #[doc = "AHB threshold = MAC threshold / 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbthrratio::Div8)
    }
}
#[doc = "Field `RXTHREN` reader - Receive Threshold Enable"]
pub type RxthrenR = crate::BitReader;
#[doc = "Field `RXTHREN` writer - Receive Threshold Enable"]
pub type RxthrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRLEN` reader - Receive Threshold Length"]
pub type RxthrlenR = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - Receive Threshold Length"]
pub type RxthrlenW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ARBPRKEN` reader - Arbiter Parking Enable"]
pub type ArbprkenR = crate::BitReader;
#[doc = "Field `ARBPRKEN` writer - Arbiter Parking Enable"]
pub type ArbprkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NonisothrenR {
        NonisothrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&self) -> IsothrenR {
        IsothrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TxthrlenR {
        TxthrlenR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AhbthrratioR {
        AhbthrratioR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&self) -> RxthrenR {
        RxthrenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RxthrlenR {
        RxthrlenR::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&self) -> ArbprkenR {
        ArbprkenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NonisothrenW<'_, DthrctlSpec> {
        NonisothrenW::new(self, 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&mut self) -> IsothrenW<'_, DthrctlSpec> {
        IsothrenW::new(self, 1)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TxthrlenW<'_, DthrctlSpec> {
        TxthrlenW::new(self, 2)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&mut self) -> AhbthrratioW<'_, DthrctlSpec> {
        AhbthrratioW::new(self, 11)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&mut self) -> RxthrenW<'_, DthrctlSpec> {
        RxthrenW::new(self, 16)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RxthrlenW<'_, DthrctlSpec> {
        RxthrlenW::new(self, 17)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&mut self) -> ArbprkenW<'_, DthrctlSpec> {
        ArbprkenW::new(self, 27)
    }
}
#[doc = "Device Threshold Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DthrctlSpec;
impl crate::RegisterSpec for DthrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dthrctl::R`](R) reader structure"]
impl crate::Readable for DthrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure"]
impl crate::Writable for DthrctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTHRCTL to value 0x0810_0020"]
impl crate::Resettable for DthrctlSpec {
    const RESET_VALUE: u32 = 0x0810_0020;
}
