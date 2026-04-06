#[doc = "Register `WRTIMING1` reader"]
pub type R = crate::R<Wrtiming1Spec>;
#[doc = "Register `WRTIMING1` writer"]
pub type W = crate::W<Wrtiming1Spec>;
#[doc = "Field `WRSETUP` reader - Write Setup Time"]
pub type WrsetupR = crate::FieldReader;
#[doc = "Field `WRSETUP` writer - Write Setup Time"]
pub type WrsetupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRSTRB` reader - Write Strobe Time"]
pub type WrstrbR = crate::FieldReader;
#[doc = "Field `WRSTRB` writer - Write Strobe Time"]
pub type WrstrbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRHOLD` reader - Write Hold Time"]
pub type WrholdR = crate::FieldReader;
#[doc = "Field `WRHOLD` writer - Write Hold Time"]
pub type WrholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HALFWE` reader - Half Cycle WEn Strobe Duration Enable"]
pub type HalfweR = crate::BitReader;
#[doc = "Field `HALFWE` writer - Half Cycle WEn Strobe Duration Enable"]
pub type HalfweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBUFDIS` reader - Write Buffer Disable"]
pub type WbufdisR = crate::BitReader;
#[doc = "Field `WBUFDIS` writer - Write Buffer Disable"]
pub type WbufdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WrsetupR {
        WrsetupR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WrstrbR {
        WrstrbR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&self) -> WrholdR {
        WrholdR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfwe(&self) -> HalfweR {
        HalfweR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    pub fn wbufdis(&self) -> WbufdisR {
        WbufdisR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&mut self) -> WrsetupW<'_, Wrtiming1Spec> {
        WrsetupW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&mut self) -> WrstrbW<'_, Wrtiming1Spec> {
        WrstrbW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&mut self) -> WrholdW<'_, Wrtiming1Spec> {
        WrholdW::new(self, 16)
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfwe(&mut self) -> HalfweW<'_, Wrtiming1Spec> {
        HalfweW::new(self, 28)
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    pub fn wbufdis(&mut self) -> WbufdisW<'_, Wrtiming1Spec> {
        WbufdisW::new(self, 29)
    }
}
#[doc = "Write Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtiming1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrtiming1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrtiming1Spec;
impl crate::RegisterSpec for Wrtiming1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrtiming1::R`](R) reader structure"]
impl crate::Readable for Wrtiming1Spec {}
#[doc = "`write(|w| ..)` method takes [`wrtiming1::W`](W) writer structure"]
impl crate::Writable for Wrtiming1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRTIMING1 to value 0x0007_7f07"]
impl crate::Resettable for Wrtiming1Spec {
    const RESET_VALUE: u32 = 0x0007_7f07;
}
