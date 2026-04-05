#[doc = "Register `PJ_DOUT` reader"]
pub type R = crate::R<PjDoutSpec>;
#[doc = "Register `PJ_DOUT` writer"]
pub type W = crate::W<PjDoutSpec>;
#[doc = "Field `DOUT` reader - Data Out"]
pub type DoutR = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Data Out"]
pub type DoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&mut self) -> DoutW<'_, PjDoutSpec> {
        DoutW::new(self, 0)
    }
}
#[doc = "Port Data Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pj_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pj_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PjDoutSpec;
impl crate::RegisterSpec for PjDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pj_dout::R`](R) reader structure"]
impl crate::Readable for PjDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`pj_dout::W`](W) writer structure"]
impl crate::Writable for PjDoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PJ_DOUT to value 0"]
impl crate::Resettable for PjDoutSpec {}
