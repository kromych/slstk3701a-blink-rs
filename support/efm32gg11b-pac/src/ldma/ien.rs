#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `DONE` reader - DONE Interrupt Enable"]
pub type DoneR = crate::FieldReader<u32>;
#[doc = "Field `DONE` writer - DONE Interrupt Enable"]
pub type DoneW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ERROR` reader - ERROR Interrupt Enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - ERROR Interrupt Enable"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - DONE Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - DONE Interrupt Enable"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IenSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 31 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IenSpec> {
        ErrorW::new(self, 31)
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
