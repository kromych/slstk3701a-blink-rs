#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `PPUPRIV` reader - PPUPRIV Interrupt Enable"]
pub type PpuprivR = crate::BitReader;
#[doc = "Field `PPUPRIV` writer - PPUPRIV Interrupt Enable"]
pub type PpuprivW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPUPRIV Interrupt Enable"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PpuprivR {
        PpuprivR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPUPRIV Interrupt Enable"]
    #[inline(always)]
    pub fn ppupriv(&mut self) -> PpuprivW<'_, IenSpec> {
        PpuprivW::new(self, 0)
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
