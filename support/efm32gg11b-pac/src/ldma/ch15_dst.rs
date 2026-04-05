#[doc = "Register `CH15_DST` reader"]
pub type R = crate::R<Ch15DstSpec>;
#[doc = "Register `CH15_DST` writer"]
pub type W = crate::W<Ch15DstSpec>;
#[doc = "Field `DSTADDR` reader - Destination Data Address"]
pub type DstaddrR = crate::FieldReader<u32>;
#[doc = "Field `DSTADDR` writer - Destination Data Address"]
pub type DstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DstaddrR {
        DstaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&mut self) -> DstaddrW<'_, Ch15DstSpec> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch15DstSpec;
impl crate::RegisterSpec for Ch15DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch15_dst::R`](R) reader structure"]
impl crate::Readable for Ch15DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch15_dst::W`](W) writer structure"]
impl crate::Writable for Ch15DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH15_DST to value 0"]
impl crate::Resettable for Ch15DstSpec {}
