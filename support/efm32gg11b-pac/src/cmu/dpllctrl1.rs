#[doc = "Register `DPLLCTRL1` reader"]
pub type R = crate::R<Dpllctrl1Spec>;
#[doc = "Register `DPLLCTRL1` writer"]
pub type W = crate::W<Dpllctrl1Spec>;
#[doc = "Field `M` reader - Factor M"]
pub type MR = crate::FieldReader<u16>;
#[doc = "Field `M` writer - Factor M"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `N` reader - Factor N"]
pub type NR = crate::FieldReader<u16>;
#[doc = "Field `N` writer - Factor N"]
pub type NW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, Dpllctrl1Spec> {
        MW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&mut self) -> NW<'_, Dpllctrl1Spec> {
        NW::new(self, 16)
    }
}
#[doc = "DPLL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dpllctrl1Spec;
impl crate::RegisterSpec for Dpllctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpllctrl1::R`](R) reader structure"]
impl crate::Readable for Dpllctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpllctrl1::W`](W) writer structure"]
impl crate::Writable for Dpllctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPLLCTRL1 to value 0"]
impl crate::Resettable for Dpllctrl1Spec {}
