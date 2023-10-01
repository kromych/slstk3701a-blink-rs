#[doc = "Register `RDDATACAPTURE` reader"]
pub type R = crate::R<RDDATACAPTURE_SPEC>;
#[doc = "Register `RDDATACAPTURE` writer"]
pub type W = crate::W<RDDATACAPTURE_SPEC>;
#[doc = "Field `BYPASS` reader - Bypass the Adapted Loopback Clock Circuit"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - Bypass the Adapted Loopback Clock Circuit"]
pub type BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DELAY` reader - Read Delay"]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Read Delay"]
pub type DELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DQSENABLE` reader - DQS Enable Bit"]
pub type DQSENABLE_R = crate::BitReader;
#[doc = "Field `DQSENABLE` writer - DQS Enable Bit"]
pub type DQSENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDRREADDELAY` reader - DDR Read Delay"]
pub type DDRREADDELAY_R = crate::FieldReader;
#[doc = "Field `DDRREADDELAY` writer - DDR Read Delay"]
pub type DDRREADDELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    pub fn dqsenable(&self) -> DQSENABLE_R {
        DQSENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    pub fn ddrreaddelay(&self) -> DDRREADDELAY_R {
        DDRREADDELAY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<RDDATACAPTURE_SPEC, 0> {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<RDDATACAPTURE_SPEC, 1> {
        DELAY_W::new(self)
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dqsenable(&mut self) -> DQSENABLE_W<RDDATACAPTURE_SPEC, 8> {
        DQSENABLE_W::new(self)
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    #[must_use]
    pub fn ddrreaddelay(&mut self) -> DDRREADDELAY_W<RDDATACAPTURE_SPEC, 16> {
        DDRREADDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Read Data Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rddatacapture::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rddatacapture::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDDATACAPTURE_SPEC;
impl crate::RegisterSpec for RDDATACAPTURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rddatacapture::R`](R) reader structure"]
impl crate::Readable for RDDATACAPTURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rddatacapture::W`](W) writer structure"]
impl crate::Writable for RDDATACAPTURE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDDATACAPTURE to value 0x01"]
impl crate::Resettable for RDDATACAPTURE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
