#[doc = "Register `DDATA1BYTE` reader"]
pub type R = crate::R<Ddata1byteSpec>;
#[doc = "Register `DDATA1BYTE` writer"]
pub type W = crate::W<Ddata1byteSpec>;
#[doc = "Field `DDATA1BYTE` reader - Ddata 1 Byte Access"]
pub type Ddata1byteR = crate::FieldReader;
#[doc = "Field `DDATA1BYTE` writer - Ddata 1 Byte Access"]
pub type Ddata1byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    pub fn ddata1byte(&self) -> Ddata1byteR {
        Ddata1byteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 1 Byte Access"]
    #[inline(always)]
    pub fn ddata1byte(&mut self) -> Ddata1byteW<'_, Ddata1byteSpec> {
        Ddata1byteW::new(self, 0)
    }
}
#[doc = "DDATA1 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata1byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata1byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Ddata1byteSpec;
impl crate::RegisterSpec for Ddata1byteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata1byte::R`](R) reader structure"]
impl crate::Readable for Ddata1byteSpec {}
#[doc = "`write(|w| ..)` method takes [`ddata1byte::W`](W) writer structure"]
impl crate::Writable for Ddata1byteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA1BYTE to value 0"]
impl crate::Resettable for Ddata1byteSpec {}
