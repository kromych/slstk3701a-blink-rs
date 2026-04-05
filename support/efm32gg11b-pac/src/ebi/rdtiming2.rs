#[doc = "Register `RDTIMING2` reader"]
pub type R = crate::R<Rdtiming2Spec>;
#[doc = "Register `RDTIMING2` writer"]
pub type W = crate::W<Rdtiming2Spec>;
#[doc = "Field `RDSETUP` reader - Read Setup Time"]
pub type RdsetupR = crate::FieldReader;
#[doc = "Field `RDSETUP` writer - Read Setup Time"]
pub type RdsetupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDSTRB` reader - Read Strobe Time"]
pub type RdstrbR = crate::FieldReader;
#[doc = "Field `RDSTRB` writer - Read Strobe Time"]
pub type RdstrbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RDHOLD` reader - Read Hold Time"]
pub type RdholdR = crate::FieldReader;
#[doc = "Field `RDHOLD` writer - Read Hold Time"]
pub type RdholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HALFRE` reader - Half Cycle REn Strobe Duration Enable"]
pub type HalfreR = crate::BitReader;
#[doc = "Field `HALFRE` writer - Half Cycle REn Strobe Duration Enable"]
pub type HalfreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREFETCH` reader - Prefetch Enable"]
pub type PrefetchR = crate::BitReader;
#[doc = "Field `PREFETCH` writer - Prefetch Enable"]
pub type PrefetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGEMODE` reader - Page Mode Access Enable"]
pub type PagemodeR = crate::BitReader;
#[doc = "Field `PAGEMODE` writer - Page Mode Access Enable"]
pub type PagemodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RdsetupR {
        RdsetupR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RdstrbR {
        RdstrbR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&self) -> RdholdR {
        RdholdR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfre(&self) -> HalfreR {
        HalfreR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetch(&self) -> PrefetchR {
        PrefetchR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    pub fn pagemode(&self) -> PagemodeR {
        PagemodeR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&mut self) -> RdsetupW<'_, Rdtiming2Spec> {
        RdsetupW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&mut self) -> RdstrbW<'_, Rdtiming2Spec> {
        RdstrbW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RdholdW<'_, Rdtiming2Spec> {
        RdholdW::new(self, 16)
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfre(&mut self) -> HalfreW<'_, Rdtiming2Spec> {
        HalfreW::new(self, 28)
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetch(&mut self) -> PrefetchW<'_, Rdtiming2Spec> {
        PrefetchW::new(self, 29)
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    pub fn pagemode(&mut self) -> PagemodeW<'_, Rdtiming2Spec> {
        PagemodeW::new(self, 30)
    }
}
#[doc = "Read Timing Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rdtiming2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdtiming2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rdtiming2Spec;
impl crate::RegisterSpec for Rdtiming2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdtiming2::R`](R) reader structure"]
impl crate::Readable for Rdtiming2Spec {}
#[doc = "`write(|w| ..)` method takes [`rdtiming2::W`](W) writer structure"]
impl crate::Writable for Rdtiming2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDTIMING2 to value 0x0007_7f07"]
impl crate::Resettable for Rdtiming2Spec {
    const RESET_VALUE: u32 = 0x0007_7f07;
}
