#[doc = "Register `CH17_SRC` reader"]
pub type R = crate::R<Ch17SrcSpec>;
#[doc = "Register `CH17_SRC` writer"]
pub type W = crate::W<Ch17SrcSpec>;
#[doc = "Field `SRCADDR` reader - Source Data Address"]
pub type SrcaddrR = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - Source Data Address"]
pub type SrcaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SrcaddrR {
        SrcaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&mut self) -> SrcaddrW<'_, Ch17SrcSpec> {
        SrcaddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch17_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch17_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch17SrcSpec;
impl crate::RegisterSpec for Ch17SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch17_src::R`](R) reader structure"]
impl crate::Readable for Ch17SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch17_src::W`](W) writer structure"]
impl crate::Writable for Ch17SrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH17_SRC to value 0"]
impl crate::Resettable for Ch17SrcSpec {}
