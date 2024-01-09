#[doc = "Register `DEVDELAY` reader"]
pub type R = crate::R<DEVDELAY_SPEC>;
#[doc = "Register `DEVDELAY` writer"]
pub type W = crate::W<DEVDELAY_SPEC>;
#[doc = "Field `DINIT` reader - Clock Delay for CS"]
pub type DINIT_R = crate::FieldReader;
#[doc = "Field `DINIT` writer - Clock Delay for CS"]
pub type DINIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DAFTER` reader - Clock Delay for Last Transaction Bit"]
pub type DAFTER_R = crate::FieldReader;
#[doc = "Field `DAFTER` writer - Clock Delay for Last Transaction Bit"]
pub type DAFTER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBTWN` reader - Clock Delay Between Two Chip Selects"]
pub type DBTWN_R = crate::FieldReader;
#[doc = "Field `DBTWN` writer - Clock Delay Between Two Chip Selects"]
pub type DBTWN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DNSS` reader - Clock Delay for Chip Select Deassert"]
pub type DNSS_R = crate::FieldReader;
#[doc = "Field `DNSS` writer - Clock Delay for Chip Select Deassert"]
pub type DNSS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    pub fn dinit(&self) -> DINIT_R {
        DINIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    pub fn dafter(&self) -> DAFTER_R {
        DAFTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    pub fn dbtwn(&self) -> DBTWN_R {
        DBTWN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    pub fn dnss(&self) -> DNSS_R {
        DNSS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    #[must_use]
    pub fn dinit(&mut self) -> DINIT_W<DEVDELAY_SPEC> {
        DINIT_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dafter(&mut self) -> DAFTER_W<DEVDELAY_SPEC> {
        DAFTER_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    #[must_use]
    pub fn dbtwn(&mut self) -> DBTWN_W<DEVDELAY_SPEC> {
        DBTWN_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    #[must_use]
    pub fn dnss(&mut self) -> DNSS_W<DEVDELAY_SPEC> {
        DNSS_W::new(self, 24)
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
#[doc = "Device Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devdelay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devdelay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVDELAY_SPEC;
impl crate::RegisterSpec for DEVDELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdelay::R`](R) reader structure"]
impl crate::Readable for DEVDELAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devdelay::W`](W) writer structure"]
impl crate::Writable for DEVDELAY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVDELAY to value 0"]
impl crate::Resettable for DEVDELAY_SPEC {
    const RESET_VALUE: u32 = 0;
}
