#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `FPIOC` writer - Set FPIOC Interrupt Flag"]
pub type FpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDZC` writer - Set FPDZC Interrupt Flag"]
pub type FpdzcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUFC` writer - Set FPUFC Interrupt Flag"]
pub type FpufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPOFC` writer - Set FPOFC Interrupt Flag"]
pub type FpofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIDC` writer - Set FPIDC Interrupt Flag"]
pub type FpidcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIXC` writer - Set FPIXC Interrupt Flag"]
pub type FpixcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set FPIOC Interrupt Flag"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FpiocW<'_, IfsSpec> {
        FpiocW::new(self, 0)
    }
    #[doc = "Bit 1 - Set FPDZC Interrupt Flag"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FpdzcW<'_, IfsSpec> {
        FpdzcW::new(self, 1)
    }
    #[doc = "Bit 2 - Set FPUFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FpufcW<'_, IfsSpec> {
        FpufcW::new(self, 2)
    }
    #[doc = "Bit 3 - Set FPOFC Interrupt Flag"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FpofcW<'_, IfsSpec> {
        FpofcW::new(self, 3)
    }
    #[doc = "Bit 4 - Set FPIDC Interrupt Flag"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FpidcW<'_, IfsSpec> {
        FpidcW::new(self, 4)
    }
    #[doc = "Bit 5 - Set FPIXC Interrupt Flag"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FpixcW<'_, IfsSpec> {
        FpixcW::new(self, 5)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
