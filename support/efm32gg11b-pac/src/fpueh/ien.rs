#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `FPIOC` reader - FPIOC Interrupt Enable"]
pub type FpiocR = crate::BitReader;
#[doc = "Field `FPIOC` writer - FPIOC Interrupt Enable"]
pub type FpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDZC` reader - FPDZC Interrupt Enable"]
pub type FpdzcR = crate::BitReader;
#[doc = "Field `FPDZC` writer - FPDZC Interrupt Enable"]
pub type FpdzcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUFC` reader - FPUFC Interrupt Enable"]
pub type FpufcR = crate::BitReader;
#[doc = "Field `FPUFC` writer - FPUFC Interrupt Enable"]
pub type FpufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPOFC` reader - FPOFC Interrupt Enable"]
pub type FpofcR = crate::BitReader;
#[doc = "Field `FPOFC` writer - FPOFC Interrupt Enable"]
pub type FpofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIDC` reader - FPIDC Interrupt Enable"]
pub type FpidcR = crate::BitReader;
#[doc = "Field `FPIDC` writer - FPIDC Interrupt Enable"]
pub type FpidcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIXC` reader - FPIXC Interrupt Enable"]
pub type FpixcR = crate::BitReader;
#[doc = "Field `FPIXC` writer - FPIXC Interrupt Enable"]
pub type FpixcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    pub fn fpioc(&self) -> FpiocR {
        FpiocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FpdzcR {
        FpdzcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpufc(&self) -> FpufcR {
        FpufcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpofc(&self) -> FpofcR {
        FpofcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    pub fn fpidc(&self) -> FpidcR {
        FpidcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    pub fn fpixc(&self) -> FpixcR {
        FpixcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPIOC Interrupt Enable"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FpiocW<'_, IenSpec> {
        FpiocW::new(self, 0)
    }
    #[doc = "Bit 1 - FPDZC Interrupt Enable"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FpdzcW<'_, IenSpec> {
        FpdzcW::new(self, 1)
    }
    #[doc = "Bit 2 - FPUFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FpufcW<'_, IenSpec> {
        FpufcW::new(self, 2)
    }
    #[doc = "Bit 3 - FPOFC Interrupt Enable"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FpofcW<'_, IenSpec> {
        FpofcW::new(self, 3)
    }
    #[doc = "Bit 4 - FPIDC Interrupt Enable"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FpidcW<'_, IenSpec> {
        FpidcW::new(self, 4)
    }
    #[doc = "Bit 5 - FPIXC Interrupt Enable"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FpixcW<'_, IenSpec> {
        FpixcW::new(self, 5)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
