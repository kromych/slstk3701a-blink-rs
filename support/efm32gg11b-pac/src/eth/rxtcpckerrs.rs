#[doc = "Register `RXTCPCKERRS` reader"]
pub type R = crate::R<RxtcpckerrsSpec>;
#[doc = "Register `RXTCPCKERRS` writer"]
pub type W = crate::W<RxtcpckerrsSpec>;
#[doc = "Field `COUNT` reader - TCP checksum errors"]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - TCP checksum errors"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TCP checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TCP checksum errors"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, RxtcpckerrsSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "TCP Checksum Errors\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtcpckerrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtcpckerrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtcpckerrsSpec;
impl crate::RegisterSpec for RxtcpckerrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtcpckerrs::R`](R) reader structure"]
impl crate::Readable for RxtcpckerrsSpec {}
#[doc = "`write(|w| ..)` method takes [`rxtcpckerrs::W`](W) writer structure"]
impl crate::Writable for RxtcpckerrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTCPCKERRS to value 0"]
impl crate::Resettable for RxtcpckerrsSpec {}
