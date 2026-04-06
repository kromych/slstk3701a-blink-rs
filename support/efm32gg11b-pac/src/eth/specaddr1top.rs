#[doc = "Register `SPECADDR1TOP` reader"]
pub type R = crate::R<Specaddr1topSpec>;
#[doc = "Register `SPECADDR1TOP` writer"]
pub type W = crate::W<Specaddr1topSpec>;
#[doc = "Field `ADDR` reader - Specific address 1 MSB"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Specific address 1 MSB"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FILTERTYPE` reader - MAC SA or DA selection"]
pub type FiltertypeR = crate::BitReader;
#[doc = "Field `FILTERTYPE` writer - MAC SA or DA selection"]
pub type FiltertypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&self) -> FiltertypeR {
        FiltertypeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific address 1 MSB"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, Specaddr1topSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 16 - MAC SA or DA selection"]
    #[inline(always)]
    pub fn filtertype(&mut self) -> FiltertypeW<'_, Specaddr1topSpec> {
        FiltertypeW::new(self, 16)
    }
}
#[doc = "Specific Address 1 Top\n\nYou can [`read`](crate::Reg::read) this register and get [`specaddr1top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`specaddr1top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Specaddr1topSpec;
impl crate::RegisterSpec for Specaddr1topSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`specaddr1top::R`](R) reader structure"]
impl crate::Readable for Specaddr1topSpec {}
#[doc = "`write(|w| ..)` method takes [`specaddr1top::W`](W) writer structure"]
impl crate::Writable for Specaddr1topSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPECADDR1TOP to value 0"]
impl crate::Resettable for Specaddr1topSpec {}
