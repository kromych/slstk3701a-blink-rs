#[doc = "Register `PRSCTRL` reader"]
pub type R = crate::R<PrsctrlSpec>;
#[doc = "Register `PRSCTRL` writer"]
pub type W = crate::W<PrsctrlSpec>;
#[doc = "Field `DECCMPVAL` reader - Decoder State Compare Value"]
pub type DeccmpvalR = crate::FieldReader;
#[doc = "Field `DECCMPVAL` writer - Decoder State Compare Value"]
pub type DeccmpvalW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DECCMPMASK` reader - Decoder State Compare Value Mask"]
pub type DeccmpmaskR = crate::FieldReader;
#[doc = "Field `DECCMPMASK` writer - Decoder State Compare Value Mask"]
pub type DeccmpmaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DECCMPEN` reader - Enable PRS Output DECCMP"]
pub type DeccmpenR = crate::BitReader;
#[doc = "Field `DECCMPEN` writer - Enable PRS Output DECCMP"]
pub type DeccmpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    pub fn deccmpval(&self) -> DeccmpvalR {
        DeccmpvalR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    pub fn deccmpmask(&self) -> DeccmpmaskR {
        DeccmpmaskR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    pub fn deccmpen(&self) -> DeccmpenR {
        DeccmpenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    pub fn deccmpval(&mut self) -> DeccmpvalW<'_, PrsctrlSpec> {
        DeccmpvalW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    pub fn deccmpmask(&mut self) -> DeccmpmaskW<'_, PrsctrlSpec> {
        DeccmpmaskW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    pub fn deccmpen(&mut self) -> DeccmpenW<'_, PrsctrlSpec> {
        DeccmpenW::new(self, 16)
    }
}
#[doc = "PRS Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prsctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prsctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrsctrlSpec;
impl crate::RegisterSpec for PrsctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prsctrl::R`](R) reader structure"]
impl crate::Readable for PrsctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`prsctrl::W`](W) writer structure"]
impl crate::Writable for PrsctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRSCTRL to value 0"]
impl crate::Resettable for PrsctrlSpec {}
