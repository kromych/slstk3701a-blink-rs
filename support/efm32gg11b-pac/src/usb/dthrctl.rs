#[doc = "Register `DTHRCTL` reader"]
pub type R = crate::R<DTHRCTL_SPEC>;
#[doc = "Register `DTHRCTL` writer"]
pub type W = crate::W<DTHRCTL_SPEC>;
#[doc = "Field `NONISOTHREN` reader - Non-ISO IN Endpoints Threshold Enable"]
pub type NONISOTHREN_R = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - Non-ISO IN Endpoints Threshold Enable"]
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOTHREN` reader - ISO IN Endpoints Threshold Enable"]
pub type ISOTHREN_R = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - ISO IN Endpoints Threshold Enable"]
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRLEN` reader - Transmit Threshold Length"]
pub type TXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - Transmit Threshold Length"]
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `AHBTHRRATIO` reader - AHB Threshold Ratio"]
pub type AHBTHRRATIO_R = crate::FieldReader<AHBTHRRATIO_A>;
#[doc = "AHB Threshold Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHBTHRRATIO_A {
    #[doc = "0: AHB threshold = MAC threshold."]
    DIV1 = 0,
    #[doc = "1: AHB threshold = MAC threshold / 2."]
    DIV2 = 1,
    #[doc = "2: AHB threshold = MAC threshold / 4."]
    DIV4 = 2,
    #[doc = "3: AHB threshold = MAC threshold / 8."]
    DIV8 = 3,
}
impl From<AHBTHRRATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBTHRRATIO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AHBTHRRATIO_A {
    type Ux = u8;
}
impl AHBTHRRATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHBTHRRATIO_A {
        match self.bits {
            0 => AHBTHRRATIO_A::DIV1,
            1 => AHBTHRRATIO_A::DIV2,
            2 => AHBTHRRATIO_A::DIV4,
            3 => AHBTHRRATIO_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "AHB threshold = MAC threshold."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV1
    }
    #[doc = "AHB threshold = MAC threshold / 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV2
    }
    #[doc = "AHB threshold = MAC threshold / 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV4
    }
    #[doc = "AHB threshold = MAC threshold / 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AHBTHRRATIO_A::DIV8
    }
}
#[doc = "Field `AHBTHRRATIO` writer - AHB Threshold Ratio"]
pub type AHBTHRRATIO_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AHBTHRRATIO_A>;
impl<'a, REG> AHBTHRRATIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AHB threshold = MAC threshold."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(AHBTHRRATIO_A::DIV1)
    }
    #[doc = "AHB threshold = MAC threshold / 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(AHBTHRRATIO_A::DIV2)
    }
    #[doc = "AHB threshold = MAC threshold / 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(AHBTHRRATIO_A::DIV4)
    }
    #[doc = "AHB threshold = MAC threshold / 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(AHBTHRRATIO_A::DIV8)
    }
}
#[doc = "Field `RXTHREN` reader - Receive Threshold Enable"]
pub type RXTHREN_R = crate::BitReader;
#[doc = "Field `RXTHREN` writer - Receive Threshold Enable"]
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRLEN` reader - Receive Threshold Length"]
pub type RXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - Receive Threshold Length"]
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ARBPRKEN` reader - Arbiter Parking Enable"]
pub type ARBPRKEN_R = crate::BitReader;
#[doc = "Field `ARBPRKEN` writer - Arbiter Parking Enable"]
pub type ARBPRKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AHBTHRRATIO_R {
        AHBTHRRATIO_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    pub fn arbprken(&self) -> ARBPRKEN_R {
        ARBPRKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<DTHRCTL_SPEC> {
        NONISOTHREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ISO IN Endpoints Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isothren(&mut self) -> ISOTHREN_W<DTHRCTL_SPEC> {
        ISOTHREN_W::new(self, 1)
    }
    #[doc = "Bits 2:10 - Transmit Threshold Length"]
    #[inline(always)]
    #[must_use]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<DTHRCTL_SPEC> {
        TXTHRLEN_W::new(self, 2)
    }
    #[doc = "Bits 11:12 - AHB Threshold Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ahbthrratio(&mut self) -> AHBTHRRATIO_W<DTHRCTL_SPEC> {
        AHBTHRRATIO_W::new(self, 11)
    }
    #[doc = "Bit 16 - Receive Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxthren(&mut self) -> RXTHREN_W<DTHRCTL_SPEC> {
        RXTHREN_W::new(self, 16)
    }
    #[doc = "Bits 17:25 - Receive Threshold Length"]
    #[inline(always)]
    #[must_use]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<DTHRCTL_SPEC> {
        RXTHRLEN_W::new(self, 17)
    }
    #[doc = "Bit 27 - Arbiter Parking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arbprken(&mut self) -> ARBPRKEN_W<DTHRCTL_SPEC> {
        ARBPRKEN_W::new(self, 27)
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
#[doc = "Device Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dthrctl::R`](R) reader structure"]
impl crate::Readable for DTHRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure"]
impl crate::Writable for DTHRCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTHRCTL to value 0x0810_0020"]
impl crate::Resettable for DTHRCTL_SPEC {
    const RESET_VALUE: u32 = 0x0810_0020;
}
