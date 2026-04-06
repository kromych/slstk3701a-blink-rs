#[doc = "Register `RDDATACAPTURE` reader"]
pub type R = crate::R<RddatacaptureSpec>;
#[doc = "Register `RDDATACAPTURE` writer"]
pub type W = crate::W<RddatacaptureSpec>;
#[doc = "Field `BYPASS` reader - Bypass the Adapted Loopback Clock Circuit"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - Bypass the Adapted Loopback Clock Circuit"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY` reader - Read Delay"]
pub type DelayR = crate::FieldReader;
#[doc = "Field `DELAY` writer - Read Delay"]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQSENABLE` reader - DQS Enable Bit"]
pub type DqsenableR = crate::BitReader;
#[doc = "Field `DQSENABLE` writer - DQS Enable Bit"]
pub type DqsenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRREADDELAY` reader - DDR Read Delay"]
pub type DdrreaddelayR = crate::FieldReader;
#[doc = "Field `DDRREADDELAY` writer - DDR Read Delay"]
pub type DdrreaddelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    pub fn dqsenable(&self) -> DqsenableR {
        DqsenableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    pub fn ddrreaddelay(&self) -> DdrreaddelayR {
        DdrreaddelayR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, RddatacaptureSpec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<'_, RddatacaptureSpec> {
        DelayW::new(self, 1)
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    pub fn dqsenable(&mut self) -> DqsenableW<'_, RddatacaptureSpec> {
        DqsenableW::new(self, 8)
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    pub fn ddrreaddelay(&mut self) -> DdrreaddelayW<'_, RddatacaptureSpec> {
        DdrreaddelayW::new(self, 16)
    }
}
#[doc = "Read Data Capture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rddatacapture::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rddatacapture::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RddatacaptureSpec;
impl crate::RegisterSpec for RddatacaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rddatacapture::R`](R) reader structure"]
impl crate::Readable for RddatacaptureSpec {}
#[doc = "`write(|w| ..)` method takes [`rddatacapture::W`](W) writer structure"]
impl crate::Writable for RddatacaptureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDDATACAPTURE to value 0x01"]
impl crate::Resettable for RddatacaptureSpec {
    const RESET_VALUE: u32 = 0x01;
}
