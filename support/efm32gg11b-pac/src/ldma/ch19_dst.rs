#[doc = "Register `CH19_DST` reader"]
pub type R = crate::R<Ch19DstSpec>;
#[doc = "Register `CH19_DST` writer"]
pub type W = crate::W<Ch19DstSpec>;
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
    pub fn dstaddr(&mut self) -> DstaddrW<'_, Ch19DstSpec> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch19_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch19_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch19DstSpec;
impl crate::RegisterSpec for Ch19DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch19_dst::R`](R) reader structure"]
impl crate::Readable for Ch19DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch19_dst::W`](W) writer structure"]
impl crate::Writable for Ch19DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH19_DST to value 0"]
impl crate::Resettable for Ch19DstSpec {}
