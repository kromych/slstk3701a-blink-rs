#[doc = "Register `DDATA0BYTE` reader"]
pub type R = crate::R<Ddata0byteSpec>;
#[doc = "Register `DDATA0BYTE` writer"]
pub type W = crate::W<Ddata0byteSpec>;
#[doc = "Field `DDATA0BYTE` reader - Ddata 0 Byte Access"]
pub type Ddata0byteR = crate::FieldReader;
#[doc = "Field `DDATA0BYTE` writer - Ddata 0 Byte Access"]
pub type Ddata0byteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Ddata 0 Byte Access"]
    #[inline(always)]
    pub fn ddata0byte(&self) -> Ddata0byteR {
        Ddata0byteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Ddata 0 Byte Access"]
    #[inline(always)]
    pub fn ddata0byte(&mut self) -> Ddata0byteW<'_, Ddata0byteSpec> {
        Ddata0byteW::new(self, 0)
    }
}
#[doc = "DDATA0 Register Byte Access\n\nYou can [`read`](crate::Reg::read) this register and get [`ddata0byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddata0byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct Ddata0byteSpec;
impl crate::RegisterSpec for Ddata0byteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddata0byte::R`](R) reader structure"]
impl crate::Readable for Ddata0byteSpec {}
#[doc = "`write(|w| ..)` method takes [`ddata0byte::W`](W) writer structure"]
impl crate::Writable for Ddata0byteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDATA0BYTE to value 0"]
impl crate::Resettable for Ddata0byteSpec {}
