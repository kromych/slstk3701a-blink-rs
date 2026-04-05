#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `FC` reader - Frame Counter Interrupt Enable"]
pub type FcR = crate::BitReader;
#[doc = "Field `FC` writer - Frame Counter Interrupt Enable"]
pub type FcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Frame Counter Interrupt Enable"]
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Counter Interrupt Enable"]
    #[inline(always)]
    pub fn fc(&mut self) -> FcW<'_, IenSpec> {
        FcW::new(self, 0)
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
