#[doc = "Register `HFXOSTARTUPCTRL` reader"]
pub type R = crate::R<HfxostartupctrlSpec>;
#[doc = "Register `HFXOSTARTUPCTRL` writer"]
pub type W = crate::W<HfxostartupctrlSpec>;
#[doc = "Field `IBTRIMXOCORE` reader - Sets the Startup Oscillator Core Bias Current"]
pub type IbtrimxocoreR = crate::FieldReader<u16>;
#[doc = "Field `IBTRIMXOCORE` writer - Sets the Startup Oscillator Core Bias Current"]
pub type IbtrimxocoreW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CTUNE` reader - Sets Oscillator Tuning Capacitance"]
pub type CtuneR = crate::FieldReader<u16>;
#[doc = "Field `CTUNE` writer - Sets Oscillator Tuning Capacitance"]
pub type CtuneW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:10 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IbtrimxocoreR {
        IbtrimxocoreR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CtuneR {
        CtuneR::new(((self.bits >> 11) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    pub fn ibtrimxocore(&mut self) -> IbtrimxocoreW<'_, HfxostartupctrlSpec> {
        IbtrimxocoreW::new(self, 0)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&mut self) -> CtuneW<'_, HfxostartupctrlSpec> {
        CtuneW::new(self, 11)
    }
}
#[doc = "HFXO Startup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxostartupctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxostartupctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxostartupctrlSpec;
impl crate::RegisterSpec for HfxostartupctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxostartupctrl::R`](R) reader structure"]
impl crate::Readable for HfxostartupctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxostartupctrl::W`](W) writer structure"]
impl crate::Writable for HfxostartupctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFXOSTARTUPCTRL to value 0x0600"]
impl crate::Resettable for HfxostartupctrlSpec {
    const RESET_VALUE: u32 = 0x0600;
}
