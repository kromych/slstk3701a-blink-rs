#[doc = "Register `STRETCHRATIO` reader"]
pub type R = crate::R<StretchratioSpec>;
#[doc = "Register `STRETCHRATIO` writer"]
pub type W = crate::W<StretchratioSpec>;
#[doc = "Field `IPGSTRETCH` reader - IPG Stretch"]
pub type IpgstretchR = crate::FieldReader<u16>;
#[doc = "Field `IPGSTRETCH` writer - IPG Stretch"]
pub type IpgstretchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    pub fn ipgstretch(&self) -> IpgstretchR {
        IpgstretchR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    pub fn ipgstretch(&mut self) -> IpgstretchW<'_, StretchratioSpec> {
        IpgstretchW::new(self, 0)
    }
}
#[doc = "IPG stretch register\n\nYou can [`read`](crate::Reg::read) this register and get [`stretchratio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stretchratio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StretchratioSpec;
impl crate::RegisterSpec for StretchratioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stretchratio::R`](R) reader structure"]
impl crate::Readable for StretchratioSpec {}
#[doc = "`write(|w| ..)` method takes [`stretchratio::W`](W) writer structure"]
impl crate::Writable for StretchratioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STRETCHRATIO to value 0"]
impl crate::Resettable for StretchratioSpec {}
