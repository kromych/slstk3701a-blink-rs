#[doc = "Register `CH15_LINK` reader"]
pub type R = crate::R<Ch15LinkSpec>;
#[doc = "Register `CH15_LINK` writer"]
pub type W = crate::W<Ch15LinkSpec>;
#[doc = "Field `LINKMODE` reader - Link Structure Addressing Mode"]
pub type LinkmodeR = crate::BitReader;
#[doc = "Field `LINK` reader - Link Next Structure"]
pub type LinkR = crate::BitReader;
#[doc = "Field `LINK` writer - Link Next Structure"]
pub type LinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINKADDR` reader - Link Structure Address"]
pub type LinkaddrR = crate::FieldReader<u32>;
#[doc = "Field `LINKADDR` writer - Link Structure Address"]
pub type LinkaddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Link Structure Addressing Mode"]
    #[inline(always)]
    pub fn linkmode(&self) -> LinkmodeR {
        LinkmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&self) -> LinkaddrR {
        LinkaddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, Ch15LinkSpec> {
        LinkW::new(self, 1)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&mut self) -> LinkaddrW<'_, Ch15LinkSpec> {
        LinkaddrW::new(self, 2)
    }
}
#[doc = "Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch15LinkSpec;
impl crate::RegisterSpec for Ch15LinkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch15_link::R`](R) reader structure"]
impl crate::Readable for Ch15LinkSpec {}
#[doc = "`write(|w| ..)` method takes [`ch15_link::W`](W) writer structure"]
impl crate::Writable for Ch15LinkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH15_LINK to value 0"]
impl crate::Resettable for Ch15LinkSpec {}
