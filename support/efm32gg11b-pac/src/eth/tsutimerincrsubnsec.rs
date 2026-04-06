#[doc = "Register `TSUTIMERINCRSUBNSEC` reader"]
pub type R = crate::R<TsutimerincrsubnsecSpec>;
#[doc = "Register `TSUTIMERINCRSUBNSEC` writer"]
pub type W = crate::W<TsutimerincrsubnsecSpec>;
#[doc = "Field `SUBNSINCR` reader - MSB \\[23:8\\] of the subscript-ns value"]
pub type SubnsincrR = crate::FieldReader<u16>;
#[doc = "Field `SUBNSINCR` writer - MSB \\[23:8\\] of the subscript-ns value"]
pub type SubnsincrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SUBNSINCRLSB` reader - LSB \\[7:0\\] of the subscript-ns value"]
pub type SubnsincrlsbR = crate::FieldReader;
#[doc = "Field `SUBNSINCRLSB` writer - LSB \\[7:0\\] of the subscript-ns value"]
pub type SubnsincrlsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - MSB \\[23:8\\] of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincr(&self) -> SubnsincrR {
        SubnsincrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\] of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincrlsb(&self) -> SubnsincrlsbR {
        SubnsincrlsbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - MSB \\[23:8\\] of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincr(&mut self) -> SubnsincrW<'_, TsutimerincrsubnsecSpec> {
        SubnsincrW::new(self, 0)
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\] of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincrlsb(&mut self) -> SubnsincrlsbW<'_, TsutimerincrsubnsecSpec> {
        SubnsincrlsbW::new(self, 24)
    }
}
#[doc = "1588 Timer Increment Register subscript nsec\n\nYou can [`read`](crate::Reg::read) this register and get [`tsutimerincrsubnsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsutimerincrsubnsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsutimerincrsubnsecSpec;
impl crate::RegisterSpec for TsutimerincrsubnsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimerincrsubnsec::R`](R) reader structure"]
impl crate::Readable for TsutimerincrsubnsecSpec {}
#[doc = "`write(|w| ..)` method takes [`tsutimerincrsubnsec::W`](W) writer structure"]
impl crate::Writable for TsutimerincrsubnsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSUTIMERINCRSUBNSEC to value 0"]
impl crate::Resettable for TsutimerincrsubnsecSpec {}
